// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The localisation component of the mini application framework.
//!
//! Add new window localisation under `src/windows/` directory.

use crate::{
    application::{environment::Environment, StringGroup},
    core::{error::CoreError, traits::AnyLocalisedTrait},
};
use i18n::{
    lexer::{DataProvider, IcuDataProvider},
    localiser::{CommandRegistry, Localiser},
    provider::RepositoryDetails,
    provider_sqlite3::LocalisationProviderSqlite3,
    utility::{
        Direction, LanguageTag, LanguageTagRegistry, LocalisationData, LocalisationErrorTrait,
        PlaceholderValue, ScriptDirection,
    },
};
use iced::Alignment;
use std::collections::HashMap;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

//
// ----- The localisation for the UI
//

pub struct StringCache {
    // Contains the shared localised strings of the window types
    // Instances specific localisation, such as messages, are stored instead in the window's state
    cache: HashMap<StringGroup, Box<dyn AnyLocalisedTrait>>,
}

impl StringCache {
    pub fn try_new() -> Result<StringCache, CoreError> {
        Ok(StringCache {
            cache: HashMap::<StringGroup, Box<dyn AnyLocalisedTrait>>::new(),
        })
    }

    pub fn try_update(&mut self, localisation: &Localisation) -> Result<(), CoreError> {
        for (string_group, strings) in self.cache.iter_mut() {
            strings.try_update(localisation)?;
            trace!(
                "Updated strings for string group ‘{:?}’: {:?}",
                string_group,
                strings
            );
        }
        Ok(())
    }

    pub fn exists(&self, string_group: &StringGroup) -> bool {
        self.cache.contains_key(string_group)
    }

    pub fn insert(
        &mut self,
        string_group: StringGroup,
        localised_strings: Box<dyn AnyLocalisedTrait>,
    ) {
        let _ = self.cache.insert(string_group, localised_strings);
    }

    pub fn get(&self, string_group: &StringGroup) -> Option<&Box<dyn AnyLocalisedTrait>> {
        self.cache.get(string_group)
    }
}

pub struct Localisation {
    // The i18n localiser
    localiser: Localiser,

    // Layout data for the default language. Cached copy from available_languages as there are many view() calls.
    layout_data: LayoutData,

    // Available languages according to supported scripts
    available_languages: HashMap<RefCount<LanguageTag>, (LayoutData, f32)>,
}

impl Localisation {
    pub fn try_new<T: AsRef<str>>(
        environment: &Environment,
        language: T,
    ) -> Result<Localisation, CoreError> {
        let directions = vec![
            ScriptDirection::TopToBottomLeftToRight,
            ScriptDirection::TopToBottomRightToLeft,
        ];
        let mut available_languages = HashMap::<RefCount<LanguageTag>, (LayoutData, f32)>::new();
        let language_tag_registry = RefCount::new(LanguageTagRegistry::new());
        let path = environment.application_path.join("l10n");
        let localisation_provider = Box::new(
            LocalisationProviderSqlite3::try_new(
                path, &language_tag_registry, false
            )?
        );
        let icu_data_provider = RefCount::new(IcuDataProvider::try_new(DataProvider::Internal)?);
        let command_registry = RefCount::new(CommandRegistry::new());
        let localiser = Localiser::try_new(
            &icu_data_provider,
            &language_tag_registry,
            localisation_provider,
            &command_registry,
            true,
            true,
            language.as_ref(),
        )?;
        let binding = localiser
            .localisation_provider()
            .component_details("application")?;
        debug!("Building language list");
        for language_data in binding.languages.iter() {
            match localiser.script_data_for_language_tag(language_data.0) {
                None => {
                    debug!("Language tag ‘{:?}’ is not supported for the application's user interface.", language_data.0);
                }
                Some(script_data) => {
                    for script_direction in script_data.directions {
                        for supported in directions.iter() {
                            if script_direction == *supported {
                                debug!("Adding language: ‘{:?}’", language_data.0);

                                available_languages.insert(
                                    language_data.0.clone(),
                                    (LayoutData::new(&script_direction), language_data.1.ratio),
                                );
                            }
                        }
                    }
                }
            }
        }
        let layout_data = available_languages
            .get(&localiser.default_language())
            .unwrap()
            .0
            .clone();
        Ok(Localisation {
            localiser,
            layout_data,
            available_languages,
        })
    }

