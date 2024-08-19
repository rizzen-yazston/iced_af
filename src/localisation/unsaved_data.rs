// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::core::{
    error::CoreError,
    localisation::Localisation,
    traits::{AnyLocalisedTrait, LocalisedTrait},
};
use i18n::utility::LanguageTag;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use std::any::Any;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

pub enum Index {
    Title,
    //UnsavedData,
    Save,
    Discard,
    Cancel,
}

#[derive(Debug)]
pub struct Strings {
    language_tag: RefCount<LanguageTag>,
    strings: Vec<RefCount<String>>,
}

impl Strings {
    pub fn try_new(
        localisation: &Localisation,
    ) -> Result<Self, CoreError> {
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
            debug!("Updating UnsavedData UI localisation.");

            let (language_tag, strings) = localise(
                localisation,
            )?;
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
    let title = RefCount::new(String::new()); // Not used, State has the dynamic string
    /*
    let unsaved_data = localisation
        .literal_with_defaults("application", "unsaved_data_statement")?.0;
    */
    let save = localisation
        .literal_with_defaults("word", "save_i")?.0;//"application", "save_and_close"
    let discard = localisation
        .literal_with_defaults("word", "discard_i")?.0;//"application", "discard_and_close"
    let cancel = localisation
        .literal_with_defaults("word", "cancel_i")?.0;
    Ok((
        language_tag,
        vec![title, /*unsaved_data, */save, discard, cancel],
    ))
}
