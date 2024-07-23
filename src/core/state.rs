// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The state manager of the mini application framework.
//!
//! Add new window states under `src/windows/` directory.

use crate::{
    application::{
        constants::WINDOW_DEFAULT_DATA, session::WindowData, Message,
        Session, WindowType,
    },
    core::{
        error::CoreError,
        traits::{AnyWindowTrait, WindowTrait},
    },
    window::{default, fatal_error},
};
use iced::{window, Point, Size, Task};
use std::collections::BTreeMap;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct Manager {
    // All active window states
    states: BTreeMap<
        window::Id,
        (
            Box<dyn AnyWindowTrait>, // The window state
            bool,                    // This window enabled
            Option<window::Id>,      // Parent
            Option<Vec<window::Id>>, // Windows disabled by this window::Id
        ),
    >,

    // Window IDs placed in main window threads
    threads: Vec<Option<Vec<window::Id>>>,

    // Reusable states cache
    reusable: BTreeMap<WindowType, Box<dyn AnyWindowTrait>>,

    // List of possible main window types. Similar concept of LibreOffice applications.
    main_types: Vec<WindowType>,

    // Default main window
    default_main: WindowType, // Empty reusable state, mainly just contains a menu bar.

    // New state to be inserted
    new_entry: Option<(
        Box<dyn AnyWindowTrait>, // The window state
        bool,                    // This window enabled
        Option<window::Id>,      // Parent
        Option<Vec<window::Id>>, // Windows disabled by this window::Id
    )>,
}

impl Manager {
    /// Initialise the state manager.
    ///
    /// * `main_types`: are the valid window types for main windows, including the placeholder
    /// window type.
    pub fn try_new(
        main_types: Vec<WindowType>,
    ) -> Result<Manager, CoreError> {
        let states = BTreeMap::<
            window::Id,
            (
                Box<dyn AnyWindowTrait>,
                bool,
                Option<window::Id>,
                Option<Vec<window::Id>>,
            ),
        >::new();
        let threads = Vec::<Option<Vec<window::Id>>>::new();
        let mut reusable = BTreeMap::<WindowType, Box<dyn AnyWindowTrait>>::new();
        reusable.insert(WindowType::Default, Box::new(default::State::new()));
        Ok(Manager {
            states,
            threads,
            reusable,
            main_types,
            default_main: WindowType::Default,
            new_entry: None,
        })
    }

    //
    // ----- Retrieve/information methods
    //

    /// Retrieve the state instance for the specified window Id.
    pub fn state(&self, id: &window::Id) -> Option<&Box<dyn AnyWindowTrait>> {
        self.states.get(id).map(|x| &x.0)
    }

    /// Retrieve the mutable state instance for the specified window Id.
    pub fn state_mut(&mut self, id: &window::Id) -> Option<&mut Box<dyn AnyWindowTrait>> {
        self.states.get_mut(id).map(|x| &mut x.0)
    }

    /// Return the number of main window threads.
    pub fn main_threads(&self) -> usize {
        let mut _count = 0;
        for option in self.threads.iter() {
            if option.is_some() {
                _count += 1;
            }
        }
        _count
    }

    /// Indicates whether the window is enabled for events.
    pub fn is_enabled(&self, id: &window::Id) -> Option<bool> {
        self.states.get(id).map(|x| x.1)
    }

    /// Return the parent window Id of the specified window Id if available.
    pub fn parent(&self, id: &window::Id) -> Option<window::Id> {
        self.states.get(id).map(|x| x.2)?
    }

    pub fn is_spawning(&self) -> bool {
        self.new_entry.is_some()
    }

    //
    // ----- Spawning methods
    //

    // If parent is `None`, then all window threads are disabled. Normally used by global windows.
    fn disable(&mut self, parent: &Option<window::Id>) -> Vec<window::Id> {
        match parent {
            None => {
                let mut disabled = Vec::<window::Id>::with_capacity(self.threads.len());
                for thread in &self.threads {
                    if thread.is_some() {
                        if let Some(id) = thread.as_ref().unwrap().last() {
                            let entry = self.states.get_mut(id).unwrap();
                            entry.1 = false;
                            disabled.push(*id);
                        }
                    }
                }
                disabled
            }
            Some(parent) => {
                let entry = self.states.get_mut(&parent).unwrap();
                entry.1 = false;
                vec![*parent]
            }
        }
    }

