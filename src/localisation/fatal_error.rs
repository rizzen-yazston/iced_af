// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{ApplicationError, constants::APPLICATION_NAME_SHORT},
    core::{
        localisation::Localisation,
        traits::{AnyLocalisedTrait, LocalisedTrait},
    },
};
use i18n::utility::{LanguageTag, LocalisationTrait, PlaceholderValue};

#[cfg(feature = "log")]
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
    UncaughtError,
    Exit,
}

#[derive(Debug)]
pub struct Strings {
    language_tag: RefCount<LanguageTag>,
    strings: Vec<String>,
}

impl Strings {
    pub fn new(localisation: &Localisation, error: ApplicationError) -> Self {
        let (language_tag, strings) = localise(localisation, error);
        Strings {
            language_tag,
            strings,
        }
    }
}

impl AnyLocalisedTrait for Strings {
    fn as_any(&self) -> &dyn Any {
        self
    }
}

impl LocalisedTrait for Strings {
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
    error: ApplicationError,
) -> (RefCount<LanguageTag>, Vec<String>) {
    let language_tag = localisation.default_language();
    let title = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
        );
        values.insert(
            "window".to_string(),
            PlaceholderValue::String(
                match localisation.literal_with_defaults("application", "fatal_error") {
                    Ok(value) => value.to_string(),
                    Err(_) => "Fatal error".to_string(),
                },
            ),
        );
        match localisation.format_with_defaults("application", "window_title_format", &values) {
            Ok(value) => value.to_string(),
            Err(_) => format!("{} - Fatal error", APPLICATION_NAME_SHORT),
        }
    };
    let uncaught_error = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "error".to_string(),
            PlaceholderValue::LocalisationData(error.localisation_data()),
        );
        match localisation.format_with_defaults("application", "uncaught_error", &values) {
            Ok(value) => value.to_string(),
            Err(_) => format!("The following error was not caught: '{}'", error),
        }
    };

    // Always print error message to the console.
    println!("{}", uncaught_error.as_str());

    let exit = {
        #[cfg(target_os = "macos")]
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "short_name".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            match localisation.format_with_defaults("application", "quit_macos", &values) {
                Ok(value) => value.to_string(),
                Err(_) => format!("Quit {}", APPLICATION_NAME_SHORT),
            }
        }

        #[cfg(not(target_os = "macos"))]
        match localisation.literal_with_defaults("word", "exit_i") {
            Ok(value) => value.to_string(),
            Err(_) => "Exit".to_string(),
        }
    };
    (language_tag, vec![title, uncaught_error, exit])
}
