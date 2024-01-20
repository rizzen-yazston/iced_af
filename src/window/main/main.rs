// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::menu_bar::{ MainMenuBar, MainMenuBarMessage };
use crate::{
    core::{
        application::{
            WindowType,
            ApplicationMessage,
            ApplicationThread,
        },
        error::ApplicationError,
        localisation::Localisation,
        environment::Environment,
        traits::{ WindowTrait, AnyWindowTrait },
    },
    widget::event_control,
    window::{
        fatal_error::display_fatal_error,
        confirm_exit::display_confirm_exit,
        preferences::display_preferences,
    },
};
use i18n::{
    utility::TaggedString,
    pattern::PlaceholderValue,
};
use iced::{
    window,
    Command,
    Renderer,
    widget::{ column, container, Column, Row, text, button, },
    alignment,
    Element,
    Length,
    Size,
    Point,
};
use log::error;
use std::{
    any::Any,
    collections::HashMap,
};

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 500f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 1000f32, 600f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = true;

#[derive( Debug, Clone )]
pub enum MainMessage {
    MenuBar( MainMenuBarMessage ),
}

//impl WindowMessage for MainMessage {}

pub struct MainLocalisation {
    language: RefCount<String>,

    // Strings
    title: TaggedString,
}

impl MainLocalisation {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let language = localisation.localiser().default_language();
        let name = localisation.localiser().literal_with_defaults(
            "word", "main_i"
        )?;
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
        )?;
        Ok( MainLocalisation {
            language: RefCount::clone( &language ),
            title,
        } )
    }
}

pub struct Main {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: MainLocalisation,
    menu_bar: MainMenuBar,
}

impl Main {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        Ok( Main {
            enabled: true,
            parent: None,
            localisation: MainLocalisation::try_new( localisation, environment )?,
            menu_bar: MainMenuBar::try_new( localisation, environment, )?,
        } )
    }

    pub fn is_unsaved( &self ) -> bool {
        // Faking unsaved data for ConfirmExit demonstration.
        // Here logic would be done to determine whether there is unsaved data or not.
        true
    }
}

impl AnyWindowTrait for Main {}

impl WindowTrait for Main {
    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }

    fn title( &self ) -> &TaggedString {
        &self.localisation.title
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        let mut content = Column::new();
        content = content.push( self.menu_bar.view().map(
            |message: MainMenuBarMessage| ApplicationMessage::Main(
                MainMessage::MenuBar( message )
            )
        ) );
        let mut body = Column::new();
        body = body.push(
            text( "Basic example demonstrating multi-window mode of iced, with localisation of windows." )
        );
        content = content.push( body.width( Length::Fill ).height( Length::Fill ) );
        event_control::Container::new( content, self.enabled )
        .width( Length::Fill )
        .height( Length::Fill )
        .into()
    }

    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        self.menu_bar.try_update_localisation( localisation, environment )?;
        Ok( () )
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

pub fn display_main(
    application: &mut ApplicationThread,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if !application.windows.contains_key( &WindowType::Main ) {
        application.windows.insert(
            WindowType::Main,
            Box::new( Main::try_new( &application.localisation, &application.environment )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::Main ).unwrap();
        window.try_update_localisation( &application.localisation, &application.environment, )?;
    }
    let size = application.session.settings.ui.main.size;
    let option = &application.session.settings.ui.main.position;
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
    application.spawn( settings, &WindowType::Main )
}

pub fn update_main(
    application: &mut ApplicationThread,
    message: ApplicationMessage,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    let mut command = Command::none();
    match  message {
        ApplicationMessage::Main( main_message ) => match main_message {
            MainMessage::MenuBar( menubar_message ) => match menubar_message {
                MainMenuBarMessage::Preferences => command = display_preferences( application )?,
                MainMenuBarMessage::None => {},
                MainMenuBarMessage::Exit => {
                    let Some( window ) =
                    application.windows.get( &WindowType::Main ) else {
                        return Ok( display_fatal_error(
                            application, ApplicationError::WindowTypeNotFound( WindowType::Main )
                        ) );
                    };
                    let actual = window.as_any().downcast_ref::<Main>().unwrap();

                    if actual.is_unsaved()/*application.unsaved*/ {
                        command = display_confirm_exit(
                            application,
                            "Menu Quit pressed and there is unsaved data."
                        )?
                    } else {
                        error!( "Menu Quit pressed and there is no unsaved data." );
                        command = application.save_and_exit()
                    }
                },
            },
        },
        _ => {}
    }
    Ok( command )
}
