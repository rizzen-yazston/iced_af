// This file is part of `iced_af` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `iced_af` crate.

//! The application files, where plenty editing takes place to add application features.

#![allow(dead_code)]

pub mod application;
pub use application::{Message, State};
//pub use application::{ApplicationMessage, ApplicationThread, StartUp};
pub mod environment;
pub use environment::Environment;
pub mod error;
pub use error::ApplicationError;
pub mod session;
pub use session::Session;
pub mod constants;
pub mod enums;
pub use enums::{WindowType, StringGroup};
//pub mod data;
pub mod clap;
pub mod log;
