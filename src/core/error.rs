// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::application::WindowType;
use i18n::{
    localiser::LocaliserError,
    provider::ProviderError,
    provider_sqlite3::ProviderSqlite3Error,
    icu::IcuError,
    utility::{ RegistryError, LocalisationTrait, LocalisationErrorTrait },
};
use rusqlite::Error as Sqlite3Error;
use ron::error::{ SpannedError, Error as RonError };
use iced::window;
use std::{
    io::Error as IoError,
    error::Error, // Experimental in `core` crate.
    path::PathBuf,
    sync::Arc,
};
use core::fmt::{ Display, Formatter, Result };

#[derive( Debug, Clone )]
#[non_exhaustive]
pub enum ApplicationError {
    ConfigDirNotFound,
    NoVendorDir( PathBuf ),
    NoConfigFile( PathBuf ),
    Io( String ), // Can't clone io::Error, as it is an OS error, thus converted to final String (can't be translated).
    RonSpannedError( SpannedError ),
    RonError( RonError ),
    ApplicationPath,
    Localiser( Arc<LocaliserError> ),
    Provider( Arc<ProviderError> ),
    ProviderSqlite3( Arc<ProviderSqlite3Error> ),
    Icu( Arc<IcuError> ),
    Sqlite3( Arc<Sqlite3Error> ),
    LanguageTagRegistry( RegistryError ),
    DatabaseAlreadyOpen,
    WindowIdNotFound( window::Id ),
    WindowTypeNotFound( WindowType ),
    ExpectedWindowParent( WindowType ),
    InvalidSchema( String ),
}

impl LocalisationTrait for ApplicationError {
    fn identifier( &self ) -> &str {
        match *self {
            ApplicationError::ConfigDirNotFound => "config_directory_not_found",
            ApplicationError::NoVendorDir( _ ) => "no_vendor_directory",
            ApplicationError::NoConfigFile( _ ) => "no_config_file",
            ApplicationError::ApplicationPath => "application_data",
            ApplicationError::DatabaseAlreadyOpen => "database_already_opened",
            ApplicationError::WindowIdNotFound( _ ) => "window_id_not_found",
            ApplicationError::WindowTypeNotFound( _ ) => "window_type_not_found",
            ApplicationError::ExpectedWindowParent( _ ) => "expected_window_parent",
            ApplicationError::InvalidSchema( _ ) => "schema_invalid",

            #[allow( unreachable_patterns )]
            _ => "",
        }
    }

    fn component( &self ) -> &str {
        "application"
    }
}

impl LocalisationErrorTrait for ApplicationError {
    fn error_type( &self ) -> &str {
        "ApplicationError"
    }

    fn error_variant( &self ) -> &str {
        match &self {
            ApplicationError::ConfigDirNotFound => "ConfigDirNotFound",
            ApplicationError::NoVendorDir( _ ) => "NoVendorDir",
            ApplicationError::NoConfigFile( _ ) => "NoConfigFile",
            ApplicationError::Io( _ ) => "Io",
            ApplicationError::RonSpannedError( _ ) => "RonSpannedError",
            ApplicationError::RonError( _ ) => "RonError",
            ApplicationError::ApplicationPath => "ApplicationPath",
            ApplicationError::Localiser( _ ) => "Localiser",
            ApplicationError::Provider( _ ) => "Provider",
            ApplicationError::ProviderSqlite3( _ ) => "ProviderSqlite3",
            ApplicationError::Icu( _ ) => "Icu",
            ApplicationError::Sqlite3( _ ) => "Sqlite3",
            ApplicationError::LanguageTagRegistry( _ ) => "LanguageTagRegistry",
            ApplicationError::DatabaseAlreadyOpen => "DatabaseAlreadyOpen",
            ApplicationError::WindowIdNotFound( _ ) => "WindowIdNotFound",
            ApplicationError::WindowTypeNotFound( _ ) => "WindowTypeNotFound",
            ApplicationError::ExpectedWindowParent( _ ) => "ExpectedWindowParent",
            ApplicationError::InvalidSchema( _ ) => "InvalidSchema",
        }
    }
}

