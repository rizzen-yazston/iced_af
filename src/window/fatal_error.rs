// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::core::{
    application::{
        WindowType,
        ApplicationMessage,
        ApplicationThread,
    },
    error::ApplicationError,
    localisation::Localisation,
    environment::Environment,
    traits::{ AnyWindowTrait, WindowTrait },
};
use i18n::{
    pattern::PlaceholderValue,
    utility::TaggedString
};
use iced::{
    window,
    Command,
    widget::{ button, column, container, text, scrollable },
    alignment,
    Alignment,
    Element,
    Length,
    Size,
    Point,
};
use std::{
    collections::HashMap,
    any::Any,
};

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 300f32, 150f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 500f32, 200f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct FatalError {
    enabled: bool,
    parent: Option<WindowType>,
    title: TaggedString,
    uncaught_error: TaggedString,
    exit: TaggedString,
}

impl FatalError {
    pub fn new(
        localisation: &Localisation,
        environment: &Environment,
        error: ApplicationError,
    ) -> Self {
        let language_registry = localisation.localiser().language_tag_registry();
        let en_za = language_registry.tag( "en-za" ).unwrap();
        let name = localisation.localiser().literal_with_defaults(
            "application", "fatal_error"
        ).unwrap_or( TaggedString::new( "Fatal error", &en_za ) );
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String( environment.application_short_name.clone() ),
        );
        values.insert(
            "window".to_string(), 
            PlaceholderValue::TaggedString( name )
        );
        let title = localisation.localiser().format_with_defaults(
            "application",
            "window_title_format",
            &values
        ).unwrap_or( TaggedString::new(
            format!( "{} - Fatal error", environment.application_short_name.clone() ),
            &en_za
        ) );
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "error".to_string(),
            PlaceholderValue::String( error.to_string() ),
        );
        let uncaught_error = localisation
        .localiser()
        .format_with_defaults( "application", "uncaught_error", &values )
        .unwrap_or( TaggedString::new(
            format!( "The following error was not caught: '{}'", error ),
            &en_za
        ) );
        println!( "{}", uncaught_error.as_str() ); // Always keep for console use.
        let exit = if cfg!( target_os = "macos" ) {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "short_name".to_string(),
                PlaceholderValue::String( environment.application_short_name.clone() ),
            );
            localisation
            .localiser()
            .format_with_defaults( "application", "quit_macos", &values )
            .unwrap_or( TaggedString::new(
                format!( "Quit {}", environment.application_short_name ),
                &en_za
            ) )
        } else {
            localisation.localiser().literal_with_defaults(
                "word", "exit_i"
            ).unwrap_or( TaggedString::new( "Exit", &en_za ) )
        };
        FatalError {
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
    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }

    fn title( &self ) -> &TaggedString {
        &self.title
    }

    fn view( &self, _id: &window::Id ) -> Element<ApplicationMessage> {
        let content = column![
            scrollable( text( self.uncaught_error.as_str() ) ),
            button( self.exit.as_str() )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Exit ),
        ].spacing( 10 ).align_items( Alignment::Center );
        container( content )
        .align_x( alignment::Horizontal::Center )
        .align_y( alignment::Vertical::Center )
        .width( Length::Fill )
        .height( Length::Fill )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn enable( &mut self ) {
        self.enabled = true;
    }

    fn disable( &mut self ) {
        self.enabled = false;
    }

    fn is_enabled( &self ) -> bool {
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
        window::Position::Specific( Point { x: value.0, y: value.1 } )
    };
    let settings = window::Settings {
        // Always centred, thus position not saved in session.
        size: Size::new( size.0, size.1 ),
        resizable: RESIZABLE,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    let ( id, spawn_window ) = window::spawn( settings );
    let mut iterator = application.windows.iter_mut();
    while let Some( ( _window_type, window ) ) = iterator.next() {
        window.disable();
    }
    application.windows.insert(
        WindowType::FatalError,
        Box::new( FatalError::new( &application.localisation, &application.environment, error ) )
    );
    application.window_ids.insert( id, WindowType::FatalError );
    application.is_fatal_error = true;
    spawn_window
}
