// This file is part of `iced_af` crate. For the terms of use, please see the file
// called `LICENSE-BSD-3-Clause` at the top level of the `iced_af` crate.

#![allow(dead_code)]

pub mod application;
pub mod core;
pub mod localisation;
pub mod widget;
pub mod window;

// This is only used when there is a cargo cache issue as a result of using the `iced`
// master branch with the `iced_aw` main branch. It is simply a copy of the `iced_aw`
// components used by this project.
#[cfg(feature = "iced_aw")]
pub mod iced_aw; 
