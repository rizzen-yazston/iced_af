// This file is part of `iced_af` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `iced_af` crate.

#![allow( dead_code )]

pub mod core;
pub mod widget;
pub mod window;

// Edit the following constants to reflect the project:

pub(crate) const VENDOR: &str = "rizzen"; // Always lower case ASCII letter [a-z].
pub(crate) const APPLICATION_NAME: &str = "iced Application Framework";
pub(crate) const APPLICATION_NAME_SHORT: &str = "iced_af";
pub(crate) const AUTHORS: &str = env!( "CARGO_PKG_AUTHORS" ); // Change if more detailed this is required.

// These constants require no editing, mainly derived from the build environment:

pub(crate) const PACKAGE_NAME: &str = env!( "CARGO_PKG_NAME" );
pub(crate) const VERSION: &str = env!( "CARGO_PKG_VERSION" );
