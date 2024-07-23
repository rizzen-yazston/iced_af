// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! TODO: figure out how to eliminate duplicate entries.

use crate::application::{
    constants,
    session::LogLevels,
};
use core::fmt::{Display, Formatter, Result as FormatterResult};
use log::LevelFilter;
use log4rs::{
    append::console::ConsoleAppender,
    config::{Appender, Logger, Root},
    Config, Handle,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};
use serde::{Deserialize, Serialize};

/// Create a new application logger
pub fn new_logger(default: LogLevel) -> Handle {
    let default = if default == LogLevel::Default {
        // Invalid variant, silently change to LogLevel::Error
        LogLevel::Error
    } else {
        default
    };
    println!("Initialise: Log level set to ‘{}’", default); // Keep this line
    let stdout = ConsoleAppender::builder().build();
    log4rs::init_config(
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            .build(
                Root::builder()
                    .appender("stdout")
                    .build(default.to_level_filter()),
            )
            .unwrap(),
    )
    .unwrap()
}

/// Update the log levels of the logger.
pub fn update_logger(handle: &mut Handle, log_levels: &LogLevels) {
    let application = match log_levels.application {
        LogLevel::Default => log_levels.default,
        _ => log_levels.application,
    };
    let other = match log_levels.other {
        LogLevel::Default => log_levels.default,
        _ => log_levels.other,
    };
    let iced = match log_levels.iced {
        LogLevel::Default => log_levels.default,
        _ => log_levels.iced,
    };
    let i18n = match log_levels.i18n {
        LogLevel::Default => log_levels.default,
        _ => log_levels.i18n,
    };
    let stdout = ConsoleAppender::builder().build();
    handle.set_config(
        Config::builder()
            .appender(Appender::builder().build("stdout", Box::new(stdout)))
            // the application itself
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("iced_af_rizzen_yazston", application.to_level_filter()),
            )
            // iced components (iced depends on many crates)
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("iced_wgpu", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("wgpu_core", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("iced_graphics", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("cosmic_text", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("calloop", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("naga", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("fontdb", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("wgpu_hal", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("winit", iced.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("iced_winit", iced.to_level_filter()),
            )
            // i18n components (i18n has a few crates)
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n", i18n.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n_lexer", i18n.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n_localiser", i18n.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n_provider", i18n.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n_provider_sqlite3", i18n.to_level_filter()),
            )
            .logger(
                Logger::builder()
                    .appender("stdout")
                    .build("i18n_utility", i18n.to_level_filter()),
            )
            .build(
                Root::builder()
                    .appender("stdout")
                    .build(other.to_level_filter()),
            )
            .unwrap(),
    );
    println!("Log levels has been updated.");
}

#[derive(Copy, Clone, PartialEq, Eq, Debug, Default, Hash, Deserialize, Serialize)]
pub enum LogLevel {
    #[default]
    Default,
    Off,
    Error,
    Warn,
    Debug,
    Info,
    Trace,
}

impl LogLevel {
    /// Convert `LogLevel` to [`log::LevelFilter`].
    pub fn to_level_filter(&self) -> LevelFilter {
        match self {
            LogLevel::Default => constants::DEFAULT_LOG_LEVEL_FILTER,
            LogLevel::Off => LevelFilter::Off,
            LogLevel::Error => LevelFilter::Error,
            LogLevel::Warn => LevelFilter::Warn,
            LogLevel::Info => LevelFilter::Info,
            LogLevel::Debug => LevelFilter::Debug,
            LogLevel::Trace => LevelFilter::Trace,
        }
    }
}

impl Display for LogLevel {
    fn fmt(&self, formatter: &mut Formatter) -> FormatterResult {
        match self {
            LogLevel::Default => write!(formatter, "default"),
            LogLevel::Off => write!(formatter, "off"),
            LogLevel::Error => write!(formatter, "error"),
            LogLevel::Warn => write!(formatter, "warn"),
            LogLevel::Info => write!(formatter, "info"),
            LogLevel::Debug => write!(formatter, "debug"),
            LogLevel::Trace => write!(formatter, "trace"),
        }
    }
}

impl std::str::FromStr for LogLevel {
    // This must be String due to map() of clap expecting String value for error.
    type Err = String;

    fn from_str(string: &str) -> Result<Self, Self::Err> {
        let s = string.to_lowercase();
        match s.as_str() {
            "default" => Ok(Self::Default),
            "off" => Ok(Self::Off),
            "trace" => Ok(Self::Trace),
            "debug" => Ok(Self::Debug),
            "info" => Ok(Self::Info),
            "warn" => Ok(Self::Warn),
            "error" => Ok(Self::Error),
            _ => Err(format!("Unknown log level: {s}")),
        }
    }
}
