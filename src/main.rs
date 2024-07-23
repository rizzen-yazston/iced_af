// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

//! The binary entry point.

use iced::{
    daemon,
    settings::Settings,
    Pixels,
};
use iced_af::application::State;

fn main() -> iced::Result {
    daemon(State::title, State::update, State::view)
    .subscription(State::subscription)
    .settings(Settings {
        default_text_size: Pixels(12.0),
        ..Default::default()
    })
    .run_with(State::new)
}
