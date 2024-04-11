// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The binary entry point.

use iced::{
    multi_window::Application,
    window::{Position, Settings as WindowSettings},
    Pixels, Point, Settings as ApplicationSettings, Size,
};
use iced_af::core::{
    application::{ApplicationThread, StartUp},
    session::Session,
};

#[cfg(feature = "clap")]
use iced_af::core::clap::Clap;

#[cfg(feature = "clap")]
use clap::Parser;

#[cfg(feature = "log")]
use iced_af::core::log::{new_logger, update_logger, LogLevel};

#[cfg(feature = "log")]
use log::warn;

/// Depending on the features selected certain preparations for displaying the first window of the application is done.
/// These preparations may included: some command arguments processing, restoration of application's session, first use
/// of application (requires the `persistent` feature), and logging of messages.
fn main() -> iced::Result {
    // Use clap for command line options. See clap.rs for various command options.
    #[cfg(feature = "clap")]
    let clap = Clap::parse();

    // TODO: include options:
    // - maximise window

    // For now just log to stdout.
    #[cfg(all(feature = "clap", feature = "log"))]
    let log_level = match clap.log_level {
        None => LogLevel::Warn,
        Some(value) => value,
    };

    #[cfg(all(not(feature = "clap"), feature = "log"))]
    let mut log_level = LogLevel::Warn;

    #[cfg(feature = "log")]
    let mut logger = new_logger(log_level);

    #[allow(unused_mut)]
    let mut session = Session::default();

    #[cfg(feature = "first_use")]
    let mut first_use = false;

    // Restore application state (including main window state)
    #[cfg(all(feature = "clap", feature = "persistent"))]
    if !clap.defaults {
        #[cfg(feature = "log")]
        warn!("Using saved settings.");

        match Session::try_restore() {
            Err(_error) => {
                #[cfg(feature = "log")]
                warn!("Restore state error: `{:?}`", _error);

                #[cfg(feature = "first_use")]
                {
                    first_use = true
                }
            }
            Ok(value) => {
                #[cfg(feature = "log")]
                warn!("Using saved settings.");

                session = value;
            }
        }
    }

    #[cfg(all(not(feature = "clap"), feature = "persistent"))]
    match Session::try_restore() {
        Err(_error) => {
            #[cfg(feature = "log")]
            warn!("Restore state error: `{:?}`", _error);

            #[cfg(feature = "first_use")]
            {
                first_use = true
            }
        }
        Ok(value) => {
            #[cfg(feature = "log")]
            warn!("Using saved settings.");

            session = value;
        }
    }

    #[cfg(all(feature = "clap", feature = "log"))]
    if clap.log_level.is_none() {
        update_logger(&mut logger, &session.settings.log_levels);
    }

    #[cfg(feature = "first_use")]
    let size_saved = if first_use {
        session.settings.ui.preferences.size
    } else {
        session.settings.ui.main.size
    };

    #[cfg(not(feature = "first_use"))]
    let size_saved = session.settings.ui.main.size;

    let size = Size::new(size_saved.0, size_saved.1);

    #[cfg(feature = "first_use")]
    let position = if first_use {
        Position::Centered
    } else {
        let option = &session.settings.ui.main.position;
        if option.is_none() {
            Position::Centered
        } else {
            let value = option.as_ref().unwrap();
            Position::Specific(Point {
                x: value.0,
                y: value.1,
            })
        }
    };

    #[cfg(not(feature = "first_use"))]
    let position = {
        let option = &session.settings.ui.main.position;
        if option.is_none() {
            Position::Centered
        } else {
            let value = option.as_ref().unwrap();
            Position::Specific(Point {
                x: value.0,
                y: value.1,
            })
        }
    };

    let start_up = StartUp {
        session,

        #[cfg(feature = "clap")]
        clap,

        #[cfg(feature = "log")]
        logger,

        #[cfg(feature = "first_use")]
        first_use,
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
        default_font: Default::default(), //Font,
        default_text_size: Pixels(15.0),
        antialiasing: false,
    };
    ApplicationThread::run(iced_settings)
}
