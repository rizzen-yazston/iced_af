// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{
        constants::APPLICATION_NAME_SHORT,
        log::LogLevel,
    },
    core::{
        error::CoreError,
        localisation::Localisation,
        traits::{AnyLocalisedTrait, LocalisedTrait},
    },
};
use i18n::utility::{LanguageTag, PlaceholderValue};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::{any::Any, collections::HashMap};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

pub enum Index {
    Title,
    Accept,
    Cancel,
    Language,
    LanguageUi,
    LanguagePlaceholder,
    Logs,
    LogLevelDefault,
    LogLevelApplication,
    LogLevelOther,
    LogLevelIced,
    LogLevelI18n,
    LogPlaceholder,
}

#[derive(Debug)]
pub struct Strings {
    language_tag: RefCount<LanguageTag>,
    strings: Vec<String>,
    language_list: Vec<String>,
    language_map_to_tag: HashMap<String, RefCount<LanguageTag>>,
    language_map_to_string: HashMap<RefCount<LanguageTag>, String>,
    log_list: Vec<String>,
    log_map_to_level: HashMap<String, LogLevel>,
    log_map_to_string: HashMap<LogLevel, String>,
}

impl Strings {
    pub fn try_new(localisation: &Localisation) -> Result<Self, CoreError> {
        let (language_tag, mut strings) = localise(localisation)?;
        let (language_list, language_map_to_tag, language_map_to_string) =
            localise_i18n(localisation, &mut strings)?;
        let (log_list, log_map_to_level, log_map_to_string) =
            localise_log(localisation, &mut strings)?;
        Ok(Strings {
            language_tag,
            strings,
            language_list,
            language_map_to_tag,
            language_map_to_string,
            log_list,
            log_map_to_level,
            log_map_to_string,
        })
    }

    pub fn language_list(&self) -> &Vec<String> {
        &self.language_list
    }

    pub fn language_map_to_tag(&self, string: &String) -> Option<&RefCount<LanguageTag>> {
        self.language_map_to_tag.get(string)
    }

    pub fn language_map_to_string(&self, tag: &RefCount<LanguageTag>) -> Option<&String> {
        self.language_map_to_string.get(tag)
    }

    pub fn log_list(&self) -> &Vec<String> {
        &self.log_list
    }

    pub fn log_map_to_level(&self, string: &String) -> Option<&LogLevel> {
        self.log_map_to_level.get(string)
    }

    pub fn log_map_to_string(&self, level: &LogLevel) -> Option<&String> {
        self.log_map_to_string.get(level)
    }
}

impl AnyLocalisedTrait for Strings {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LocalisedTrait for Strings {
    fn try_update(&mut self, localisation: &Localisation) -> Result<(), CoreError> {
        if self.language_tag != localisation.default_language().into() {
            debug!("Updating Preference UI localisation.");

            // Obtain localised strings
            let (language_tag, mut strings) = localise(localisation)?;
            let (language_list, language_map_to_tag, language_map_to_string) =
                localise_i18n(localisation, &mut strings)?;
            let (log_list, log_map_to_level, log_map_to_string) =
                localise_log(localisation, &mut strings)?;

            // Store localised strings
            self.language_tag = language_tag;
            self.strings = strings;
            self.language_list = language_list;
            self.language_map_to_tag = language_map_to_tag;
            self.language_map_to_string = language_map_to_string;
            self.log_list = log_list;
            self.log_map_to_level = log_map_to_level;
            self.log_map_to_string = log_map_to_string;
        }
        Ok(())
    }

    fn title(&self) -> &String {
        &self.strings[0]
    }

    fn string(&self, index: usize) -> &String {
        &self.strings[index]
    }

    fn language_tag(&self) -> &RefCount<LanguageTag> {
        &self.language_tag
    }
}

fn localise(
    localisation: &Localisation,
) -> Result<(RefCount<LanguageTag>, Vec<String>), CoreError> {
    let language_tag = localisation.default_language();
    let title = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
        );
        values.insert(
            "window".to_string(),
            PlaceholderValue::TaggedString(
                localisation.literal_with_defaults("word", "preferences_i")?,
            ),
        );
        localisation.format_with_defaults("application", "window_title_format", &values)?
    }
    .to_string();
    let accept = localisation
        .literal_with_defaults("word", "accept_i")?
        .to_string();
    let cancel = localisation
        .literal_with_defaults("word", "cancel_i")?
        .to_string();
    Ok((language_tag, vec![title, accept, cancel]))
}

