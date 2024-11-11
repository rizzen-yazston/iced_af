// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The state manager of the multi-window application.
//!
//! Add new window states under the `src/windows/` directory.

use crate::{
    application::{
        constants::WINDOW_DEFAULT_DATA, session::WindowData, ApplicationError,
        Message, Session, WindowType
    },
    core::{
        error::CoreError,
        traits::{AnyWindowTrait, WindowTrait},
    },
    window::{default, fatal_error},
};
use iced::{window, Point, Size, Task};
use std::{collections::BTreeMap, usize};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

/// The manager is responsible for creating new windows, and removing windows.
/// Keeping the window state and window Id in sync, and the windows are in a
/// safe hierarchy, allowing lower windows to be disabled, or all other windows
/// disabled except the active window (useful when changes may affects all
/// opened files, databases, etc). Also included is a cache for re-usable
/// states.
pub struct Manager {
    // All active window states
    states: BTreeMap<window::Id, Entry>,

    // Window IDs placed in main window threads
    threads: VecOption<Vec<window::Id>>,

    // Reusable states cache
    reusable: BTreeMap<WindowType, Box<dyn AnyWindowTrait>>,
}

impl Manager {
    /// Initialise the state manager.
    /// 
    /// The reusable Default window state is added to the cache.
    /// 
    /// The return `Result` is used instead of `Manager` instance, just in case
    /// the default window creation produces an error.
    pub fn try_new() -> Result<Manager, ApplicationError> {
        let mut reusable = BTreeMap::<WindowType, Box<dyn AnyWindowTrait>>::new();
        reusable.insert(WindowType::Default, Box::new(default::State::new()));
        Ok(Manager {
            states: BTreeMap::<window::Id, Entry>::new(),
            threads: VecOption::<Vec<window::Id>>::new(),
            reusable,
        })
    }

    //
    // ----- Retrieve/information methods
    //

    /// Retrieve a reference to the state instance for the specified window
    /// Id.
    pub fn state(&self, id: &window::Id) -> Option<&Box<dyn AnyWindowTrait>> {
        self.states.get(id).map(|x| &x.state)
    }

    /// Retrieve a mutable reference to the mutable state instance for the
    /// specified window Id.
    pub fn state_mut(&mut self, id: &window::Id) -> Option<&mut Box<dyn AnyWindowTrait>> {
        self.states.get_mut(id).map(|x| &mut x.state)
    }

    /// Return the number of window threads.
    pub fn thread_count(&self) -> usize {
        self.threads.count
    }

    /// Return a list of the threads' root window Id.
    pub fn thread_list(&self) -> Vec<window::Id> {
        let mut _vec = Vec::<window::Id>::new();
        for option in self.threads.vec.iter() {
            if option.is_some() {
                _vec.push(option.as_ref().unwrap()[0].clone());
            }
        }
        _vec
    }

    /// Indicates whether the window is enabled for events.
    pub fn is_enabled(&self, id: &window::Id) -> Option<bool> {
        self.states.get(id).map(|x| x.enabled)
    }

    /// Return the parent window Id of the specified window Id if available.
    pub fn parent(&self, id: &window::Id) -> Option<window::Id> {
        self.states.get(id).map(|x| x.parent)?
    }

    //
    // ----- Spawning methods
    //

    /// Disable the parent window of a thread, or all threads by passing
    /// `None`. Disabling all threads is usually done for window that
    /// contains settings, that can affect all threads.
    fn disable_windows(&mut self, parent: &Option<window::Id>) -> Vec<window::Id> {
        match parent {
            None => {
                trace!("disable(): all threads");
                let mut disabled = Vec::<window::Id>::with_capacity(self.threads.vec.len());
                for thread in &self.threads.vec {
                    if thread.is_some() {
                        if let Some(id) = thread.as_ref().unwrap().last() {
                            let entry = self.states.get_mut(id).unwrap();
                            entry.enabled = false;
                            disabled.push(*id);
                        }
                    }
                }
                disabled
            }
            Some(parent) => {
                trace!("disable(): for parent {:?}", parent);
                let entry = self.states.get_mut(&parent).unwrap();
                entry.enabled = false;
                vec![*parent]
            }
        }
    }

