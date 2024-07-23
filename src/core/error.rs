// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! All the core errors of the mini application framework.
//!
//! Add new errors to `ApplicationError` in the `src/application/error.rs` file.

use crate::application::WindowType;
use core::fmt::{Display, Formatter, Result};
use i18n::{
    lexer::IcuError,
    localiser::LocaliserError,
    provider::ProviderError,
    provider_sqlite3::ProviderSqlite3Error,
    utility::{
        LocalisationData, LocalisationErrorTrait, LocalisationTrait, PlaceholderValue,
        RegistryError,
    },
};
use iced::window;
use rusqlite::Error as Sqlite3Error;
use std::{error::Error, fmt::Debug, io::Error as IoError, path::PathBuf};
use ron::error::{Error as RonError, SpannedError};
use std::collections::HashMap;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum CoreError {
    RonSpannedError(SpannedError),
    RonError(RonError),
    LanguageTagRegistry(RegistryError),
    Localiser(LocaliserError),
    Provider(ProviderError),
    ProviderSqlite3(ProviderSqlite3Error),
    Icu(IcuError),
    Sqlite3(RefCount<Sqlite3Error>),
    Io(String), // Can't clone io::Error, as it is an OS error, thus converted to final String (can't be translated).
    ApplicationPath,
    ConfigDirNotFound,
    NoVendorDir(PathBuf),
    NoConfigFile(PathBuf),
    WindowIdNotFound(window::Id, String),
    WindowTypeNotFound(WindowType, String),
    ExpectedWindowParent(WindowType),
    LanguageTagNotSupported(String),
    InvalidWindowTypeMain(WindowType),
    StateNotReusable(WindowType),
    PlaceholderNotFound(WindowType),
}

impl LocalisationErrorTrait for CoreError {}

