// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! Session data to be saved when application terminates.
//!
//! Add data as needed.

use crate::{
    application::{constants, log::LogLevel, WindowType},
    core::error::CoreError,
};
use std::{
    collections::{BTreeMap, VecDeque},
    fs,
    path::PathBuf,
};
use dirs::config_dir;
use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Deserialize, Serialize, Clone)]
pub struct Session {
    pub settings: Settings,
    pub history: VecDeque<PathBuf>, // Hold last 10 opened databases.
    pub windows: BTreeMap<WindowType, WindowData>,
}

impl Session {
    pub fn save(&self) -> Result<(), CoreError> {
        let path_config = match config_dir() {
            None => return Err(CoreError::ConfigDirNotFound),
            Some(value) => value,
        };
        let path_vendor = path_config.join(constants::VENDOR);
        if !path_vendor.exists() {
            fs::create_dir(path_vendor.clone())?;
        }
        let mut path_file = path_vendor.join(constants::PACKAGE_NAME);
        path_file.set_extension("ron");
        let contents = ron::to_string(&self)?;
        fs::write(path_file, contents)?;
        Ok(())
    }

    pub fn try_restore() -> Result<Session, CoreError> {
        let path_config = match config_dir() {
            None => return Err(CoreError::ConfigDirNotFound),
            Some(value) => value,
        };
        let path_vendor = path_config.join(constants::VENDOR);
        if !path_vendor.exists() {
            return Err(CoreError::NoVendorDir(path_vendor));
        }
        if !path_vendor.is_dir() {
            return Err(CoreError::NoVendorDir(path_vendor));
        }
        let mut path_file = path_vendor.join(constants::PACKAGE_NAME);
        path_file.set_extension("ron");
        if !path_file.exists() {
            return Err(CoreError::NoConfigFile(path_file));
        }
        if !path_file.is_file() {
            return Err(CoreError::NoConfigFile(path_file));
        }
        let string = fs::read_to_string(path_file)?;
        Ok(ron::from_str(string.as_str())?)
    }
}

impl Default for Session {
    fn default() -> Self {
        let windows = BTreeMap::<WindowType, WindowData>::new();
        Session {
            settings: Settings::default(),
            history: VecDeque::<PathBuf>::new(),
            windows,
        }
    }
}

#[derive(Deserialize, Serialize, Clone, Default)]
pub struct Settings {
    pub ui: Ui,
    pub log_levels: LogLevels,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct Ui {
    pub language: String,
}

impl Default for Ui {
    fn default() -> Self {
        Ui {
            language: "en_ZA".to_string(),  // Same as the default language of the "application"
                                            // component in localisation database.
        }
    }
}

#[derive(Deserialize, Serialize, Clone)]
pub struct WindowData {
    pub size: (f32, f32),
    pub position: Option<(f32, f32)>,
}

#[derive(Deserialize, Serialize, Clone)]
pub struct LogLevels {
    pub default: LogLevel,
    pub application: LogLevel,
    pub other: LogLevel,
    pub iced: LogLevel,
    pub i18n: LogLevel,
}

impl Default for LogLevels {
    fn default() -> Self {
        LogLevels {
            default: LogLevel::Error,
            application: LogLevel::Info,
            other: LogLevel::Default,
            iced: LogLevel::Default,
            i18n: LogLevel::Default,
        }
    }
}