    // ----- Exposed Localiser methods

    pub fn language_tag_registry(&self) -> &RefCount<LanguageTagRegistry> {
        self.localiser.language_tag_registry()
    }

    pub fn icu_data_provider(&self) -> &RefCount<IcuDataProvider> {
        self.localiser.icu_data_provider()
    }

    pub fn command_registry(&self) -> &RefCount<CommandRegistry> {
        self.localiser.command_registry()
    }

    pub fn default_language(&self) -> RefCount<LanguageTag> {
        self.localiser.default_language()
    }

    pub fn repository_details(&self) -> Result<RefCount<RepositoryDetails>, CoreError> {
        Ok(self
            .localiser
            .localisation_provider()
            .repository_details()?)
    }

    pub fn literal_with_defaults(
        &self,
        component: &str,
        identifier: &str,
    ) -> Result<(RefCount<String>, RefCount<LanguageTag>), CoreError> {
        Ok(self
            .localiser
            .literal_with_defaults(component, identifier)?)
    }

    pub fn format_with_defaults(
        &self,
        component: &str,
        identifier: &str,
        values: &HashMap<String, PlaceholderValue>,
    ) -> Result<(RefCount<String>, RefCount<LanguageTag>), CoreError> {
        Ok(self
            .localiser
            .format_with_defaults(component, identifier, values)?)
    }

    pub fn format_error_with_defaults(
        &self,
        error: &impl LocalisationErrorTrait,
    ) -> Result<(RefCount<String>, RefCount<LanguageTag>), CoreError> {
        Ok(self.localiser.format_error_with_defaults(error)?)
    }

    pub fn format_localisation_data_with_defaults(
        &self,
        data: &LocalisationData,
    ) -> Result<(RefCount<String>, RefCount<LanguageTag>), CoreError> {
        Ok(self
            .localiser
            .format_localisation_data_with_defaults(data)?)
    }

    // ----- Localisation methods

    pub fn available_languages(&self) -> &HashMap<RefCount<LanguageTag>, (LayoutData, f32)> {
        &self.available_languages
    }

    pub fn change_default_language(
        &mut self,
        tag: RefCount<LanguageTag>,
    ) -> Result<bool, CoreError> {
        if tag != self.localiser.default_language() {
            let Some(layout) = self.available_languages.get(&tag) else {
                return Err(CoreError::LanguageTagNotSupported(tag.as_str().to_string()));
            };
            self.localiser.defaults(Some(tag), None, None)?;
            self.layout_data = layout.0.clone();
            return Ok(true);
        }
        Ok(false)
    }

    pub fn layout_data(&self) -> &LayoutData {
        &self.layout_data
    }
}

//
// ----- Script directionality
//

/// Text flow data of scripts.
///
/// Field meaning:
///
/// * `flow_line`: The direction of the line stack goes in.
///
/// * `flow_word`: The direction of the words within the line.
///
/// * `reverse_lines`: Normally used to indicate the page elements of a [`Vec`] needs to be reversed before placement.
///
/// * `reverse_words`: Normally used to indicate the line elements of a [`Vec`] needs to be reversed before placement.
///
/// * `align_lines_start`: Align the stack of lines to the start direction. Horizontal taken as top, and vertical
/// taken as left.
///
/// * `align_lines_end`: Align the stack of lines to the end direction.
///
/// * `align_words_start`: Align the words of the lines to the start direction. Horizontal taken as left, and vertical
/// taken as top.
///
/// * `align_words_end`: Align the words of the lines to the end direction.
///
/// `iced` horizontal layout flow is top to bottom for lines/rows and left to right for words/columns. Currently `iced`
/// has not vertical layout support.
#[allow(dead_code)]
#[derive(Debug, Clone)]
pub struct LayoutData {
    pub flow_line: Direction,
    pub flow_word: Direction,
    pub reverse_lines: bool,
    pub reverse_words: bool,
    pub align_lines_start: Alignment,
    pub align_lines_end: Alignment,
    pub align_words_start: Alignment,
    pub align_words_end: Alignment,
}