impl LocalisationTrait for CoreError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("CoreError".to_string());
        match self {
            CoreError::RonSpannedError(ref error) => {
                // Currently no localisation is available for this error type: SpannedError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("RonSpannedError".to_string()),
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
            CoreError::RonError(ref error) => {
                // Currently no localisation is available for this error type: RonError.
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("RonError".to_string()),
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

            CoreError::Localiser(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Localiser".to_string()),
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
            CoreError::Provider(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Provider".to_string()),
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
            CoreError::ProviderSqlite3(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ProviderSqlite3".to_string()),
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
            CoreError::Icu(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("Icu".to_string()),
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
            CoreError::Sqlite3(ref error) => {
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
            CoreError::LanguageTagRegistry(ref error) => {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LanguageTagRegistry".to_string()),
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
            CoreError::Io(ref error) => {
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
            CoreError::ApplicationPath => {
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "application_data".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ApplicationPath".to_string()),
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
            CoreError::ConfigDirNotFound => {
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "config_directory_not_found".to_string(),
                    values: None,
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ConfigDirNotFound".to_string()),
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
            CoreError::NoVendorDir(ref path) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "path".to_string(),
                    PlaceholderValue::String(path.display().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "no_vendor_directory".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NoVendorDir".to_string()),
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
            CoreError::NoConfigFile(ref path) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "path".to_string(),
                    PlaceholderValue::String(path.display().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "no_config_file".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("NoConfigFile".to_string()),
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
            CoreError::WindowIdNotFound(ref id, field) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "id".to_string(),
                    PlaceholderValue::String(format!("{:?}", id)),
                );
                message_values.insert(
                    "field".to_string(),
                    PlaceholderValue::String(format!("{:?}", field)),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "window_id_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("WindowIdNotFound".to_string()),
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
            CoreError::WindowTypeNotFound(ref window_type, field) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
                );
                message_values.insert(
                    "field".to_string(),
                    PlaceholderValue::String(format!("{:?}", field)),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "window_type_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("WindowTypeNotFound".to_string()),
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
            CoreError::ExpectedWindowParent(ref window_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "window_type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "expected_window_parent".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("ExpectedWindowParent".to_string()),
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
            CoreError::LanguageTagNotSupported(ref tag) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert("tag".to_string(), PlaceholderValue::String(tag.to_string()));
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "language_tag".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LanguageTagNotSupported".to_string()),
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
            CoreError::InvalidWindowTypeMain(ref window_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "window_id_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("InvalidWindowTypeMain".to_string()),
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
            CoreError::StateNotReusable(ref window_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "state_not_reusable".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("StateNotReusable".to_string()),
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
            CoreError::PlaceholderNotFound(ref window_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "placeholder_not_found".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("PlaceholderNotFound".to_string()),
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
        }
    }
}

impl Display for CoreError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            CoreError::RonSpannedError(ref error) => Display::fmt(&error, formatter),
            CoreError::RonError(ref error) => Display::fmt(&error, formatter),
            CoreError::LanguageTagRegistry(ref error) => Display::fmt(&error, formatter),
            CoreError::Localiser(ref error) => Display::fmt(&error, formatter),
            CoreError::Provider(ref error) => Display::fmt(&error, formatter),
            CoreError::ProviderSqlite3(ref error) => Display::fmt(&error, formatter),
            CoreError::Icu(ref error) => Display::fmt(&error, formatter),
            CoreError::Sqlite3(ref error) => Display::fmt(&error, formatter),
            CoreError::Io(ref error) => Display::fmt(&error, formatter),
            CoreError::ApplicationPath => {
                write!(formatter, "Failed to retrieve the application path.")
            }
            CoreError::ConfigDirNotFound => write!(
                formatter,
                "Failed to retrieve the user's configuration path."
            ),
            CoreError::NoVendorDir(ref path) => write!(
                formatter,
                "The vendor directory ‘{}’ does not exist.",
                path.display()
            ),
            CoreError::NoConfigFile(ref path) => {
                write!(formatter, "The file ‘{}’ does not exist.", path.display())
            }
            CoreError::WindowIdNotFound(ref id, field) => write!(
                formatter,
                "The window Id ‘{:?}’ was not found in the struct field ‘{:?}’.",
                id, field
            ),
            CoreError::WindowTypeNotFound(ref window_type, field) => write!(
                formatter,
                "The window type ‘{:?}’ was not found in the struct field ‘{:?}’.",
                window_type, field
            ),
            CoreError::ExpectedWindowParent(ref window_type) => write!(
                formatter,
                "Expected to get the parent window for the window type ‘{:?}’.",
                window_type
            ),
            CoreError::LanguageTagNotSupported(ref tag) => write!(
                formatter,
                "The language tag ‘{}’ is supported for the application's user interface.",
                tag
            ),
            CoreError::InvalidWindowTypeMain(ref window_type) => write!(
                formatter,
                "The window type ‘{:?}’ is invalid for a main window.",
                window_type
            ),
            CoreError::StateNotReusable(ref window_type) => write!(
                formatter,
                "The window type ‘{:?}’ is not a reusable state.",
                window_type
            ),
            CoreError::PlaceholderNotFound(ref window_type) => write!(
                formatter,
                "The placeholder of window type ‘{:?}’ is not found.",
                window_type
            ),
        }
    }
}

// Source is embedded in the enum value.
impl Error for CoreError {}

impl From<IoError> for CoreError {
    fn from(error: IoError) -> CoreError {
        CoreError::Io(format!("{}", error.to_string()))
    }
}

impl From<SpannedError> for CoreError {
    fn from(error: SpannedError) -> CoreError {
        CoreError::RonSpannedError(error)
    }
}

impl From<RonError> for CoreError {
    fn from(error: RonError) -> CoreError {
        CoreError::RonError(error)
    }
}

impl From<LocaliserError> for CoreError {
    fn from(error: LocaliserError) -> CoreError {
        CoreError::Localiser(error)
    }
}

impl From<ProviderError> for CoreError {
    fn from(error: ProviderError) -> CoreError {
        CoreError::Provider(error)
    }
}

impl From<ProviderSqlite3Error> for CoreError {
    fn from(error: ProviderSqlite3Error) -> CoreError {
        CoreError::ProviderSqlite3(error)
    }
}

impl From<IcuError> for CoreError {
    fn from(error: IcuError) -> CoreError {
        CoreError::Icu(error)
    }
}

impl From<RegistryError> for CoreError {
    fn from(error: RegistryError) -> CoreError {
        CoreError::LanguageTagRegistry(error)
    }
}

impl From<Sqlite3Error> for CoreError {
    fn from(error: Sqlite3Error) -> CoreError {
        CoreError::Sqlite3(RefCount::new(error))
    }
}