    /// Try to create a new window thread, using the provided `iced` window
    /// settings located in the application's session data, and the window
    /// state.
    pub fn try_create_thread(
        &mut self,
        session: &mut Session,
        state: Box<dyn AnyWindowTrait>,
    ) -> Result<Task<Message>, CoreError> {
        debug!(
            "try_create_thread(): for window type ‘{:?}’",
            state.window_type()
        );

        // Set `iced` window settings, and spawn.
        let id = try_create(session, state.window_type())?;

        // Insert state and open the window.
        let entry = Entry {state, enabled: true, parent: None, disabled: None};
        let _ = self.threads.push(vec![id.0]);
        self.states.insert(id.0, entry);
        trace!("try_create_thread(): inserted state for {:?}, next open window", id.0);
        Ok(id.1.map(move |id| Message::WindowOpened(id)))
    }

    /// Try to create a window on an existing main window thread, using the
    /// provided `iced` window settings located in the application's session
    /// data, the window state, and parent window Id (used to locate the main
    /// thread to attached window to).
    pub fn try_create_window(
        &mut self,
        session: &mut Session,
        state: Box<dyn AnyWindowTrait>,
        parent: window::Id, // Typically be the calling window
    ) -> Result<Task<Message>, CoreError> {
        debug!(
            "try_create_window(): for window type ‘{:?}’",
            state.window_type()
        );
        if !self.states.contains_key(&parent) {
            return Err(CoreError::WindowIdNotFound(
                parent,
                "Manager.state".to_string(),
            ));
        }
        let parent = Some(parent);
        let disabled = if state.is_global_disable() {
            self.disable_windows(&None)
        } else {
            self.disable_windows(&parent)
        };

        // Set `iced` window settings, and spawn
        let id = try_create(session, state.window_type())?;

        // Insert state and open the window.
        let entry = Entry {state, enabled: true, parent, disabled: Some(disabled)};
        for thread in &mut self.threads.vec {
            if thread.is_some() {
                let actual = thread.as_mut().unwrap();
                if actual.last() == entry.parent.as_ref() {
                    actual.push(id.0);
                    break;
                }
            }
        }
        self.states.insert(id.0, entry);
        trace!("try_create_window(): inserted state for {:?}, next open window", id.0);
        Ok(id.1.map(move |id| Message::WindowOpened(id)))
    }

    /// Obtain reusable state if available, for the specified window type.
    pub fn use_reusable(&mut self, window_type: WindowType) -> Option<Box<dyn AnyWindowTrait>> {
        self.reusable.remove(&window_type)
    }

