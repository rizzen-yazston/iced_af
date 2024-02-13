// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    application::StartUp,
    error::ApplicationError,
};
use std::{
    path::PathBuf,
    env,
};

#[cfg( feature = "clap" )]
use super::clap::Clap;

#[cfg( feature = "log" )]
use log4rs::Handle as LoggerHandler;

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

pub struct Environment {
    pub application_path: PathBuf,
    pub application_name: String,
    pub application_short_name: String,
    pub application_abbreviation: String,
    #[cfg( feature = "log" )] pub logger: LoggerHandler,
    #[cfg( feature = "clap" )] pub clap: Clap,
}

impl Environment {
    pub fn try_new( _flags: &StartUp ) -> Result<Environment, ApplicationError> {
        let application_path = match env::current_exe()?.parent() {
            None => return Err( ApplicationError::ApplicationPath ),
            Some( value ) => value.to_owned(),
        };
        Ok( Environment {
            application_path,
            application_name: "Iced Application Framework Example".to_string(),
            application_short_name: "iced_af Example".to_string(),
            application_abbreviation: "iced_af".to_string(),
            #[cfg( feature = "log" )] logger: _flags.logger.clone(),
            #[cfg( feature = "clap" )] clap: _flags.clap.clone(),
        } )
    }
}
