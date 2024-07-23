// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::clap::Clap,
    core::error::CoreError,
};
use std::{env, path::PathBuf};
use log4rs::Handle as LoggerHandler;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct Environment {
    pub application_path: PathBuf,
    pub logger: LoggerHandler,
    pub clap: Clap,
}

impl Environment {
    pub fn try_new(logger: LoggerHandler, clap: Clap) -> Result<Environment, CoreError> {
        let application_path = match env::current_exe()?.parent() {
            None => return Err(CoreError::ApplicationPath),
            Some(value) => value.to_owned(),
        };
        Ok(Environment {
            application_path,
            logger,
            clap,
        })
    }
}
