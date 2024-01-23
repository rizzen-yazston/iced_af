// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::error::ApplicationError;
use crate::window::main;
use serde::{ Deserialize, Serialize };
use dirs::config_dir;
use std::{
    fs,
    path::PathBuf,
};

#[derive( Clone, Deserialize, Serialize )]
pub struct Session {
    pub database_path: PathBuf,
    pub settings: Settings,
}

impl Session {
    pub fn save( &self ) -> Result<(), ApplicationError> {
        let path_config = match config_dir() {
            None => return Err( ApplicationError::ConfigDirNotFound ),
            Some( value ) => value,
        };
        let path_vendor = path_config.join( "iced_af" );
        if !path_vendor.exists() {
            fs::create_dir( path_vendor.clone() )?;
        }
        let path_file = path_vendor.join( "iced_af.ron" );
        let contents = ron::to_string( &self )?;
        fs::write( path_file, contents )?;
        Ok( () )
    }

    pub fn try_restore() -> Result<Session, ApplicationError> {
        let path_config = match config_dir() {
            None => return Err( ApplicationError::ConfigDirNotFound ),
            Some( value ) => value,
        };
        let path_vendor = path_config.join( "iced_af" );
        if !path_vendor.exists() {
            return Err( ApplicationError::NoVendorDir( path_vendor ) );
        }
        if !path_vendor.is_dir() {
            return Err( ApplicationError::NoVendorDir( path_vendor ) );
        }
        let path_file = path_vendor.join( "iced_af.ron" );
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

#[derive( Clone, Deserialize, Serialize )]
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

#[derive( Clone, Deserialize, Serialize )]
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
            main: Generic::default(),
            preferences: Generic::default(),
            confirm_exit: Generic::default(),
            fatal_error: Generic::default(),
            information: Generic::default(),
            about: Generic::default(),
        }
    }
}

#[derive( Clone, Deserialize, Serialize )]
pub struct Generic {
    pub size: ( f32, f32 ),
    pub position: Option<( f32, f32 )>,
}

impl Default for Generic {
    fn default() -> Self {
        Generic {
            size: main::SIZE_DEFAULT,
            position: None,
        }
    }
}
