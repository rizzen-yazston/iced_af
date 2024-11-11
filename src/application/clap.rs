// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use clap::{Parser, builder::TypedValueParser as _};
use crate::application::log::LogLevel;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// Specify the various command line options.
#[derive(Parser, Debug, Clone)]
#[command( author, version, about, long_about = None )]
pub struct Clap {
    /// To override the log levels that are stored in the session.
    #[arg(
        long,
        value_parser = clap::builder::PossibleValuesParser::new(
            ["default", "off", "error", "warn", "info", "debug", "trace" ]
        )
            .map( |s| {
                s.to_lowercase().as_str().parse::<LogLevel>().unwrap()
            } ),
    )]
    pub log_level: Option<LogLevel>,

    /// Use the application defaults. Reset session data.
    #[arg(short, long)]
    pub defaults: bool,
    
    /*
    /// Name of the person to greet
    #[arg(short, long)]
    name: String,

    /// Number of times to greet
    #[arg(short, long, default_value_t = 1)]
    count: u8,
    */
}
