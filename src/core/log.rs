// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use log4rs::{
    append::console::ConsoleAppender,
    config::{ Appender, Root },
    Config,
    Handle,
};
use log::LevelFilter;
use core::fmt::{ Display, Formatter, Result as FormatterResult };

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

#[derive( Copy, Clone, PartialEq, Eq, Debug )]
pub enum LogLevelConverter {
    Off,
    Trace,
    Debug,
    Info,
    Warn,
    Error,
}

impl LogLevelConverter {
    /// Convert `LogLevelConverter`` to [`log::LevelFilter`].
    pub fn to_level_filter( &self ) -> LevelFilter {
        match self {
            LogLevelConverter::Off => LevelFilter::Off,
            LogLevelConverter::Trace => LevelFilter::Trace,
            LogLevelConverter::Debug => LevelFilter::Debug,
            LogLevelConverter::Info => LevelFilter::Info,
            LogLevelConverter::Warn => LevelFilter::Warn,
            LogLevelConverter::Error => LevelFilter::Error,
        }
    }

    /// Initialise the logger and obtain the logger [`Handle`].
    pub fn initalise_logger( &self ) -> Handle {
        println!( "Log level set to ‘{}’", self );
        log4rs::init_config( self.logger_config() ).unwrap()
    }

    /// Configure the logger [`Handle`].
    pub fn configure_logger( &self, logger: &Handle ) {
        println!( "Log level set to ‘{}’", self );
        logger.set_config( self.logger_config() );
    }

    /// Obtain the logger [`Config`].
    fn logger_config( &self ) -> Config {
        let stdout = ConsoleAppender::builder().build();
        Config::builder()
            .appender( Appender::builder().build( "stdout", Box::new( stdout ) ) )
            .build( Root::builder().appender( "stdout" ).build( self.to_level_filter() ) )
            .unwrap()
    }
}

impl Display for LogLevelConverter {
    fn fmt( &self, formatter: &mut Formatter ) -> FormatterResult {
        match self {
            LogLevelConverter::Off => write!( formatter, "Off" ),
            LogLevelConverter::Trace => write!( formatter, "trace" ),
            LogLevelConverter::Debug => write!( formatter, "debug" ),
            LogLevelConverter::Info => write!( formatter, "info" ),
            LogLevelConverter::Warn => write!( formatter, "warn" ),
            LogLevelConverter::Error => write!( formatter, "error" ),
        }
    }
}

impl std::str::FromStr for LogLevelConverter {

    // This must be String due to map() of clap expecting String value for error.
    type Err = String;

    fn from_str( string: &str ) -> Result<Self, Self::Err> {
        let s = string.to_lowercase();
        match s.as_str() {
            "off" => Ok( Self::Off ),
            "trace" => Ok( Self::Trace ),
            "debug" => Ok( Self::Debug ) ,
            "info" => Ok( Self::Info ),
            "warn" => Ok( Self::Warn ),
            "error" => Ok( Self::Error ),
            _ => Err( format!( "Unknown log level: {s} )" ) ),
        }
    }
}