impl Display for ApplicationError {
    fn fmt( &self, formatter: &mut Formatter ) -> Result {
        match self {
            ApplicationError::ConfigDirNotFound => write!(
                formatter, "Failed to retrieve the user's configuration path."
        ),
            ApplicationError::NoVendorDir( ref path ) => write!(
                formatter,
                "The vendor directory ‘{}’ does not exist.",
                path.display()
            ),
            ApplicationError::NoConfigFile( ref path ) => write!(
                formatter,
                "The file ‘{}’ does not exist.",
                path.display()
            ),
            ApplicationError::Io( ref error ) => error.fmt( formatter ),
            ApplicationError::RonSpannedError( ref error ) => error.fmt( formatter ),
            ApplicationError::RonError( ref error ) => error.fmt( formatter ),
            ApplicationError::ApplicationPath => write!(
                formatter, "Failed to retrieve the application path."
            ),
            ApplicationError::Localiser( ref error ) => error.fmt( formatter ),
            ApplicationError::Provider( ref error ) => error.fmt( formatter ),
            ApplicationError::ProviderSqlite3( ref error ) => error.fmt( formatter ),
            ApplicationError::Icu( ref error ) => error.fmt( formatter ),
            ApplicationError::Sqlite3( ref error ) => error.fmt( formatter ),
            ApplicationError::LanguageTagRegistry( ref error ) => error.fmt( formatter ),
            ApplicationError::DatabaseAlreadyOpen => write!(
                formatter, "The database is already opened."
            ),
            ApplicationError::WindowIdNotFound( ref id ) => write!(
                formatter,
                "The window Id ‘{:?}’ was not found in the struct field ‘window_ids’.",
                id
            ),
            ApplicationError::WindowTypeNotFound( ref window_type ) => write!(
                formatter,
                "The window type ‘{:?}’ was not found in the struct field ‘windows’.",
                window_type
            ),
            ApplicationError::ExpectedWindowParent( ref window_type ) => write!(
                formatter,
                "Expected to get the parent window for the window type ‘{:?}’.",
                window_type
            ),
            ApplicationError::InvalidSchema( ref name ) => write!(
                formatter,
                "The Sqlite3 file schema is invalid for the database ‘{}’.",
                name
            )
        }
    }
}

// Source is embedded in the enum value.
impl Error for ApplicationError {}

impl From<IoError> for ApplicationError {
    fn from( error: IoError ) -> ApplicationError {
        ApplicationError::Io( format!( "{}", error.to_string() ) )
    }
}

impl From<SpannedError> for ApplicationError {
    fn from( error: SpannedError ) -> ApplicationError {
        ApplicationError::RonSpannedError( error )
    }
}

impl From<RonError> for ApplicationError {
    fn from( error: RonError ) -> ApplicationError {
        ApplicationError::RonError( error )
    }
}

impl From<LocaliserError> for ApplicationError {
    fn from( error: LocaliserError ) -> ApplicationError {
        ApplicationError::Localiser( Arc::new( error ) )
    }
}

impl From<ProviderError> for ApplicationError {
    fn from( error: ProviderError ) -> ApplicationError {
        ApplicationError::Provider( Arc::new( error ) )
    }
}

impl From<ProviderSqlite3Error> for ApplicationError {
    fn from( error: ProviderSqlite3Error ) -> ApplicationError {
        ApplicationError::ProviderSqlite3( Arc::new( error ) )
    }
}

impl From<Sqlite3Error> for ApplicationError {
    fn from( error: Sqlite3Error ) -> ApplicationError {
        ApplicationError::Sqlite3( Arc::new( error ) )
    }
}

impl From<IcuError> for ApplicationError {
    fn from( error: IcuError ) -> ApplicationError {
        ApplicationError::Icu( Arc::new( error ) )
    }
}

impl From<RegistryError> for ApplicationError {
    fn from( error: RegistryError ) -> ApplicationError {
        ApplicationError::LanguageTagRegistry( error )
    }
}
