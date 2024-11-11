// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The application errors in addition to the core errors found in crate::core::error::CoreError.

use crate::core::error::CoreError;
use core::fmt::{Display, Formatter, Result};
use i18n::utility::{
    LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue,
};
use rusqlite::Error as Sqlite3Error;
use std::collections::HashMap;
use std::{error::Error, fmt::Debug, io::Error as IoError};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ApplicationError {
    Core(CoreError),
    DatabaseAlreadyOpen,
    InvalidSchema(String),

    // Copied from CoreError as these are common error in application.
    Sqlite3(RefCount<Sqlite3Error>),
    Io(String), // Can't clone io::Error, as it is an OS error, thus converted to final String (can't be translated).
}

impl LocalisationErrorTrait for ApplicationError {}

impl LocalisationTrait for ApplicationError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("ApplicationError".to_string());
        match self {
            ApplicationError::Core(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Core".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::LocalisationData(error.localisation_data()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            ApplicationError::DatabaseAlreadyOpen => {
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "database_already_opened".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("DatabaseAlreadyOpen".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            ApplicationError::InvalidSchema(ref name) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "name".to_string(),
                    PlaceholderValue::String(name.to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "schema_invalid".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidSchema".to_string()),
                );
                values.insert(
                    "message".to_string(),
                    PlaceholderValue::LocalisationData(message),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum".to_string(),
                    values: Some(values),
                }
            }
            ApplicationError::Sqlite3(ref error) => {
                // Currently no localisation is available for this error type: Sqlite3Error.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Sqlite3".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
            ApplicationError::Io(ref error) => {
                // Currently no localisation is available for this error type: IoError (always a String).
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Io".to_string()),
                );
                values.insert(
                    "error".to_string(),
                    PlaceholderValue::String(error.to_string()),
                );
                LocalisationData {
                    component: "i18n_localiser".to_string(),
                    identifier: "error_format_enum_embedded".to_string(),
                    values: Some(values),
                }
            }
        }
    }
}

impl Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            ApplicationError::Core(ref error) => Display::fmt(&error, formatter),
            ApplicationError::DatabaseAlreadyOpen => {
                write!(formatter, "The database is already opened.")
            }
            ApplicationError::InvalidSchema(ref name) => write!(
                formatter,
                "The Sqlite3 file schema is invalid for the database ‘{}’.",
                name
            ),
            ApplicationError::Sqlite3(ref error) => Display::fmt(&error, formatter),
            ApplicationError::Io(ref error) => Display::fmt(&error, formatter),
        }
    }
}

// Source is embedded in the enum value.
impl Error for ApplicationError {}

impl From<CoreError> for ApplicationError {
    fn from(error: CoreError) -> ApplicationError {
        ApplicationError::Core(error)
    }
}

impl From<IoError> for ApplicationError {
    fn from(error: IoError) -> ApplicationError {
        ApplicationError::Io(format!("{}", error.to_string()))
    }
}

impl From<Sqlite3Error> for ApplicationError {
    fn from(error: Sqlite3Error) -> ApplicationError {
        ApplicationError::Sqlite3(RefCount::new(error))
    }
}

