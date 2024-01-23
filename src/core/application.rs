// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    error::ApplicationError,
    environment::Environment,
    clap::Clap,
    localisation::Localisation,
    session::{ Session, Generic, },
    traits::AnyWindowTrait,
};
use crate::{
    widget::event_control::Container,
    window::{ // Windows of the application.
        confirm_exit::{ self, display_confirm_exit, },
        fatal_error::{ self, display_fatal_error, },
        preferences::{ self, update_preferences, PreferencesMessage, Preferences, },
        information,
        main::{ self, update_main, Main, MainMessage },
        about,
    },
};
use iced::{
    multi_window::Application,
    executor,
    window as window_iced,
    event::{ self, Event },
    widget::{ column, text },
    Command,
    Element,
    Subscription,
    Theme,
};
use std::collections::HashMap;
use core::panic;

#[cfg( feature = "log" )]
use log4rs::Handle as LoggerHandler;

#[cfg( feature = "log" )]
use log::{ /*debug,*/ error, info };

#[derive( Debug, Clone, Eq, PartialEq, Hash )]
pub enum WindowType {
    Main,
    ConfirmExit,
    FatalError,
    Information,
    Preferences,
    About,
}

#[derive( Debug, Clone )]
pub enum ApplicationMessage {
    EventOccurred( Event ),
    Exit, // Save settings and exit.
    Close( window_iced::Id ), // Generic window close, nothing else is done.

    // Windows
    Main( MainMessage ),
    Preferences( PreferencesMessage ),
}

// Just a container for transferring initialisation instances, that can't be done within ApplicationThread::new().
#[derive( Clone )]
pub struct StartUp {
    pub clap: Clap,

    #[cfg( feature = "log" )]
    pub logger: LoggerHandler,

    pub session: Session,
    pub first_time: bool,
}

pub struct ApplicationThread {
    first_time: bool, // Indicates if application is running for the first time.
    pub session: Session, // Includes application settings.
    pub environment: Environment,
    pub localisation: Localisation,
    pub is_fatal_error: bool,
    pub window_ids: HashMap<window_iced::Id, WindowType>,
    pub windows: HashMap<WindowType, Box<dyn AnyWindowTrait>>,
}

impl ApplicationThread {
    fn try_new( flags: StartUp ) -> Result<( ApplicationThread, Command<ApplicationMessage> ), ApplicationError> {
        let environment = Environment::try_new( &flags )?;
        let localisation = Localisation::try_new(
            &environment,
            &flags.session.settings.ui.language,
        )?;
        println!( "Localisation initialised." );// Keep this line
        let mut window_ids = HashMap::<window_iced::Id, WindowType>::new();
        let mut windows =
        HashMap::<WindowType, Box<dyn AnyWindowTrait>>::new();
        let mut session = flags.session;
        if flags.first_time {
            session.settings.ui.preferences = Generic { size: preferences::SIZE_DEFAULT, position: None };
            session.settings.ui.main = Generic { size: main::SIZE_DEFAULT, position: None };
            session.settings.ui.confirm_exit = Generic { size: confirm_exit::SIZE_DEFAULT, position: None };
            session.settings.ui.fatal_error = Generic { size: fatal_error::SIZE_DEFAULT, position: None };
            session.settings.ui.information = Generic { size: information::SIZE_DEFAULT, position: None };
            session.settings.ui.about = Generic { size: about::SIZE_DEFAULT, position: None };
            window_ids.insert( window_iced::Id::MAIN, WindowType::Preferences );
            println!( "Creating Preferences window." );
            windows.insert(
                WindowType::Preferences,
                Box::new( Preferences::try_new(
                    &localisation, &session.settings, flags.first_time, &environment
                )? )
            );
        } else {
            window_ids.insert( window_iced::Id::MAIN, WindowType::Main );
            println!( "Creating Main window." );
            windows.insert(
                WindowType::Main,
                Box::new( Main::try_new( &localisation, &environment )? )
            );
        }
        Ok( ( ApplicationThread {
            first_time: flags.first_time,
            session,
            environment,
            localisation,
            is_fatal_error: false,
            window_ids,
            windows,
        }, Command::none() ) )
    }

    pub fn first_time( &self ) -> bool {
        self.first_time
    }

