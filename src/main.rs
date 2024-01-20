// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The binary entry point.

use iced_af_rizzen_yazston::core::{
    clap::Clap,
    session::Session,
    application::{
        ApplicationThread,
        StartUp,
    },
};
use iced::{
    Settings as ApplicationSettings,
    Point,
    Size,
    Pixels,
    window::{
        Settings as WindowSettings,
        Position
    },
    multi_window::Application,
};
use clap::Parser;

#[cfg( feature = "log" )]
use iced_af::core::log::LogLevelConverter;

#[cfg( feature = "log" )]
use log::warn;

#[cfg( feature = "log" )]
use std::str::FromStr;

/// Simply sets the logger level, and load the previous session's data, before control over to iced to render the main
/// window and handle window events.
fn main() -> iced::Result {

    // Use clap for command line options. See clap.rs for various command options.
    let clap = Clap::parse();

    // TODO: include options:
    // - maximise window
 
    // For now just log to stdout.
    #[cfg( feature = "log" )]
    let mut log_level = match clap.log_level {
        None => LogLevelConverter::Warn,
        Some( value ) => value
    };
    #[cfg( feature = "log" )]
    let logger = log_level.initalise_logger();

    // Restore application state (including main window state)
    let mut session = Session::default();
    let mut first_time = false;
    if !clap.defaults {

        #[cfg( feature = "log" )]
        warn!( "Using saved settings." );

        match Session::try_restore() {
            Err( _error ) => {

                #[cfg( feature = "log" )]
                warn!( "Restore state error: `{:?}`", _error );

                first_time = true
            },
            Ok( value ) => session = value,
        }
    }

    #[cfg( feature = "log" )]
    if clap.log_level.is_none() {
        log_level = match LogLevelConverter::from_str( session.settings.log_level.as_str() ) {
            Err( _error ) => {
                warn!( "Invalid log level in saved session state, using default." );
                session.settings.log_level = "warn".to_string();
                LogLevelConverter::Warn
            },
            Ok( value ) => value,
        };
        log_level.configure_logger( &logger );
    }
    let size_saved = if first_time {
        ( 300f32, 100f32 )
    } else {
        session.settings.ui.main.size
    };
    let size = Size::new( size_saved.0, size_saved.1 );
    let position = if first_time {
        Position::Centered
    } else {
        let option = &session.settings.ui.main.position;
        if option.is_none() {
            Position::Centered
        } else {
            let value = option.as_ref().unwrap();
            Position::Specific( Point { x: value.0, y: value.1 } )
        }
    };
    let start_up = StartUp {
        clap,

        #[cfg( feature = "log" )]
        logger,

        session,
        first_time,
    };
    let iced_settings = ApplicationSettings {
        id: None,
        window: WindowSettings {
            size,
            position,
            exit_on_close_request: false,
            ..Default::default()
        },
        flags: start_up,
        fonts: Default::default(),
        default_font: Default::default(),//Font,
        default_text_size: Pixels( 15.0 ),
        antialiasing: false,
    };
    ApplicationThread::run( iced_settings )
}
