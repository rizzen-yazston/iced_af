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
    New,
    Open,
    Edit,
    Preferences,
    Help,
    About,
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

    let title = RefCount::new(String::new()); // Not used.

    // File menu
    let file_ = localisation
        .literal_with_defaults("word", "file_i")?.0;
    let new = localisation
        .literal_with_defaults("word", "new_i")?.0;
    let open = localisation
        .literal_with_defaults("word", "open_i")?.0;

    // Edit menu
    let edit = localisation
        .literal_with_defaults("word", "edit_i")?.0;
    let preferences = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        let localised = localisation.literal_with_defaults("word", "preferences_i")?;
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::Localised(localised.0, localised.1),
        );
        localisation.format_with_defaults("application", "add_elipsis_format", &values)?
    }.0;

    // Help menu
    let help = localisation
        .literal_with_defaults("word", "help_i")?.0;
    let about = localisation
        .literal_with_defaults("word", "about_i")?.0;

    Ok((
        language_tag,
        vec![title, file_, new, open, edit, preferences, help, about],
    ))
}