    fn try_update(
        &mut self,
        message: <ApplicationThread as Application>::Message
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let mut command = Command::none();
        match message {
            ApplicationMessage::EventOccurred( event ) => {
                match event {
                    Event::Window( id, event ) => {
                        match event {
                            window_iced::Event::CloseRequested => {
                                let Some( window_type ) = self.window_ids.get( &id ) else {
                                    return Ok( display_fatal_error(
                                        self, ApplicationError::WindowIdNotFound( id )
                                    ) );
                                };
                                match window_type {
                                    WindowType::Main => {
                                        let Some( window ) =
                                        self.windows.get( &WindowType::Main ) else {
                                            return Ok( display_fatal_error(
                                                self,
                                                ApplicationError::WindowTypeNotFound( window_type.clone() )
                                            ) );
                                        };
                                        if window.is_enabled() {
                                            if self.is_fatal_error {

                                                #[cfg( feature = "log" )]
                                                info!( "Window decoration button was pressed on fatal window, save \
                                                session and exit." );

                                                command = self.save_and_exit();
                                            } else if self.is_unsaved() {
                                                command = display_confirm_exit(
                                                    self,
                                                    "Window decoration button was pressed and there is \
                                                    unsaved data."
                                                )?;
                                            } else {

                                                #[cfg( feature = "log" )]
                                                info!( "Window decoration button was pressed and there is no unsaved \
                                                data." );

                                                command = self.save_and_exit();
                                            }
                                        }
                                    },
                                    WindowType::FatalError => {
                                        command = self.save_and_exit();
                                    },
                                    WindowType::Preferences => command = preferences::close( self, id )?,
                                    //WindowType::Connecting => {}, // Disable close button
                                    _ => { // Generic window close
                                        command = self.close( id )?
                                    }
                                }
                            },
                            window_iced::Event::Resized { width, height } =>
                            command = self.resized( &id, width, height )?,
                            window_iced::Event::Moved { x, y } => 
                            command = self.moved( &id, x, y )?,
                            _ => {} // Ignore window other events
                        }
                    },
                    _ => {} // Ignore other events.
                }
            },
            ApplicationMessage::Exit => command = self.save_and_exit(),
            ApplicationMessage::Close( id ) => command = self.close( id )?,
            ApplicationMessage::Main( _ ) =>
            command = update_main( self, message )?,
            ApplicationMessage::Preferences( _ ) => 
            command = update_preferences( self, message )?,
        }
        Ok( command )
    }

    pub fn close(
        &mut self,
        id: window_iced::Id
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let mut _parent_option: Option<WindowType> = None;
        {
            let Some( window_type ) = self.window_ids.get( &id ) else {
                return Ok( display_fatal_error( self, ApplicationError::WindowIdNotFound( id ) ) );
            };
            let Some( window ) =
            self.windows.get_mut( window_type ) else {
                return Ok( display_fatal_error(
                    self, ApplicationError::WindowTypeNotFound( window_type.clone() )
                ) );
            };
            _parent_option = window.parent_remove();
            if _parent_option.is_none() {
                return Ok( display_fatal_error(
                    self, ApplicationError::ExpectedWindowParent( window_type.clone() )
                ) );
            }
        }
        let parent = _parent_option.unwrap();
        let Some( parent_window ) =
        self.windows.get_mut( &parent ) else {
            return Ok( display_fatal_error(
                self, ApplicationError::WindowTypeNotFound( parent )
            ) );
        };
        parent_window.enable();
        self.window_ids.remove( &id );
        Ok( window_iced::close( id ) )
    }

    pub fn save_and_exit( &mut self ) -> Command<<ApplicationThread as Application>::Message> {
        let window = self.windows.get( &WindowType::Main );
        if window.is_some() { // Could be not present, on first time use.
            if self.is_unsaved() {
                //flush database to file
            }
            let _ = self.session.save();
        }
        let mut commands =
        Vec::<Command<<ApplicationThread as Application>::Message>>::new();
        let mut iterator = self.window_ids.iter();
        while let Some( ( id, _window ) ) = iterator.next() {
            commands.push( window_iced::close( *id ) );
        }
        Command::batch( commands )
    }

    pub fn spawn(
        &mut self,
        settings: window_iced::Settings,
        window_type: &WindowType,
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let ( id, spawn_window ) = window_iced::spawn( settings );
        self.window_ids.insert( id, window_type.clone() );
        Ok( spawn_window )
    }

    pub fn spawn_with_disable(
        &mut self,
        settings: window_iced::Settings,
        window_type: &WindowType,
        parent: &WindowType,
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let Some( parent_window ) =
        self.windows.get_mut( parent ) else {
            return Ok( display_fatal_error(
                self, ApplicationError::WindowTypeNotFound( parent.clone() )
            ) );
        };
        parent_window.disable();
        let ( id, spawn_window ) = window_iced::spawn( settings );
        self.window_ids.insert( id, window_type.clone() );
        Ok( spawn_window )
    }

