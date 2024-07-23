// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{
        clap::Clap,
        error::ApplicationError,
        environment::Environment,
        log::{new_logger, update_logger, LogLevel,},
        session::Session,
        StringGroup,
        WindowType,
    },
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
        state::Manager,
    },
    localisation,
    widget::event_control,
    window::{
        default,
        confirm_exit,
        fatal_error,
        main,
        preferences,
    },
};
use clap::Parser;
use core::panic;
use iced::{
    event::{self, Event}, widget::Row, window, Element, Length, Point, Size, Subscription, Task,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

//
// ----- The application supported messages
//

#[derive(Debug, Clone)]
pub enum Message {
    // Window events
    CloseRequested(window::Id),
    Resized(window::Id, Size),
    Moved(window::Id, Point),

    // Generic application messages
    Initialise, // Continue with initialising once application state instance exists.
    WindowOpened(window::Id), // Prepare to insert state.
    WindowClosed(window::Id), // Remove the state.
    FatalErrorOpened(window::Id), // Prepare to spawn fatal window.
    Exit,  // Save settings and exit.
    Terminate,         // Terminates application without any saving.
    Close(window::Id), // Generic window close, nothing else is done.

    // Application window specific messages
    Default(window::Id, default::Message),
    Main(window::Id, main::Message),
    Preferences(window::Id, preferences::Message),
}

//
// ----- The application event processing loop
//

pub struct State {
    // Indicates that the state is fully initialised.
    initialised: bool,

    // Data that can be persistent
    pub session: Session,

    // Data that can't be persistent
    pub environment: Environment,

    // The localisation system for the UI.
    pub localisation: Localisation,

    // The shared localised strings, static except for language change
    pub string_cache: StringCache,

    // The state manager
    pub manager: Manager,

    // Indicates if application is running for the first time
    first_use: bool,
}

impl State {
    /// Entry point fpr initialising the application state.
    pub fn new() -> (State, Task<Message>) {
        match State::try_new() {
            Err(error) => panic!("Application initialisation error: {}", error),
            Ok(value) => value,
        }
    }

    fn try_new() -> Result<(State, Task<Message>), ApplicationError> {
        // Use clap for task line options. See clap.rs for various task options.
        let clap = Clap::parse();

        // Initialise logging to console.
        // For now just log to stdout.
        let log_level = match clap.log_level {
            None => LogLevel::Default,
            Some(value) => value,
        };
        let mut logger = new_logger(log_level);

        // Initialise the session, if available from previous saved session.
        #[allow(unused_mut)]
        let mut session = Session::default();
        let mut first_use = false;
        if !clap.defaults {
            info!("Using saved settings.");
            match Session::try_restore() {
                Err(_error) => {
                    warn!("Restore state error: `{:?}`", _error);
                    first_use = true
                }
                Ok(value) => session = value,
            }
        }

        // Update logger to all the log categories
        if clap.log_level.is_none() {
            update_logger(&mut logger, &session.settings.log_levels);
        }

        let environment = Environment::try_new(logger, clap)?;
        let localisation =
            Localisation::try_new(&environment, &session.settings.ui.language)?;
        let string_cache = StringCache::try_new()?;
        debug!("Localisation initialised.");
        let mut valid = vec![
            WindowType::Default,
            WindowType::Main,
            // Add other main window types here
        ];
        if first_use {
            valid.push(WindowType::Preferences);
        }
        let manager = Manager::try_new(
            valid, 
        )?;
        debug!("State manager initialised.");
        Ok((
            State {
                initialised: false,
                session,
                environment,
                localisation,
                string_cache,
                manager,
                first_use,
            },
            Task::done(Message::Initialise),
        ))
    }

    pub fn first_use(&self) -> bool {
        self.first_use
    }

    //
    // ------ Update methods
    //

    pub fn subscription(&self) -> Subscription<Message> {
        event::listen_with(
            |event, _status, id| {
                match event {
                    Event::Window(event) => match event {
                        window::Event::CloseRequested => Some(Message::CloseRequested(id)),
                        window::Event::Resized(size) => Some(Message::Resized(id, size)),
                        window::Event::Moved(point) => Some(Message::Moved(id, point)),
                        _ => None
                    }
                    _ => None
                }
            }
        )
    }

    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self.try_update(message) {
            Err(error) => fatal_error::display(self, error),
            Ok(value) => value,
        }
    }

    fn try_update(
        &mut self,
        message: Message,
    ) -> Result<Task<Message>, ApplicationError> {
        let mut task = Task::none();
        match message {
            // Window events
            Message::CloseRequested(id) => {
                let Some(state) = self.manager.state(&id) else {
                    return Ok(fatal_error::display(
                        self,
                        ApplicationError::Core(CoreError::WindowIdNotFound(
                            id,
                            "window_states".to_string(),
                        )),
                    ));
                };
                if self.manager.is_enabled(&id).unwrap() {
                    match state.window_type() {
                        WindowType::Default => {
                            debug!("Default window's decoration button was pressed.");
                            task = self.close_thread(id)?
                        }
                        WindowType::Main => {
                            debug!("Main window's decoration button was pressed.");
                            task = self.close_thread(id)?
                        }
                        WindowType::FatalError => task = iced::exit(),
                        WindowType::Preferences => task = preferences::cancel_and_close(self, id)?,

                        // Generic window close
                        _ => task = self.manager.close(id)?,
                    }
                }
            }
            Message::Resized(id, size) => task = self.resized(&id, size)?,
            Message::Moved(id, point) => task = self.moved(&id, point)?,

            // Generic application messages
            Message::Initialise => {
                debug!("Reached 2nd part of initialise.");
                // Display the window, now that application state exists.
                if !self.initialised {
                    if self.first_use {
                        trace!("First use");
                        if !self.string_cache.exists(&StringGroup::Preferences) {
                            self.string_cache.insert(
                                StringGroup::Preferences,
                                Box::new(
                                    localisation::preferences::Strings::try_new(
                                        &self.localisation
                                    )?
                                ),
                            );
                        }
                        let state = preferences::State::try_new(
                            &self.localisation,
                            &self.string_cache,
                            &self.session.settings,
                            true,
                        )?;
                        task = self.manager.try_spawn_new_thread(&mut self.session, Box::new(state))?;
                        debug!("Spawned Preferences window.");
                    } else {
                        task = default::display(self)?;
                        debug!("Spawned Default window.");
                    }
                    self.initialised = true;
                }
                println!("Initialise has completed."); // Keep both these line
                info!("Initialise has completed."); // Keep both these line
            },
            Message::WindowOpened(id) => self.manager.window_opened(id),
            Message::WindowClosed(id) => self.manager.window_closed(id)?,
            Message::FatalErrorOpened(id) => self.manager.fatal_error_opened(id),
            Message::Exit => task = self.exit(),
            Message::Terminate => task = iced::exit(),
            Message::Close(id) => task = self.manager.close(id)?,

            // Application window specific messages
            Message::Default(_, _) => task = default::try_update(self, message)?,
            Message::Main(_, _) => task = main::try_update(self, message)?,
            Message::Preferences(_, _) => task = preferences::try_update(self, message)?,
        }
        Ok(task)
    }

    //
    // ----- Window geometry methods
    //

    fn resized(
        &mut self,
        id: &window::Id,
        size: Size,
    ) -> Result<Task<Message>, CoreError> {
        let Some(state) = self.manager.state(&id) else {
            return Err(CoreError::WindowIdNotFound(
                *id,
                "Manager.states".to_string(),
            ));
        };
        let Some(data) = self.session.windows.get_mut(&state.window_type()) else {
            return Err(CoreError::WindowTypeNotFound(
                state.window_type(),
                "session.windows".to_string(),
            ));
        };
        data.size = (size.width, size.height);
        Ok(Task::none())
    }

    fn moved(
        &mut self,
        id: &window::Id,
        position: Point,
    ) -> Result<Task<Message>, CoreError> {
        let Some(state) = self.manager.state(&id) else {
            return Err(CoreError::WindowIdNotFound(
                *id,
                "window_states".to_string(),
            ));
        };
        let Some(data) = self.session.windows.get_mut(&state.window_type()) else {
            return Err(CoreError::WindowTypeNotFound(
                state.window_type(),
                "session.windows".to_string(),
            ));
        };
        data.position = Some((position.x, position.y));
        Ok(Task::none())
    }

    //
    // ------ Viewing methods
    //

    pub fn title(&self, id: window::Id) -> String {
        if !self.manager.is_spawning() {
            let window = self.manager.state(&id).expect(format!("title(): Failed to get state for window id {:?}", id).as_str());
            window.title(&self.string_cache).to_string()
        } else {
            String::new()
        }
    }

    pub fn view(&self, id: window::Id) -> Element<Message> {
        if !self.manager.is_spawning() {
            let state = self.manager.state(&id).expect(format!("view(): Failed to get state for window id {:?}", id).as_str());
            let content = state.view(id, &self.localisation, &self.string_cache);
            event_control::Container::new(content, self.manager.is_enabled(&id).unwrap())
                .width(Length::Fill)
                .height(Length::Fill)
                .padding(0)
                .into()
        } else {
            event_control::Container::new(Element::new(Row::new()), false)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
        }
    }

    //
    // ----- Window opening methods
    //

    // Opens a new main window thread, for the specified window type.
    pub fn open_thread(
        &mut self,
        id: window::Id,
        window_type: WindowType,
    ) -> Result<Task<Message>, ApplicationError> {
        debug!("Opening. Threads: {:?}", self.manager.main_threads());
        let tasks = if self.manager.main_threads() > 1 {
            match window_type {
                WindowType::Main => main::display(self)?,
                _ => Task::none(),
            }
        } else {
            let Some(state) = self.manager.state(&id) else {
                return Err(ApplicationError::Core(CoreError::WindowIdNotFound(
                    id,
                    "Manager.states".to_string(),
                )));
            };
            match state.window_type() {
                WindowType::Default => {
                    trace!("Default window.");
                    match window_type {
                        WindowType::Main => {
                            let tasks = main::display(self)?;
                            tasks.chain(self.manager.close_thread(id)?)
                        }
                        _ => Task::none(),
                    }
                }
                _ => {
                    trace!("Not default window.");
                    match window_type {
                        WindowType::Main => main::display(self)?,
                        _ => Task::none(),
                    }
                }
            }
        };
        Ok(tasks)
    }

    //
    // ----- Window closing methods
    //

    // Close a thread, and if the last thread and not default, then spawn default, else exit.
    pub fn close_thread(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, ApplicationError> {
        debug!("Closing. Threads: {:?}", self.manager.main_threads());
        #[allow(unused_assignments)]
        let mut tasks = Task::none();
        if self.manager.main_threads() > 1 {
            tasks = self.manager.close_thread(id)?;
        } else {
            let Some(state) = self.manager.state(&id) else {
                return Err(ApplicationError::Core(CoreError::WindowIdNotFound(
                    id,
                    "Manager.states".to_string(),
                )));
            };
            tasks = match state.window_type() {
                WindowType::Default => {
                    trace!("Default window.");
                    confirm_exit::display(self, id)?
                }
                _ => {
                    trace!("Not default window.");
                    tasks = self.manager.close_thread(id)?;
                    tasks.chain(default::display(self)?)
                }
            };
        }
        Ok(tasks)
    }

    // Save the session, and terminate the application.
    pub fn exit(
        &mut self,
    ) -> Task<Message> {
        let _ = self.session.save();
        iced::exit()
    }
}
