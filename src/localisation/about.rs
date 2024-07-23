// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::constants::APPLICATION_NAME_SHORT,
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
    Contributors,
    Ok,
    Localisation,
}

#[derive(Debug)]
pub struct Strings {
    language_tag: RefCount<LanguageTag>,
    strings: Vec<String>,
}

impl Strings {
    pub fn try_new(localisation: &Localisation) -> Result<Self, CoreError> {
        let (language_tag, strings) = localise(localisation)?;
        Ok(Strings {
            language_tag,
            strings,
        })
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
            debug!("Updating About UI localisation.");

            let (language_tag, strings) = localise(localisation)?;
            self.language_tag = language_tag;
            self.strings = strings;
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
            PlaceholderValue::TaggedString(localisation.literal_with_defaults("word", "about_i")?),
        );
        localisation.format_with_defaults("application", "window_title_format", &values)?
    }
    .to_string();
    let contributors = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::TaggedString(
                localisation.literal_with_defaults("word", "contributors_ip")?,
            ),
        );
        localisation.format_with_defaults("application", "add_colon_format", &values)?
    }
    .to_string();
    let ok = { localisation.literal_with_defaults("word", "ok_i")? }.to_string();
    let localisation_contributors = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::TaggedString(
                localisation.literal_with_defaults("application", "localisation_contributors")?,
            ),
        );
        localisation.format_with_defaults("application", "add_colon_format", &values)?
    }
    .to_string();
    Ok((
        language_tag,
        vec![title, contributors, ok, localisation_contributors],
    ))
}
