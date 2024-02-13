// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::window::{
    main,
    preferences,
    confirm_exit,
    fatal_error,
    information,
    about,
};
use std::path::PathBuf;

#[cfg( feature = "persistent" )]
use super::error::ApplicationError;

#[cfg( feature = "persistent" )]
use crate::{
    VENDOR,
    PACKAGE_NAME,
};

#[cfg( feature = "persistent" )]
use dirs::config_dir;

#[cfg( feature = "persistent" )]
use std::fs;

#[cfg( feature = "persistent" )]
use serde::{ Deserialize, Serialize };

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

#[cfg_attr( feature = "persistent", derive( Deserialize, Serialize ) )]
#[derive( Clone )]
pub struct Session {
    pub database_path: PathBuf,
    pub settings: Settings,
}

impl Session {
    #[cfg( feature = "persistent" )]
    pub fn save( &self ) -> Result<(), ApplicationError> {
        let path_config = match config_dir() {
            None => return Err( ApplicationError::ConfigDirNotFound ),
            Some( value ) => value,
        };
        let path_vendor = path_config.join( VENDOR );
        if !path_vendor.exists() {
            fs::create_dir( path_vendor.clone() )?;
        }
        let mut path_file = path_vendor.join( PACKAGE_NAME );
        path_file.set_extension( "ron" );
        let contents = ron::to_string( &self )?;
        fs::write( path_file, contents )?;
        Ok( () )
    }

    #[cfg( feature = "persistent" )]
    pub fn try_restore() -> Result<Session, ApplicationError> {
        let path_config = match config_dir() {
            None => return Err( ApplicationError::ConfigDirNotFound ),
            Some( value ) => value,
        };
        let path_vendor = path_config.join( VENDOR );
        if !path_vendor.exists() {
            return Err( ApplicationError::NoVendorDir( path_vendor ) );
        }
        if !path_vendor.is_dir() {
            return Err( ApplicationError::NoVendorDir( path_vendor ) );
        }
        let mut path_file = path_vendor.join( PACKAGE_NAME );
        path_file.set_extension( "ron" );
        if !path_file.exists() {
            return Err( ApplicationError::NoConfigFile( path_file ) );
        }
        if !path_file.is_file() {
            return Err( ApplicationError::NoConfigFile( path_file ) );
        }
        let string = fs::read_to_string( path_file )?;
        Ok( ron::from_str( string.as_str() )? )
    }
}

impl Default for Session {
    fn default() -> Self {
        Session {
            database_path: dirs::home_dir().unwrap(),
            settings: Settings::default(),
        }
    }
}

#[cfg_attr( feature = "persistent", derive( Deserialize, Serialize ) )]
#[derive( Clone )]
pub struct Settings {
    pub ui: Ui,
    pub log_level: String,
}

impl Default for Settings {
    fn default() -> Self {
        Settings {
            ui: Ui::default(),
            log_level: "warn".to_string(),
        }
    }
}

#[cfg_attr( feature = "persistent", derive( Deserialize, Serialize ) )]
#[derive( Clone )]
pub struct Ui {
    pub language: String,
    pub main: Generic,
    pub preferences: Generic,
    pub confirm_exit: Generic,
    pub fatal_error: Generic,
    pub information: Generic,
    pub about: Generic,
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            language: "en-ZA".to_string(),
            main: Generic { size: main::SIZE_DEFAULT, position: None },
            preferences: Generic { size: preferences::SIZE_DEFAULT, position: None },
            confirm_exit: Generic { size: confirm_exit::SIZE_DEFAULT, position: None },
            fatal_error: Generic { size: fatal_error::SIZE_DEFAULT, position: None },
            information: Generic { size: information::SIZE_DEFAULT, position: None },
            about: Generic { size: about::SIZE_DEFAULT, position: None },
        }
    }
}

#[cfg_attr( feature = "persistent", derive( Deserialize, Serialize ) )]
#[derive( Clone )]
pub struct Generic {
    pub size: ( f32, f32 ),
    pub position: Option<( f32, f32 )>,
}