fn localise_i18n(
    localisation: &Localisation,
    strings: &mut Vec<String>,
) -> Result<
    (
        Vec<String>,
        HashMap<String, RefCount<LanguageTag>>,
        HashMap<RefCount<LanguageTag>, String>,
    ),
    CoreError,
> {
    let mut map_to_tag = HashMap::<String, RefCount<LanguageTag>>::new();
    let mut map_to_string = HashMap::<RefCount<LanguageTag>, String>::new();
    let mut list = Vec::<String>::new();
    for (tag, (_layout, ratio)) in localisation.available_languages().iter() {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        let language_string = tag.as_str().to_string();
        values.insert(
            "language".to_string(),
            PlaceholderValue::String(language_string.clone()),
        );
        values.insert(
            "percent".to_string(),
            PlaceholderValue::Unsigned((ratio * 100f32) as u128),
        );
        let text = localisation
            .format_with_defaults("application", "language_percent_format", &values)?
            .to_string();
        list.push(text.clone());
        map_to_tag.insert(text.clone(), RefCount::clone(tag));
        map_to_string.insert(RefCount::clone(tag), text);
    }
    strings.push(
        localisation
            .literal_with_defaults("word", "language_i")?
            .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "ui_language")?
            .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "placeholder_language")?
            .to_string(),
    );
    Ok((list, map_to_tag, map_to_string))
}

fn localise_log(
    localisation: &Localisation,
    strings: &mut Vec<String>,
) -> Result<
    (
        Vec<String>,
        HashMap<String, LogLevel>,
        HashMap<LogLevel, String>,
    ),
    CoreError,
> {
    let mut map_to_level = HashMap::<String, LogLevel>::new();
    let mut map_to_string = HashMap::<LogLevel, String>::new();
    let mut list = Vec::<String>::new();
    strings.push(
        localisation
            .literal_with_defaults("word", "logs_i")?
            .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "log_level_default")?
            .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "log_level_application")?
            .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "log_level_other")?
            .to_string(),
    );
    strings.push(
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "component".to_string(),
                PlaceholderValue::String("iced".to_string()),
            );
            localisation.format_with_defaults("application", "log_level_component", &values)?
        }
        .to_string(),
    );
    strings.push(
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "component".to_string(),
                PlaceholderValue::String("i18n".to_string()),
            );
            localisation.format_with_defaults("application", "log_level_component", &values)?
        }
        .to_string(),
    );
    strings.push(
        localisation
            .literal_with_defaults("application", "placeholder_log_level")?
            .to_string(),
    );
    let default = localisation
        .literal_with_defaults("word", "default_i")?
        .to_string();
    list.push(default.clone());
    map_to_level.insert(default.clone(), LogLevel::Default);
    map_to_string.insert(LogLevel::Default, default);
    let off = localisation
        .literal_with_defaults("word", "off_i")?
        .to_string();
    list.push(off.clone());
    map_to_level.insert(off.clone(), LogLevel::Off);
    map_to_string.insert(LogLevel::Off, off);
    let error = localisation
        .literal_with_defaults("word", "error_i")?
        .to_string();
    list.push(error.clone());
    map_to_level.insert(error.clone(), LogLevel::Error);
    map_to_string.insert(LogLevel::Error, error);
    let warning = localisation
        .literal_with_defaults("word", "warning_i")?
        .to_string();
    list.push(warning.clone());
    map_to_level.insert(warning.clone(), LogLevel::Warn);
    map_to_string.insert(LogLevel::Warn, warning);
    let information = localisation
        .literal_with_defaults("word", "information_i")?
        .to_string();
    list.push(information.clone());
    map_to_level.insert(information.clone(), LogLevel::Info);
    map_to_string.insert(LogLevel::Info, information);
    let debug = localisation
        .literal_with_defaults("word", "debug_i")?
        .to_string();
    list.push(debug.clone());
    map_to_level.insert(debug.clone(), LogLevel::Debug);
    map_to_string.insert(LogLevel::Debug, debug);
    let trace = localisation
        .literal_with_defaults("word", "trace_i")?
        .to_string();
    list.push(trace.clone());
    map_to_level.insert(trace.clone(), LogLevel::Trace);
    map_to_string.insert(LogLevel::Trace, trace);
    Ok((list, map_to_level, map_to_string))
}
