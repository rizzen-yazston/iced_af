// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! This is generally the default window that is displayed when there are no file opened,
//! connected databases, etc. Though for simple applications, where the data is not in
//! various formats requiring different windows for editing the data.
//! 
//! At a minimal, the default window should have these components, such as opening a file,
//! connect to database, exiting the application, editing preferences, read the about.
//! A small window can be used for this purpose. Typically containing a minimal menu bar.
//! 
//! The [`Manager`](crate::core::state::Manager) is initialised with this window state.

pub mod state;
pub use state::*;
pub mod menu_bar;
