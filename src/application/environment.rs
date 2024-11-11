// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{ApplicationError, clap::Clap},
    core::error::CoreError,
};
use std::{env, path::PathBuf};
use log4rs::Handle as LoggerHandler;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// The non-persistent environment.
/// 
/// Add more environment components.
pub struct Environment {
    pub application_path: PathBuf,
    pub logger: LoggerHandler,
    pub clap: Clap,
}

impl Environment {
    /// Creates the environment struct.
    pub fn try_new(logger: LoggerHandler, clap: Clap) -> Result<Environment, ApplicationError> {
        let application_path = match env::current_exe() {
            Err(error) => return Err(ApplicationError::Core(CoreError::Io(error.to_string()))),
            Ok(value) => match value.parent() {
                None => return Err(ApplicationError::Core(CoreError::ApplicationPath)),
                Some(value) => value.to_owned(),
            }
        };
        Ok(Environment {
            application_path,
            logger,
            clap,
        })
    }
}
