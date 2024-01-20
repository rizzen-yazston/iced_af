// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    environment::Environment,
    error::ApplicationError,
};
use i18n::{
    icu::{ IcuDataProvider, DataProvider },
    localiser::Localiser,
    pattern::CommandRegistry,
    utility::LanguageTagRegistry,
    provider_sqlite3::LocalisationProviderSqlite3,
};
use icu_locid_transform::LocaleDirectionality;

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

// Initialise the i18n message system for the UI, using ICU4X internal data.
pub struct Localisation {
    localiser: Localiser<LocalisationProviderSqlite3>,
    directionality: LocaleDirectionality,
}

impl Localisation {
    pub fn try_new<T: AsRef<str>>(
        environment: &Environment,
        language: T,
    ) -> Result<Localisation, ApplicationError> {
        let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
        let path = environment.application_path.join( "l10n" );
        let localisation_provider = LocalisationProviderSqlite3::try_new(
            path, &language_tag_registry, false
        )?;
        let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
        let command_registry = RefCount::new( CommandRegistry::new() );
        let localiser = Localiser::try_new(
            &icu_data_provider,
            &language_tag_registry,
            localisation_provider,
            &command_registry,
            true,
            true,
            language,
        )?;
        let directionality = LocaleDirectionality::new();
        Ok( Localisation {                
            localiser,
            directionality,
        } )
    }

    pub fn localiser( &self ) -> &Localiser<LocalisationProviderSqlite3> {
        &self.localiser
    }

    pub fn directionality( &self ) -> &LocaleDirectionality {
        &self.directionality
    }
}