    /// Spawn a new main window thread, using the provided `iced` window settings, and the window state.
    pub fn try_spawn_new_thread(
        &mut self,
        session: &mut Session,
        state: Box<dyn AnyWindowTrait>,
    ) -> Result<Task<Message>, CoreError> {
        debug!(
            "Spawning main thread with window type ‘{:?}’",
            state.window_type()
        );
        let mut _found = false;
        for window_type in &self.main_types {
            if window_type == &state.window_type() {
                _found = true;
                break;
            }
        }
        if !_found {
            return Err(CoreError::InvalidWindowTypeMain(state.window_type()));
        }

        // Set `iced` window settings, and spawn
        let task = spawn(session, state.window_type())?;

        // Prepare to insert state once window is opened
        self.new_entry = Some((state, true, None, None));
        Ok(task.map(|id| Message::WindowOpened(id)))
    }

    /// Spawn a window on an existing main window thread, using the provided `iced` window settings, the
    /// window state, and parent window Id (used to locate the main thread to attached window to).
    pub fn try_spawn(
        &mut self,
        session: &mut Session,
        state: Box<dyn AnyWindowTrait>,
        parent: window::Id, // Typically be the calling window
    ) -> Result<Task<Message>, CoreError> {
        debug!("Spawning window type ‘{:?}’", state.window_type());
        if !self.states.contains_key(&parent) {
            return Err(CoreError::WindowIdNotFound(
                parent,
                "Manager.state".to_string(),
            ));
        }
        let parent = Some(parent);
        let disabled = if state.global_disable() {
            self.disable(&None)
        } else {
            self.disable(&parent)
        };

        // Set `iced` window settings, and spawn
        let id = spawn(session, state.window_type())?;

        // Prepare to insert state once window is opened
        self.new_entry = Some((state, true, parent, Some(disabled)));
        Ok(id.map(|id| Message::WindowOpened(id)))
    }

    /// Obtain reusable state if available, for the specified window type.
    pub fn use_reusable(&mut self, window_type: WindowType) -> Option<Box<dyn AnyWindowTrait>> {
        self.reusable.remove(&window_type)
    }

    /// Spawn the fatal error window.
    ///
    /// Every window is disable to prevent any additional fatal errors from occurring, before spawning
    /// the `FatalError` window.
    pub fn spawn_fatal_error(
        &mut self,
        session: &mut Session,
    ) -> Task<Message> {
        let state = Box::new(fatal_error::State::new());
        debug!("Spawning window type ‘{:?}’", state.window_type());
        let id = match spawn(session, state.window_type()) {
            Ok(value) => value,
            Err(_) => {
                let settings = window::Settings {
                    size: Size::new(500f32, 200f32),
                    resizable: false,
                    position: window::Position::Centered,
                    exit_on_close_request: false,
                    ..Default::default()
                };
                window::open(settings)
            }
        };
        let _ = self.disable(&None);

        // Prepare to insert state once window is opened
        self.new_entry = Some((state, true, None, None));
        id.map(|id| Message::WindowOpened(id))
    }

    pub fn window_opened(&mut self, id: window::Id) {
        let entry = self.new_entry.take().unwrap();
        if entry.2.is_some() {
            for thread in &mut self.threads {
                if thread.is_some() {
                    let actual = thread.as_mut().unwrap();
                    if actual.last() == entry.2.as_ref() {
                        actual.push(id);
                        break;
                    }
                }
            }
        } else {
            match self.threads.iter().position(|x| x.is_none()) {
                None => self.threads.push(Some(vec![id])),
                Some(index) => self.threads[index] = Some(vec![id]),
            }
        }
        self.states.insert(id, entry);
    }

    pub fn fatal_error_opened(&mut self, id: window::Id) {
        self.states.insert(id, self.new_entry.take().unwrap());
    }

    //
    // ----- Closing methods
    //

