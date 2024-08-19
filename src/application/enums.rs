// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The available type of windows in the application.
//!
//! Remember to also add to the src/application/constants.rs file.

use serde::{Deserialize, Serialize};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Deserialize, Serialize, Debug, Clone, Eq, PartialEq, Hash, Ord, PartialOrd)]
pub enum WindowType {
    Default, // The default main window.

    // Core windows
    ConfirmExit,
    FatalError,
    Information,
    Preferences,
    About,
    UnsavedData,

    // Main windows
    Main,

    // Application windows
}

impl WindowType {
    pub fn as_str(&self) -> &str {
        match self {
            WindowType::Default => "Default",

            // Core windows
            WindowType::ConfirmExit => "ConfirmExit",
            WindowType::FatalError => "FatalError",
            WindowType::Information => "Information",
            WindowType::Preferences => "Preferences",
            WindowType::About => "About",
            WindowType::UnsavedData => "UnsavedData",

            // Main windows
            WindowType::Main => "Main",

            // Application windows
        }
    }
}

#[derive(Debug, Clone, Eq, PartialEq, Hash)]
pub enum StringGroup {
    // Core windows
    ConfirmExit,
    FatalError,
    Information,
    Preferences,
    About,
    UnsavedData,

    // Main windows
    MainCommon,
    Default,
    Main,

    // Application windows
}

impl StringGroup {
    pub fn as_str(&self) -> &str {
        match self {
            // Core windows
            StringGroup::ConfirmExit => "ConfirmExit",
            StringGroup::FatalError => "FatalError",
            StringGroup::Information => "Information",
            StringGroup::Preferences => "Preferences",
            StringGroup::About => "About",
            StringGroup::UnsavedData => "UnsavedData",

            // Main windows
            StringGroup::MainCommon => "MainCommon",
            StringGroup::Default => "Default",
            StringGroup::Main => "Main",

            // Application windows
        }
    }
}
