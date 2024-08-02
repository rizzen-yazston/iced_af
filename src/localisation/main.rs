// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    core::{
        error::CoreError,
        localisation::Localisation,
        traits::{AnyLocalisedTrait, LocalisedTrait},
    },
    application::constants::APPLICATION_NAME_SHORT,
};
use i18n::utility::{LanguageTag, PlaceholderValue};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

use std::{
    any::Any,
    collections::HashMap,
};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

pub enum Index {
    Title,
    Close,
}

#[derive(Debug)]
pub struct Strings {
    language_tag: RefCount<LanguageTag>,
    strings: Vec<RefCount<String>>,
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
            debug!("Updating Main UI localisation.");

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
) -> Result<(RefCount<LanguageTag>, Vec<RefCount<String>>), CoreError> {
    let language_tag = localisation.default_language();

    // File menu
    let title = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
        );
        let localised = localisation.literal_with_defaults("word", "main_i")?;
        values.insert(
            "window".to_string(),
            PlaceholderValue::Localised(localised.0, localised.1),
        );
        localisation.format_with_defaults("application", "window_title_format", &values)?
    }.0;
    let close = localisation
        .literal_with_defaults("word", "close_i")?.0;

    Ok((
        language_tag,
        vec![title, close],
    ))
}