    /// Close a single window.
    pub fn close(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, CoreError> {
        let task = window::close(id);
        Ok(task.chain(Task::done(Message::WindowClosed(id))))
    }

    /// Allows for multiple windows to be closed at once.
    /// 
    /// Ensure the vector is ordered from newest to oldest window, else fatal error may occur.
    pub fn close_multiple(
        &mut self,
        ids: Vec<window::Id>,
    ) -> Result<Task<Message>, CoreError> {
        let mut tasks = Task::none();
        for id in ids {
            tasks = tasks.chain(window::close(id));
            tasks = tasks.chain(Task::done(Message::WindowClosed(id)));
        }
        Ok(tasks)
    }

    /// Close an entire main window thread.
    ///
    /// This method is only called once checks for unsaved data is done
    pub fn close_thread(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, CoreError> {
        debug!("Closing main thread {:?}", id);
        if self.states.get(&id).is_none() {
            return Err(CoreError::WindowIdNotFound(
                id,
                "Manager.states".to_string(),
            ));
        };

        // Find the thread to close.
        let index = self
            .threads
            .iter()
            .position(|thread| {
                if thread.is_some() {
                    let actual = thread.as_ref().unwrap();
                    if actual.first() == Some(&id) {
                        return true;
                    }
                }
                false
            })
            .unwrap();

        // Close the thread.
        let mut thread = self.threads[index].as_ref().unwrap().clone();
        thread.reverse();
        let mut tasks = Task::none();
        for state_id in thread {
            tasks = tasks.chain(window::close(state_id));
            tasks = tasks.chain(Task::done(Message::WindowClosed(state_id)));
        }
        self.threads[index] = None;
        trace!("{:?}", self.threads);
        Ok(tasks)
    }
    
    /// Close a single window and re-enable windows that was disabled by this window.
    pub fn window_closed(
        &mut self,
        id: window::Id,
    ) -> Result<(), CoreError> {
        debug!("Window closed {:?}, now removing state.", id);

        // Re-enable disabled windows by this window.
        let mut _disabled: Option<Vec<window::Id>> = None;
        {
            let Some(entry) = self.states.get_mut(&id) else {
                return Err(CoreError::WindowIdNotFound(
                    id,
                    "Manager.states".to_string(),
                ));
            };
            _disabled = entry.3.take();
        }
        if _disabled.is_some() {
            let disabled = _disabled.unwrap();
            trace!("Disabled windows: {:?}", disabled);
            for disabled_id in disabled {
                let disabled_state = self.states.get_mut(&disabled_id).unwrap();
                disabled_state.1 = true;
            }
        }

        // Remove window ID from the window tread
        for thread in &mut self.threads {
            if thread.is_some() {
                let actual = thread.as_mut().unwrap();
                if actual.last() == Some(&id) {
                    _ = actual.pop();
                    break;
                }
            }
        }

        // Remove the state
        let entry = self.states.remove(&id).unwrap();
        if entry.0.reusable() {
            self.reusable.insert(entry.0.window_type(), entry.0);
        }
        //Ok(window::close(id))
        Ok(())
    }
}

//
// ----- Private functions
//

// Simply creates the required `iced` windows Settings, and spawn the window.
fn spawn(
    session: &mut Session,
    window_type: WindowType,
) -> Result<Task<window::Id>, CoreError> {
    let Some(defaults) = WINDOW_DEFAULT_DATA.get(&window_type.as_str()) else {
        return Err(CoreError::WindowTypeNotFound(
            window_type,
            "WINDOW_DEFAULT_DATA".to_string(),
        ));
    };
    if !session.windows.contains_key(&window_type) {
        session.windows.insert(
            window_type.clone(),
            WindowData {
                size: defaults.size.clone(),
                position: None,
            },
        );
    }
    let data = session.windows.get(&window_type).unwrap();
    let position = if data.position.is_none() {
        window::Position::Centered
    } else {
        let value = data.position.as_ref().unwrap();
        window::Position::Specific(Point {
            x: value.0,
            y: value.1,
        })
    };
    let settings = window::Settings {
        size: Size::new(data.size.0, data.size.1),
        resizable: defaults.resizable,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    Ok(window::open(settings))
}
