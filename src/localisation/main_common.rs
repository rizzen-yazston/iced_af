// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    core::{
        error::CoreError,
        localisation::Localisation,
        traits::{AnyLocalisedTrait, LocalisedTrait},
    },
    //application::APPLICATION_NAME_SHORT,
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
    File,
    Open,
    Edit,
    Preferences,
    Help,
    About,
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
) -> Result<(RefCount<LanguageTag>, Vec<String>), CoreError> {
    let language_tag = localisation.default_language();

    // File menu
    let title = String::new(); // Not used.
    let file_ = localisation
        .literal_with_defaults("word", "file_i")?
        .to_string();
    let open = localisation
        .literal_with_defaults("word", "open_i")?
        .to_string();

    // Edit menu
    let edit = localisation
        .literal_with_defaults("word", "edit_i")?
        .to_string();
    let preferences = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::TaggedString(
                localisation.literal_with_defaults("word", "preferences_i")?,
            ),
        );
        localisation.format_with_defaults("application", "add_elipsis_format", &values)?
    }
    .to_string();

    // Help menu
    let help = localisation
        .literal_with_defaults("word", "help_i")?
        .to_string();
    let about = localisation
        .literal_with_defaults("word", "about_i")?
        .to_string();

    Ok((
        language_tag,
        vec![title, file_, open, edit, preferences, help, about],
    ))
}