impl LayoutData {
    pub fn new(script_direction: &ScriptDirection) -> Self {
        match script_direction {
            // Most common is top to bottom line flow.
            ScriptDirection::TopToBottomLeftToRight => LayoutData {
                flow_line: Direction::TopToBottom,
                flow_word: Direction::LeftToRight,
                reverse_lines: false,
                reverse_words: false,
                align_lines_start: Alignment::Start,
                align_lines_end: Alignment::End,
                align_words_start: Alignment::Start,
                align_words_end: Alignment::End,
            },
            ScriptDirection::TopToBottomRightToLeft => LayoutData {
                flow_line: Direction::TopToBottom,
                flow_word: Direction::RightToLeft,
                reverse_lines: false,
                reverse_words: true,
                align_lines_start: Alignment::Start,
                align_lines_end: Alignment::End,
                align_words_start: Alignment::End,
                align_words_end: Alignment::Start,
            },

            // Commonly known as vertical texts, for various eastern asian scripts.
            ScriptDirection::RightToLeftTopToBottom => LayoutData {
                flow_line: Direction::RightToLeft,
                flow_word: Direction::TopToBottom,
                reverse_lines: true,
                reverse_words: false,
                align_lines_start: Alignment::End,
                align_lines_end: Alignment::Start,
                align_words_start: Alignment::Start,
                align_words_end: Alignment::End,
            },
            ScriptDirection::RightToLeftBottomToTop => LayoutData {
                flow_line: Direction::LeftToRight,
                flow_word: Direction::TopToBottom,
                reverse_lines: false,
                reverse_words: false,
                align_lines_start: Alignment::Start,
                align_lines_end: Alignment::End,
                align_words_start: Alignment::Start,
                align_words_end: Alignment::End,
            },

            // The bottom to top line flow is very rare, though some have been seen on monuments.
            // Mongolian script is such a script
            ScriptDirection::LeftToRightTopToBottom => LayoutData {
                flow_line: Direction::BottomToTop,
                flow_word: Direction::LeftToRight,
                reverse_lines: true,
                reverse_words: false,
                align_lines_start: Alignment::End,
                align_lines_end: Alignment::Start,
                align_words_start: Alignment::Start,
                align_words_end: Alignment::End,
            },
            ScriptDirection::LeftToRightBottomToTop => LayoutData {
                flow_line: Direction::BottomToTop,
                flow_word: Direction::RightToLeft,
                reverse_lines: true,
                reverse_words: true,
                align_lines_start: Alignment::End,
                align_lines_end: Alignment::Start,
                align_words_start: Alignment::End,
                align_words_end: Alignment::Start,
            },
            ScriptDirection::BottomToTopLeftToRight => LayoutData {
                flow_line: Direction::RightToLeft,
                flow_word: Direction::BottomToTop,
                reverse_lines: true,
                reverse_words: true,
                align_lines_start: Alignment::End,
                align_lines_end: Alignment::Start,
                align_words_start: Alignment::End,
                align_words_end: Alignment::Start,
            },
            ScriptDirection::BottomToTopRightToLeft => LayoutData {
                flow_line: Direction::LeftToRight,
                flow_word: Direction::BottomToTop,
                reverse_lines: false,
                reverse_words: true,
                align_lines_start: Alignment::Start,
                align_lines_end: Alignment::End,
                align_words_start: Alignment::End,
                align_words_end: Alignment::Start,
            },
        }
    }
}
