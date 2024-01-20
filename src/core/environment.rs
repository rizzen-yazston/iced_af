// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    application::StartUp,
    error::ApplicationError,
    clap::Clap,
};
use std::{
    path::PathBuf,
    env,
};

#[cfg( feature = "log" )]
use log4rs::Handle as LoggerHandler;

pub struct Environment {

    #[cfg( feature = "log" )]
    pub logger: LoggerHandler,

    pub clap: Clap,
    pub application_path: PathBuf,
    pub application_name: String,
    pub application_short_name: String,
    pub application_abbreviation: String,
}

impl Environment {
    pub fn try_new( flags: &StartUp ) -> Result<Environment, ApplicationError> {
        let application_path = match env::current_exe()?.parent() {
            None => return Err( ApplicationError::ApplicationPath ),
            Some( value ) => value.to_owned(),
        };
        Ok( Environment {
            #[cfg( feature = "log" )]
            logger: flags.logger.clone(),

            clap: flags.clap.clone(),
            application_path,
            application_name: "Iced Application Framework Example".to_string(),
            application_short_name: "iced_af Example".to_string(),
            application_abbreviation: "iced_af".to_string(),
        } )
    }
}
