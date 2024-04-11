// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::application::WindowType;
use core::fmt::{Display, Formatter, Result};
use iced::window;
use std::{error::Error, fmt::Debug, io::Error as IoError, path::PathBuf};

#[cfg(feature = "i18n")]
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

#[cfg(feature = "persistent")]
use ron::error::{Error as RonError, SpannedError};

#[cfg(feature = "persistent")]
use std::collections::HashMap;

#[derive(Debug, Clone)]
#[non_exhaustive]
pub enum ApplicationError {
    #[cfg(feature = "persistent")]
    RonSpannedError(SpannedError),
    #[cfg(feature = "persistent")]
    RonError(RonError),
    #[cfg(feature = "i18n")]
    LanguageTagRegistry(RegistryError),
    #[cfg(feature = "i18n")]
    Localiser(LocaliserError),
    #[cfg(feature = "i18n")]
    Provider(ProviderError),
    #[cfg(feature = "i18n")]
    ProviderSqlite3(ProviderSqlite3Error),
    #[cfg(feature = "i18n")]
    Icu(IcuError),
    Io(String), // Can't clone io::Error, as it is an OS error, thus converted to final String (can't be translated).
    ApplicationPath,
    ConfigDirNotFound,
    NoVendorDir(PathBuf),
    NoConfigFile(PathBuf),
    DatabaseAlreadyOpen,
    WindowIdNotFound(window::Id),
    WindowTypeNotFound(WindowType),
    ExpectedWindowParent(WindowType),
    InvalidSchema(String),
    LogLevel(String),
}

#[cfg(feature = "i18n")]
impl LocalisationErrorTrait for ApplicationError {}

#[cfg(feature = "i18n")]
impl LocalisationTrait for ApplicationError {
    fn localisation_data(&self) -> LocalisationData {
        let type_string = PlaceholderValue::String("ApplicationError".to_string());
        match self {
            #[cfg(feature = "persistent")]
            ApplicationError::RonSpannedError(ref error) => {
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

            #[cfg(feature = "persistent")]
            ApplicationError::RonError(ref error) => {
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

            ApplicationError::Localiser(ref error) => {
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
            ApplicationError::Provider(ref error) => {
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
            ApplicationError::ProviderSqlite3(ref error) => {
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
            ApplicationError::Icu(ref error) => {
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
            ApplicationError::LanguageTagRegistry(ref error) => {
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
            ApplicationError::ApplicationPath => {
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
            ApplicationError::ConfigDirNotFound => {
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
            ApplicationError::NoVendorDir(ref path) => {
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
            ApplicationError::NoConfigFile(ref path) => {
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
            ApplicationError::WindowIdNotFound(ref id) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "window_type".to_string(),
                    PlaceholderValue::String(format!("{:?}", id)),
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
            ApplicationError::WindowTypeNotFound(ref window_type) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "window_type".to_string(),
                    PlaceholderValue::String(window_type.as_str().to_string()),
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
            ApplicationError::ExpectedWindowParent(ref window_type) => {
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
            ApplicationError::LogLevel(ref name) => {
                let mut message_values = HashMap::<String, PlaceholderValue>::new();
                message_values.insert(
                    "level".to_string(),
                    PlaceholderValue::String(name.to_string()),
                );
                let message = LocalisationData {
                    component: "application".to_string(),
                    identifier: "unknown_log_level".to_string(),
                    values: Some(message_values),
                };
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert("type".to_string(), type_string);
                values.insert(
                    "variant".to_string(),
                    PlaceholderValue::String("LogLevel".to_string()),
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

impl Display for ApplicationError {
    fn fmt(&self, formatter: &mut Formatter) -> Result {
        match self {
            #[cfg(feature = "persistent")]
            ApplicationError::RonSpannedError(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "persistent")]
            ApplicationError::RonError(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "i18n")]
            ApplicationError::LanguageTagRegistry(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "i18n")]
            ApplicationError::Localiser(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "i18n")]
            ApplicationError::Provider(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "i18n")]
            ApplicationError::ProviderSqlite3(ref error) => Display::fmt(&error, formatter),

            #[cfg(feature = "i18n")]
            ApplicationError::Icu(ref error) => Display::fmt(&error, formatter),

            ApplicationError::Io(ref error) => Display::fmt(&error, formatter),
            ApplicationError::ApplicationPath => {
                write!(formatter, "Failed to retrieve the application path.")
            }
            ApplicationError::ConfigDirNotFound => write!(
                formatter,
                "Failed to retrieve the user's configuration path."
            ),
            ApplicationError::NoVendorDir(ref path) => write!(
                formatter,
                "The vendor directory ‘{}’ does not exist.",
                path.display()
            ),
            ApplicationError::NoConfigFile(ref path) => {
                write!(formatter, "The file ‘{}’ does not exist.", path.display())
            }
            ApplicationError::DatabaseAlreadyOpen => {
                write!(formatter, "The database is already opened.")
            }
            ApplicationError::WindowIdNotFound(ref id) => write!(
                formatter,
                "The window Id ‘{:?}’ was not found in the struct field ‘window_ids’.",
                id
            ),
            ApplicationError::WindowTypeNotFound(ref window_type) => write!(
                formatter,
                "The window type ‘{:?}’ was not found in the struct field ‘windows’.",
                window_type
            ),
            ApplicationError::ExpectedWindowParent(ref window_type) => write!(
                formatter,
                "Expected to get the parent window for the window type ‘{:?}’.",
                window_type
            ),
            ApplicationError::InvalidSchema(ref name) => write!(
                formatter,
                "The Sqlite3 file schema is invalid for the database ‘{}’.",
                name
            ),
            ApplicationError::LogLevel(ref level) => {
                write!(formatter, "Unknown log level: ‘{}’.", level)
            }
        }
    }
}

// Source is embedded in the enum value.
impl Error for ApplicationError {}

impl From<IoError> for ApplicationError {
    fn from(error: IoError) -> ApplicationError {
        ApplicationError::Io(error.to_string())
    }
}

#[cfg(feature = "persistent")]
impl From<SpannedError> for ApplicationError {
    fn from(error: SpannedError) -> ApplicationError {
        ApplicationError::RonSpannedError(error)
    }
}

#[cfg(feature = "persistent")]
impl From<RonError> for ApplicationError {
    fn from(error: RonError) -> ApplicationError {
        ApplicationError::RonError(error)
    }
}

#[cfg(feature = "i18n")]
impl From<LocaliserError> for ApplicationError {
    fn from(error: LocaliserError) -> ApplicationError {
        ApplicationError::Localiser(error)
    }
}

#[cfg(feature = "i18n")]
impl From<ProviderError> for ApplicationError {
    fn from(error: ProviderError) -> ApplicationError {
        ApplicationError::Provider(error)
    }
}

#[cfg(feature = "i18n")]
impl From<ProviderSqlite3Error> for ApplicationError {
    fn from(error: ProviderSqlite3Error) -> ApplicationError {
        ApplicationError::ProviderSqlite3(error)
    }
}

#[cfg(feature = "i18n")]
impl From<IcuError> for ApplicationError {
    fn from(error: IcuError) -> ApplicationError {
        ApplicationError::Icu(error)
    }
}

#[cfg(feature = "i18n")]
impl From<RegistryError> for ApplicationError {
    fn from(error: RegistryError) -> ApplicationError {
        ApplicationError::LanguageTagRegistry(error)
    }
}
