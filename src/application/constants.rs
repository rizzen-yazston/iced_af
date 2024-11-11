// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use log::LevelFilter;
use phf::phf_map;

//
//
// ----- Derived, no editing required
//
//

// Derived from the build environment:
pub const PACKAGE_NAME: &str = env!("CARGO_PKG_NAME");
pub const VERSION: &str = env!("CARGO_PKG_VERSION");

//
//
// ----- Edit these as needed for the application
//
//

// Project information:
pub const VENDOR: &str = "rizzen"; // Always lower case ASCII letters [a-z].
pub const APPLICATION_NAME: &str = "Iced Application Framework";
pub const APPLICATION_NAME_SHORT: &str = "Iced AF";
pub const APPLICATION_ABBREVIATION: &str = "iced_af";
pub const AUTHORS: &str = env!("CARGO_PKG_AUTHORS"); // Change this if more detailed is required.

// Default settings values
pub const DEFAULT_LOG_LEVEL_FILTER: LevelFilter = LevelFilter::Error; // This is the default log level of entire application.

// Tabs constants
pub const TAB_HEADER_SIZE: u16 = 32;
pub const TAB_PADDING: u16 = 16;

/// As pfh does not support enums as keys due to rust compiler limitations,
/// thus `&str` are used instead. Ensure the `&str` matches the
/// `WindowType::as_str()`.
///
/// Every `WindowType` variant, there must an entry present.
/// Add additional windows to Main windows and Application windows below.
pub static WINDOW_DEFAULT_DATA: phf::Map<&str, WindowDefaultsData> = phf_map! {
    // Core windows
    "About" => WindowDefaultsData {
        size: (300f32, 250f32),
        size_max: (300f32, 250f32),
        size_min: (300f32, 250f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },
    "ConfirmExit" => WindowDefaultsData {
        size: (300f32, 120f32),
        size_max: (300f32, 120f32),
        size_min: (300f32, 120f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },
    "FatalError" => WindowDefaultsData {
        size: (500f32, 200f32),
        size_max: (500f32, 200f32),
        size_min: (500f32, 200f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },
    "Information" => WindowDefaultsData {
        size: (600f32, 200f32),
        size_max: (600f32, 200f32),
        size_min: (600f32, 200f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },
    "Preferences" => WindowDefaultsData {
        size: (500f32, 300f32),
        size_max: (500f32, 500f32),
        size_min: (300f32, 300f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },
    "UnsavedData" => WindowDefaultsData {
        size: (450f32, 120f32),
        size_max: (500f32, 120f32),
        size_min: (300f32, 120f32),
        resizable: false,
        minimise: false,
        maximise: false,
    },

    // Main windows
    "Default" => WindowDefaultsData {
        size: (500f32, 250f32),
        size_max: (1920f32, 1080f32),
        size_min: (300f32, 250f32),
        resizable: true,
        minimise: true,
        maximise: true,
    },
    "Main" => WindowDefaultsData {
        size: (500f32, 250f32),
        size_max: (1920f32, 1080f32),
        size_min: (300f32, 250f32),
        resizable: true,
        minimise: true,
        maximise: true,
    },

    // Application windows

};

/// Simple struct for holding window settings.
#[derive(Debug)]
pub struct WindowDefaultsData {
    pub size: (f32, f32),
    pub size_max: (f32, f32),
    pub size_min: (f32, f32),
    pub resizable: bool,
    pub minimise: bool,
    pub maximise: bool,
}
