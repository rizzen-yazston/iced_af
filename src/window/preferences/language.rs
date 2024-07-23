// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

#![allow(clippy::single_match)]

use crate::{
    application::{self, ApplicationError, session::Settings, StringGroup},
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
        traits::{AnyLocalisedTrait, TabTrait},
    },
    localisation::preferences::{Index, Strings},
    window::preferences::{self, Setting,},
};

#[allow(unused_imports)]
use iced::{
    widget::{button, column, combo_box, row, scrollable, text, Column, Container, Row},
    window, Alignment, Task, Element, Length, Point, Size,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(feature = "sync")]
use std::sync::Arc as RefCount;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    LanguageSelected(String),
}

pub struct Tab {
    list: combo_box::State<String>,
    original: Option<String>,
    selected: Option<String>,
    changed: bool, // Indicates if UI to be updated to original language.
    update: bool,  // Indicates the UI needs to be updated for selected language.
}

impl Tab {
    pub fn try_new(
        localisation: &Localisation,
        strings: &Box<dyn AnyLocalisedTrait>,
        settings: &Settings,
    ) -> Result<Self, ApplicationError> {
        let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
        let original = actual
            .language_map_to_string(
                match &localisation
                    .language_tag_registry()
                    .tag(settings.ui.language.as_str())
                {
                    Err(error) => {
                        return Err(ApplicationError::Core(CoreError::LanguageTagRegistry(
                            *error,
                        )))
                    }
                    Ok(value) => value,
                },
            )
            .map(|x| x.to_owned());
        let selected = original.clone();
        Ok(Tab {
            list: combo_box::State::new(actual.language_list().to_vec()),
            original,
            selected,
            changed: false,
            update: false,
        })
    }

    pub fn update(&self) -> bool {
        self.update
    }

    pub fn changed(&self) -> bool {
        self.changed
    }

    pub fn update_changed(&mut self) {
        if self.original != self.selected {
            self.changed = true;
        }
    }

    pub fn selected(&mut self, message: Message) {
        match message {
            Message::LanguageSelected(language) => {
                if self.selected != Some(language.clone()) {
                    self.selected = Some(language);
                    self.update = true;
                } else {
                    self.update = false;
                }
            }
        }
    }

    pub fn check_change(
        &self,
        strings: &Box<dyn AnyLocalisedTrait>,
        changed_settings: &mut Vec<Setting>,
    ) {
        let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
        if self.original != self.selected {
            let language_selected_tag = actual
                .language_map_to_tag(&self.selected.as_ref().unwrap())
                .unwrap();
            info!("Current language: {:?}", language_selected_tag);

            changed_settings.push(Setting::Language(RefCount::clone(language_selected_tag)));
        }
    }
}

impl TabTrait for Tab {
    fn title<'a>(&self, string_cache: &'a StringCache) -> String {
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        String::from(strings.string(Index::Language as usize))
    }

    fn content<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, application::Message> {
        let reverse_words = localisation.layout_data().reverse_words;
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        let mut setting: Vec<Element<application::Message>> = vec![
            text(strings.string(Index::LanguageUi as usize)).into(),
            text("").width(Length::Fill).into(),
            combo_box(
                &self.list,
                strings.string(Index::LanguagePlaceholder as usize),
                self.selected.as_ref(),
                move |string| {
                    application::Message::Preferences(
                        id,
                        preferences::Message::Language(Message::LanguageSelected(string)),
                    )
                },
            )
            .width(100)
            .into(),
        ];
        if reverse_words {
            setting.reverse();
        }
        row(setting).width(Length::Fill).height(Length::Fill).into()
    }
}
