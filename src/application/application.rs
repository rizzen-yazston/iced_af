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
        unsaved_data,
    },
};
use clap::Parser;
use core::panic;
use iced::{
    event::{self, Event}, window, Element, Length, Point, Size, Subscription, Task,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

//
// ----- The application supported messages
//

/// The application's `Message`s. Window specific messages are grouped in their
/// own `Message` enum, and is an entry of the application `Message` enum.
#[derive(Debug, Clone)]
pub enum Message {
    // Window events
    CloseRequested(window::Id),
    Resized(window::Id, Size),
    Moved(window::Id, Point),

    // Generic application messages
    Initialise, // Continue with initialising once application state instance exists.
    WindowOpened(window::Id), // Indicates window has opened. Optional action be done
    WindowClosed(window::Id), // Remove the state of closed window Id.
    ThreadClosed(usize), // Remove the thread, now that windows are closed.
    Exit,  // Save settings and exit.
    Terminate,         // Terminates application without any saving.
    Close(window::Id), // Generic window close, nothing else is done.

    // Application window specific messages
    UnsavedData(window::Id, unsaved_data::Message),
    Default(window::Id, default::Message),
    Main(window::Id, main::Message),
    Preferences(window::Id, preferences::Message),
}

//
// ----- The application event processing loop
//

/// The application state is a special state, as it does not have any UI
/// window, that is the application state is headless. The primary purpose of
/// this state is to contain the global environment (global data of the
/// application), the UI localisation, the main message handler (handles global
/// messages, delegates messages to other window states to update their
/// states), and the view handler (requesting specific window to be redrawn).
pub struct State {
    // Indicates that the state is fully initialised.
    initialised: bool,

    // Data that can be persistent.
    pub session: Session,

    // Data that can't be persistent
    pub environment: Environment,

    // The localisation system for the UI.
    pub localisation: Localisation,

    // The shared localised strings, rather static except when there is language changes.
    pub string_cache: StringCache,

    // The state manager containing all the window states.
    pub manager: Manager,

    // Indicates if application is running for the first time.
    first_use: bool,
}

impl State {
    /// Entry point for initialising the application state.
    pub fn new() -> (State, Task<Message>) {
        match State::try_new() {
            Err(error) => panic!("Application initialisation error: {}", error),
            Ok(value) => value,
        }
    }

    /// The actual implementation of creating the state.
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
        let string_cache = StringCache::new();
        debug!("Localisation initialised.");
        let manager = Manager::try_new()?;
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

    /// Indicates if the application has started for the first time
    pub fn first_use(&self) -> bool {
        self.first_use
    }

    //
    // ------ Update methods
    //

    /// To capture the `iced` window events.
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

    /// The entry point for the `iced` update functionality.
    pub fn update(&mut self, message: Message) -> Task<Message> {
        match self.try_update(message) {
            Err(error) => fatal_error::display(self, error),
            Ok(value) => value,
        }
    }

    /// The actual implementation of updating the state.
    fn try_update(
        &mut self,
        message: Message,
    ) -> Result<Task<Message>, ApplicationError> {
        let mut tasks = Task::none();
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
                            tasks = self.close_thread(id)?
                        }
                        WindowType::Main => {
                            debug!("Main window's decoration button was pressed.");
                            tasks = main::try_to_close(self, id)?
                        }
                        WindowType::FatalError => tasks = iced::exit(), // Session is not saved.
                        WindowType::Preferences => tasks = preferences::cancel_and_close(self, id)?,

                        // Generic window close
                        _ => tasks = self.manager.close_window(id)?,
                    }
                }
            }
            Message::Resized(id, size) => tasks = self.resized(&id, size)?,
            Message::Moved(id, point) => tasks = self.moved(&id, point)?,

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
                        tasks = self.manager.try_create_thread(&mut self.session, Box::new(state))?;
                        debug!("Opening Preferences window.");
                    } else {
                        tasks = default::display(self)?;
                        debug!("Opening Default window.");
                    }
                    self.initialised = true;
                }
                println!("Initialise has completed."); // Keep both these line
                info!("Initialise has completed."); // Keep both these line
            },
            Message::WindowOpened(_id) => {} // Post actions can be place here for the opened window (Id provided)
            Message::WindowClosed(id) => self.manager.window_closed(id)?,
            Message::ThreadClosed(id) => {
                self.manager.thread_closed(id)?;
                if self.manager.thread_count() == 0 {
                    tasks = default::display(self)?;
                }
            }
            Message::Exit => tasks = self.exit(),
            Message::Terminate => tasks = iced::exit(),
            Message::Close(id) => tasks = self.manager.close_window(id)?,
            Message::UnsavedData(_, _) => tasks = unsaved_data::try_update(self, message)?,

            // Application window specific messages
            Message::Default(_, _) => tasks = default::try_update(self, message)?,
            Message::Main(_, _) => tasks = main::try_update(self, message)?,
            Message::Preferences(_, _) => tasks = preferences::try_update(self, message)?,
        }
        Ok(tasks)
    }

    //
    // ----- Window geometry methods
    //

    /// Window was resized.
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

    /// Window was moved.
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

    /// Get the title string for the window.
    pub fn title(&self, id: window::Id) -> String {
        let window = self.manager.state(&id).expect(format!("title(): Failed to get state for window id {:?}", id).as_str());
        window.title(&self.string_cache).to_string()
    }

    /// The entry point for the `iced` view functionality.
    pub fn view(&self, id: window::Id) -> Element<Message> {
        let state = self.manager.state(&id).expect(format!("view(): Failed to get state for window id {:?}", id).as_str());
        let content = state.view(id, &self.localisation, &self.string_cache);
        event_control::Container::new(content, self.manager.is_enabled(&id).unwrap())
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(0)
            .into()
    }

    //
    // ----- Window opening methods
    //

    /// Opens a new main window thread, for the specified window type.
    pub fn open_thread(
        &mut self,
        window_type: WindowType,
    ) -> Result<Task<Message>, ApplicationError> {
        debug!("Opening. Threads: {:?}", self.manager.thread_count());
        let tasks = if self.manager.thread_count() > 1 {
            match window_type {
                // If there are additional main window types, add them.
                WindowType::Main => {
                    trace!("open_thread: try to display Main");
                    let (tasks, _) = main::display(self)?;
                    tasks
                }
                _ => {
                    trace!("open_thread: error: window type must be Main");
                    Task::none()
                }
            }
        } else {
            // Only 1 thread, which means current window can be Default window,
            // which would need to be closed on successful opening of a main
            // window.
            let id = self.manager.thread_list().pop().unwrap();
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
                    	// If there are additional main window types, add them.
                        WindowType::Main => {
                            trace!("open_thread: try to display Main");
                            let (mut tasks, success) = main::display(self)?;
                            if success {
                                // Have new Main window, close the Default window
                                tasks = tasks.chain(self.manager.close_thread(id)?);
                            }
                            tasks
                        }
                        _ => Task::none(),
                    }
                }
                _ => {
                    trace!("Not default window.");
                    match window_type {
	                    // If there are additional main window types, add them.
                        WindowType::Main => {
                            let (tasks, _) = main::display(self)?;
                            tasks
                        }
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

    /// Close a thread, and if the last thread and not default, then spawn default, else exit.
    pub fn close_thread(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, ApplicationError> {
        debug!("Closing. Threads: {:?}", self.manager.thread_count());
        #[allow(unused_assignments)]
        let mut tasks = Task::none();
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
                    self.manager.close_thread(id)?
                }
            };
        Ok(tasks)
    }

    /// Save the session, and terminate the application.
    ///
    /// Note: Unsaved data is not saved.
    pub fn exit(
        &mut self,
    ) -> Task<Message> {
        let _ = self.session.save();
        iced::exit()
    }

    /// Attempt to close all threads.
    ///
    /// Any window that has unsaved data will produce a dialogue for that window.
    pub fn close_all(
        &mut self,
    ) -> Result<Task<Message>, ApplicationError> {
        let mut tasks = Task::none();
        for id in self.manager.thread_list() {
            let Some(state) = self.manager.state(&id) else {
                return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
            };
            match state.window_type() {
                WindowType::Main => tasks = tasks.chain(main::try_to_close(self, id)?),
                _ => return Err(CoreError::InvalidWindowTypeMain(state.window_type()))?,
            }
        }
        Ok(tasks)
    }
}