    /// Create the fatal error window.
    ///
    /// Every window is disable to prevent any additional fatal errors from occurring, before
    /// spawning the `FatalError` window.
    pub fn create_fatal_error_window(
        &mut self,
        session: &mut Session,
    ) -> Task<Message> {
        let state = Box::new(fatal_error::State::new());
        debug!(
            "create_fatal_window(): for window type ‘{:?}’",
            state.window_type()
        );
        let id = match try_create(session, state.window_type()) {
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
        let _ = self.disable_windows(&None);

        // Insert state and open the window.
        let entry = Entry {state, enabled: true, parent: None, disabled: None};
        for thread in &mut self.threads.vec {
            if thread.is_some() {
                let actual = thread.as_mut().unwrap();
                if actual.last() == entry.parent.as_ref() {
                    actual.push(id.0);
                    break;
                }
            }
        }
        self.states.insert(id.0, entry);
        trace!("create_fatal_error_window(): inserted state for {:?}, next open window", id.0);
        id.1.map(move |id| Message::WindowOpened(id))
    }

    //
    // ----- Closing methods
    //

    /// Close a single window.
    pub fn close_window(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, CoreError> {
        trace!("close_window(): id {:?}", id);
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
        debug!("close_multiple()");
        let mut tasks = Task::none();
        for id in ids {
            tasks = tasks.chain(window::close(id));
            tasks = tasks.chain(Task::done(Message::WindowClosed(id)));
        }
        Ok(tasks)
    }

    /// Close an entire main window thread, using any window Id in the thread.
    ///
    /// This method is only called once checks for unsaved data is done
    pub fn close_thread(
        &mut self,
        id: window::Id,
    ) -> Result<Task<Message>, CoreError> {
        debug!("close_thread(): contains {:?}", id);
        if self.states.get(&id).is_none() {
            return Err(CoreError::WindowIdNotFound(id, "Manager.states".to_string()));
        };

        // Find the thread to close.
        let index = self.threads.vec.iter().position(|thread| {
            if thread.is_some() {
                let actual = thread.as_ref().unwrap();
                for id_ in actual {
                    if id_ == &id {
                        return true;
                    }
                }
            }
            false
        })
        .unwrap();

        // Close the thread.
        let mut thread = self.threads.vec[index].as_ref().unwrap().clone();
        thread.reverse();
        let mut tasks = Task::none();
        for state_id in thread {
            tasks = tasks.chain(window::close(state_id));
        }
        tasks = tasks.chain(Task::done(Message::ThreadClosed(index)));
        trace!("{:?}", self.threads.vec);
        Ok(tasks)
    }
    
    /// Re-enable windows that was disabled by this window, and remove the state.
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
            _disabled = entry.disabled.take();
        }
        if _disabled.is_some() {
            let disabled = _disabled.unwrap();
            trace!("Disabled windows: {:?}", disabled);
            for disabled_id in disabled {
                let disabled_state = self.states.get_mut(&disabled_id).unwrap();
                disabled_state.enabled = true;
            }
        }

        // Remove window ID from the window thread
        for thread in &mut self.threads.vec {
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
        if entry.state.is_reusable() {
            debug!("window_closed(): cached reusable state for {:?}", id);
            self.reusable.insert(entry.state.window_type(), entry.state);
        }
        trace!("window_closed(): removed state for {:?}", id);
        Ok(())
    }
    
    /// Remove the thread from threads, now that all windows has been closed.
    pub fn thread_closed(
        &mut self,
        index: usize,
    ) -> Result<(), CoreError> {
        trace!("thread_closed(): removed thread {:?}", index);
        let _ = self.threads.take(index); 
        Ok(())
    }
}

//
// ----- Private functions
//

/// Try to create the `iced` window for the specified window type, using the
/// `iced` windows Settings located in the application's session data.
fn try_create(
    session: &mut Session,
    window_type: WindowType,
) -> Result<(window::Id, Task<window::Id>), CoreError> {
    let Some(defaults) = WINDOW_DEFAULT_DATA.get(&window_type.as_str()) else {
        return Err(CoreError::WindowTypeNotFound(
            window_type,
            "WINDOW_DEFAULT_DATA".to_string(),
        ));
    };
    trace!("try_create(): WindowType: {:?}; defaults: {:?}", window_type, defaults);
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

#[derive(Debug)]
struct VecOption<T> {
    count: usize,
    vec: Vec<Option<T>>
}

impl<T> VecOption<T> {
    fn new() -> Self {
        VecOption::<T> {
            count: 0,
            vec: Vec::<Option<T>>::new(),
        }
    }

    fn push(&mut self, element: T) -> usize {
        let mut len = self.vec.len();
        trace!("push(): start: len: {}, count: {}", len, self.count);
        if self.count < len {
            if let Some(index) = self.vec.iter().position(|x| x.is_none()) {
                self.vec[index] = Some(element);
                len = index;
            }
        } else {
            self.vec.push(Some(element));
        }
        self.count += 1;
        trace!("push(): end: len: {}, count: {}, index: {}", self.vec.len(), self.count, len);
        len
    }

    fn pop(&mut self) -> Option<T> {
        trace!("pop(): start: len: {}, count: {}", self.vec.len(), self.count);
        if self.count == 0 {
            return None;
        }
        let element = self.vec.pop().unwrap();
        if element.is_some() {
            self.count -= 1;
            if self.count == 0 {
                self.vec.clear();
            }
        }
        trace!("pop(): end: len: {}, count: {}", self.vec.len(), self.count);
        element
    }

    fn replace(&mut self, index: usize, element: Option<T>) -> Option<T> {
        trace!("replace(): start: len: {}, count: {}, index: {}", self.vec.len(), self.count, index);
        let old = self.vec[index].take();
        self.vec[index] = element;
        trace!("replace(): end: len: {}, count: {}", self.vec.len(), self.count);
        old
    }

    fn take(&mut self, index: usize) -> Option<T> {
        trace!("take(): start: len: {}, count: {}, index: {}", self.vec.len(), self.count, index);
        if self.count == 0 {
            return None;
        }
        let element = self.vec[index].take();
        if element.is_some() {
            self.count -= 1;
            if self.count == 0 {
                self.vec.clear();
            }
        }
        trace!("take(): end: len: {}, count: {}", self.vec.len(), self.count);
        element
    }
}

/// Just a simple struct with named fields.
struct Entry {
    state: Box<dyn AnyWindowTrait>,    // The window state
    enabled: bool,                     // This window enabled
    parent: Option<window::Id>,        // Parent
    disabled: Option<Vec<window::Id>>, // Windows disabled by this window::Id
}
