// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, WindowType, StringGroup},
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, WindowTrait},
    },
    localisation::fatal_error::{Index, Strings},
};
use iced::{
    widget::{button, column, scrollable, text},
    window, Alignment, Task, Element, Length,
};
use std::any::Any;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

pub struct State {}

impl State {
    pub fn new() -> Self {
        State {}
    }
}

impl AnyWindowTrait for State {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }
}

impl WindowTrait for State {
    fn window_type(&self) -> WindowType {
        WindowType::FatalError
    }

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String {
        let strings = string_cache.get(&StringGroup::FatalError).unwrap();
        strings.title()
    }

    fn view<'a>(
        &'a self,
        _id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let strings = string_cache.get(&StringGroup::FatalError).unwrap();
        let align_start = localisation.layout_data().align_words_start;

        #[allow(unused_mut)]
        let mut content: Vec<Element<application::Message>> = vec![
            // Message - scrollable
            scrollable(
                column![text(strings.string(Index::UncaughtError as usize).as_str())]
                    .width(Length::Fill)
                    .align_x(align_start),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            " ".into(), // Paragraph separation
            // Exit button
            column![button(text(strings.string(Index::Exit as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::Terminate)]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into(),
        ];
        if localisation.layout_data().reverse_lines {
            content.reverse();
        }
        column(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .into()
    }
}

// This is final window displayed, with all other windows being disabled.
pub fn display(
    application: &mut application::State,
    error: ApplicationError,
) -> Task<application::Message> {
    if !application
        .string_cache
        .exists(&StringGroup::FatalError)
    {
        application.string_cache.insert(
            StringGroup::FatalError,
            Box::new(Strings::new(&application.localisation, error)),
        );
    }
    application.manager.create_fatal_error_window(&mut application.session)
}
