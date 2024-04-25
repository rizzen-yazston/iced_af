// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use clap::Parser;

#[cfg(feature = "log")]
use clap::builder::TypedValueParser as _;

#[cfg(feature = "log")]
use crate::core::log::LogLevel;

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Parser, Debug, Clone)]
#[command( author, version, about, long_about = None )]
pub struct Clap {
    #[cfg(feature = "log")]
    #[arg(
        long,
        value_parser = clap::builder::PossibleValuesParser::new(
            ["default", "off", "trace", "debug", "info", "warn", "error"]
        )
        .map(|s| {
            s.to_lowercase().as_str().parse::<LogLevel>().unwrap()
        }),
    )]
    #[cfg(feature = "log")]
    pub log_level: Option<LogLevel>,

    #[arg(short, long)]
    pub defaults: bool,
    /* examples
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    */
}
