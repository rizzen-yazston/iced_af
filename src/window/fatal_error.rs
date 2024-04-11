// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    core::{
        application::{ApplicationMessage, ApplicationThread, WindowType},
        error::ApplicationError,
        traits::{AnyWindowTrait, WindowTrait},
    },
    APPLICATION_NAME_SHORT,
};
use iced::{
    widget::{button, column, container, scrollable, text},
    window, Alignment, Command, Element, Length, Point, Size,
};
use std::any::Any;

#[cfg(feature = "i18n")]
use crate::core::localisation::{Localisation, ScriptData};

#[cfg(feature = "i18n")]
use i18n::utility::{LocalisationTrait, PlaceholderValue, TaggedString as LString};

#[cfg(not(feature = "i18n"))]
use std::string::String as LString;

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(feature = "i18n")]
use std::collections::HashMap;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 300f32, 150f32 );
pub const SIZE_DEFAULT: (f32, f32) = (500f32, 200f32);
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct FatalError {
    #[cfg(feature = "i18n")]
    script_data: ScriptData,
    enabled: bool,
    parent: Option<WindowType>,
    title: LString,
    uncaught_error: LString,
    exit: LString,
}

impl FatalError {
    pub fn new(
        #[cfg(feature = "i18n")] localisation: &Localisation,
        error: ApplicationError,
    ) -> Self {
        #[cfg(feature = "i18n")]
        let language = localisation.localiser().default_language();

        #[cfg(feature = "i18n")]
        let en_za = localisation
            .localiser()
            .language_tag_registry()
            .tag("en-za")
            .unwrap();

        #[cfg(feature = "i18n")]
        let language_identifier = localisation
            .localiser()
            .language_tag_registry()
            .identifier(language.as_str())
            .unwrap_or(
                localisation
                    .localiser()
                    .language_tag_registry()
                    .identifier(en_za.as_str())
                    .unwrap(),
            );

        #[cfg(feature = "i18n")]
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            values.insert(
                "window".to_string(),
                PlaceholderValue::TaggedString(
                    localisation
                        .localiser()
                        .literal_with_defaults("application", "fatal_error")
                        .unwrap_or(LString::new("Fatal error", &en_za)),
                ),
            );
            localisation
                .localiser()
                .format_with_defaults("application", "window_title_format", &values)
                .unwrap_or(LString::new(
                    format!("{} - Fatal error", APPLICATION_NAME_SHORT),
                    &en_za,
                ))
        };

        #[cfg(not(feature = "i18n"))]
        let title = format!("{} - Fatal error", APPLICATION_NAME_SHORT);

        #[cfg(feature = "i18n")]
        let uncaught_error = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "error".to_string(),
                PlaceholderValue::LocalisationData(error.localisation_data()),
            );
            localisation
                .localiser()
                .format_with_defaults("application", "uncaught_error", &values)
                .unwrap_or(LString::new(
                    format!("The following error was not caught: '{}'", error),
                    &en_za,
                ))
        };

        #[cfg(not(feature = "i18n"))]
        let uncaught_error = format!("The following error was not caught: '{}'", error);

        // Always print error message to the console.
        println!("{}", uncaught_error.as_str());

        #[cfg(feature = "i18n")]
        let exit = {
            #[cfg(target_os = "macos")]
            {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert(
                    "short_name".to_string(),
                    PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
                );
                localisation
                    .localiser()
                    .format_with_defaults("application", "quit_macos", &values)
                    .unwrap_or(LString::new(
                        format!("Quit {}", APPLICATION_NAME_SHORT),
                        &en_za,
                    ))
            }

            #[cfg(not(target_os = "macos"))]
            localisation
                .localiser()
                .literal_with_defaults("word", "exit_i")
                .unwrap_or(LString::new("Exit", &en_za))
        };

        #[cfg(not(feature = "i18n"))]
        let exit = {
            #[cfg(target_os = "macos")]
            format!("Quit {}", APPLICATION_NAME_SHORT);

            #[cfg(not(target_os = "macos"))]
            "Exit".to_string()
        };

        FatalError {
            #[cfg(feature = "i18n")]
            script_data: ScriptData::new(localisation, &language_identifier),
            enabled: true,
            parent: None,
            title,
            uncaught_error,
            exit,
        }
    }
}

impl AnyWindowTrait for FatalError {}

impl WindowTrait for FatalError {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn title(&self) -> &LString {
        &self.title
    }

    fn view(&self, _id: &window::Id) -> Element<ApplicationMessage> {
        #[cfg(feature = "i18n")]
        let align_start = self.script_data.align_words_start;

        #[cfg(not(feature = "i18n"))]
        let align_start = Alignment::Start;

        #[allow(unused_mut)]
        let mut content: Vec<Element<ApplicationMessage>> = vec![
            // Message - scrollable
            scrollable(
                column![text(self.uncaught_error.as_str())]
                    .width(Length::Fill)
                    .align_items(align_start),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            " ".into(), // Paragraph separation
            // Exit button
            column![button(text(self.exit.as_str()))
                .padding([5, 10])
                .on_press(ApplicationMessage::Exit)]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into(),
        ];

        #[cfg(feature = "i18n")]
        if self.script_data.reverse_lines {
            content.reverse();
        }

        container(column(content).width(Length::Fill))
            .height(Length::Fill)
            .padding(2)
            .into()
    }

    fn parent(&self) -> &Option<WindowType> {
        &self.parent
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

// This is final window displayed, with all other windows being disabled.
pub fn display_fatal_error(
    application: &mut ApplicationThread,
    error: ApplicationError,
) -> Command<ApplicationMessage> {
    let size = application.session.settings.ui.fatal_error.size;
    let option = &application.session.settings.ui.fatal_error.position;
    let position = if option.is_none() {
        window::Position::Centered
    } else {
        let value = option.as_ref().unwrap();
        window::Position::Specific(Point {
            x: value.0,
            y: value.1,
        })
    };
    let settings = window::Settings {
        size: Size::new(size.0, size.1),
        resizable: RESIZABLE,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    let (id, spawn_window) = window::spawn(settings);
    let iterator = application.windows.iter_mut();
    for (_window_type, window) in iterator {
        window.disable();
    }

    #[cfg(feature = "i18n")]
    application.windows.insert(
        WindowType::FatalError,
        Box::new(FatalError::new(&application.localisation, error)),
    );

    #[cfg(not(feature = "i18n"))]
    application
        .windows
        .insert(WindowType::FatalError, Box::new(FatalError::new(error)));
    application.window_ids.insert(id, WindowType::FatalError);
    application.is_fatal_error = true;
    spawn_window
}
