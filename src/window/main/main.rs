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
        traits::{ WindowTrait, AnyWindowTrait },
    },
    widget::event_control,
    window::{
        about::display_about,
        confirm_exit::display_confirm_exit,
        fatal_error::display_fatal_error,
        preferences::display_preferences
    },
    APPLICATION_NAME_SHORT,
};
use iced::{
    window,
    Command,
    Renderer,
    widget::{ column, container, Column, Row, text, button, },
    Alignment,
    Element,
    Length,
    Size,
    Point,
};
use std::{
    any::Any,
    cmp::Ordering,
    collections::HashMap,
    path::{ Path, PathBuf, },
};

#[cfg( feature = "i18n" )]
use crate::core::{
    localisation::{
        Localisation,
        ScriptData,
    },
    environment::Environment,
};

#[cfg( feature = "i18n" )]
use i18n::utility::{ TaggedString as LString, PlaceholderValue, };

#[cfg( not( feature = "i18n" ) )]
use std::string::String as LString;

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

#[cfg( all( feature = "i18n", feature = "sync" ) )]
use std::sync::Arc as RefCount;

#[cfg( all( feature = "i18n", not( feature = "sync" ) ) )]
use std::rc::Rc as RefCount;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 500f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 1000f32, 600f32 );
pub const RESIZABLE: bool = false;
//const MAXIMISE: bool = true;

#[derive( Debug, Clone )]
pub enum MainMessage {
    MenuBar( MainMenuBarMessage ),
}

//impl WindowMessage for MainMessage {}

pub struct MainLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    title: LString,
}

impl MainLocalisation {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        #[cfg( feature = "i18n" )]
        let language = localisation.localiser().default_language();

        #[cfg( feature = "i18n" )]
        let locale = localisation.localiser().language_tag_registry().locale(
            language.as_str()
        )?;

        #[cfg( feature = "i18n" )]
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString(
                    localisation.localiser().literal_with_defaults(
                        "word", "main_i",
                    )?
                )
            );
            localisation.localiser().format_with_defaults(
                "application", "window_title_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let title = format!( "{} - Main", APPLICATION_NAME_SHORT );

        Ok( MainLocalisation {
            #[cfg( feature = "i18n" )] language,
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
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
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        Ok( Main {
            enabled: true,
            parent: None,
            localisation: MainLocalisation::try_new(
                #[cfg( feature = "i18n" )] localisation,
            )?,
            menu_bar: MainMenuBar::try_new(
                #[cfg( feature = "i18n" )] localisation,
            )?,
        } )
    }

    pub fn is_unsaved( &self ) -> bool {
        // Replace with actual logic to detect unsaved data
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

    fn title( &self ) -> &LString {
        &self.localisation.title
    }

    fn view( &self, _id: &window::Id ) -> Element<ApplicationMessage> {
        #[cfg( feature = "i18n" )]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg( not( feature = "i18n" ) )]
        let align_start = Alignment::Start;

        let mut content: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        // Menubar
        content.push( self.menu_bar.view().map(
            |message: MainMenuBarMessage| ApplicationMessage::Main(
                MainMessage::MenuBar( message )
            )
        ) );

        // Content
        content.push(
            text( "Just a mini application framework to create applications." ).into() //TODO: Add to localisation database.
        );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        event_control::Container::new(
            column( content ).width( Length::Fill ),
            self.enabled
        ).height( Length::Fill ).padding( 2 )
        .into()
    }

    #[cfg( feature = "i18n" )]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg( feature = "log" )]
            info!( "Updating localisation." );

            self.localisation = MainLocalisation::try_new( localisation, )?;
        }
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
            Box::new( Main::try_new(
                #[cfg( feature = "i18n" )] &application.localisation,
            )? )
        );
    } else {
        #[cfg( feature = "i18n" )]
        {
            let window = application.windows.get_mut( &WindowType::Main ).unwrap();
            window.try_update_localisation( &application.localisation, &application.environment, )?;
        }
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
    let ( id, spawn_window ) = iced::window::spawn( settings );
    application.window_ids.insert( id, WindowType::Main );
    Ok( spawn_window )
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
                    if actual.is_unsaved() {
                        command = display_confirm_exit(
                            application,
                            #[cfg( feature = "log" )] "Menu Quit pressed and there is unsaved data."
                        )?
                    } else {
                        #[cfg( feature = "log" )]
                        error!( "Menu Quit pressed and there is no unsaved data." );

                        command = application.save_and_exit()
                    }
                },
                MainMenuBarMessage::About => command = display_about( application )?,
            },
        },
        _ => {}
    }
    Ok( command )
}