    fn resized(
        &mut self,
        id: &window_iced::Id,
        width: u32,
        height: u32,
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let Some( window_type ) = self.window_ids.get( &id ) else {
            return Ok( display_fatal_error(
                self, ApplicationError::WindowIdNotFound( *id )
            ) );
        };
        match window_type {
            WindowType::ConfirmExit => self.session.settings.ui.confirm_exit.size = (
                width as f32, height as f32
            ),
            WindowType::FatalError => self.session.settings.ui.fatal_error.size = (
                width as f32, height as f32
            ),
            WindowType::Preferences => self.session.settings.ui.preferences.size = (
                width as f32, height as f32
            ),
            WindowType::Information => self.session.settings.ui.information.size = (
                width as f32, height as f32
            ),
            WindowType::Main => self.session.settings.ui.main.size = (
                width as f32, height as f32
            ),
            WindowType::About => self.session.settings.ui.about.size = (
                width as f32, height as f32
            ),
        }
        Ok( Command::none() )
    }

    fn moved(
        &mut self,
        id: &window_iced::Id,
        x: i32,
        y: i32,
    ) -> Result<Command<<ApplicationThread as Application>::Message>, ApplicationError> {
        let Some( window_type ) = self.window_ids.get( &id ) else {
            return Ok( display_fatal_error(
                self, ApplicationError::WindowIdNotFound( *id )
            ) );
        };
        match window_type {
            WindowType::ConfirmExit => self.session.settings.ui.confirm_exit.position =
            Some( ( x as f32, y as f32 ) ),
            WindowType::FatalError => self.session.settings.ui.fatal_error.position =
            Some( ( x as f32, y as f32 ) ),
            WindowType::Preferences => self.session.settings.ui.preferences.position =
            Some( ( x as f32, y as f32 ) ),
            WindowType::Information => self.session.settings.ui.information.position =
            Some( ( x as f32, y as f32 ) ),
            WindowType::Main => self.session.settings.ui.main.position =
            Some( ( x as f32, y as f32 ) ),
            WindowType::About => self.session.settings.ui.about.position =
            Some( ( x as f32, y as f32 ) ),
        }
        Ok( Command::none() )
    }

    fn is_unsaved( &self ) -> bool {
        let window = self.windows.get( &WindowType::Main );
        if window.is_none() {
            return false; // No Main window, thus can't have no unsaved data.
        }
        let actual = window.unwrap().as_any().downcast_ref::<Main>().unwrap();
        actual.is_unsaved()
    }
}

impl Application for ApplicationThread {
    type Executor = executor::Default;
    type Flags = StartUp;
    type Message = ApplicationMessage;
    type Theme = Theme;

    fn new( flags: StartUp ) -> ( ApplicationThread, Command<Self::Message> ) {
        match ApplicationThread::try_new( flags ) {
            Err( error ) => panic!( "Instance initialisation error: {}", error ),
            Ok( value ) => value,
        }
    }

    fn title( &self, window_id: window_iced::Id ) -> String {

        // These errors should never occur, unless forgotten to add create window and type instance.
        let Some( window_type ) = self.window_ids.get( &window_id ) else {
            let message = format!( "Failed to get WindowType for window id: {:?}", window_id );

            #[cfg( feature = "log" )]
            error!( "{}", message );

            return message;
        };
        let Some( window ) = self.windows.get( window_type ) else {
            let message = format!( "Failed to get model for WindowType {:?}", window_type );

            #[cfg( feature = "log" )]
            error!( "{}", message );

            return message;
        };

        window.title().to_string()
    }

    fn subscription( &self ) -> Subscription<ApplicationMessage> {
        event::listen().map( ApplicationMessage::EventOccurred )
    }

    fn update( &mut self, message: Self::Message ) -> Command<Self::Message> {
        match self.try_update( message ) {
            Err( error ) => display_fatal_error( self, error ),
            Ok( value ) => value
        }
    }

    fn view( &self, window_id: window_iced::Id ) -> Element<Self::Message> {

        // These errors should never occur, unless forgotten to add create window and type instance.
        let Some( window_type ) = self.window_ids.get( &window_id ) else {
            let message = format!( "Failed to get WindowType for window id: {:?}", window_id );

            #[cfg( feature = "log" )]
            error!( "{}", message );

            let column = column![ text( message.as_str() ), ];
            return Container::new( column, false ).into();
        };
        let Some( window ) = self.windows.get( window_type ) else {
            let message = format!( "Failed to get model for WindowType {:?}", window_type );

            #[cfg( feature = "log" )]
            error!( "{}", message );

            let column = column![ text( message.as_str() ), ];
            return Container::new( column, false ).into();
        };

        let content = window.view( &window_id );
        content
    }
}
