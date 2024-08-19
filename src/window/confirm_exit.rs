// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, WindowType, StringGroup},
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, WindowTrait},
    },
    localisation::confirm_exit::{Index, Strings},
};
use iced::{
    widget::{button, column, row, text},
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
        WindowType::ConfirmExit
    }

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String {
        let strings = string_cache.get(&StringGroup::ConfirmExit).unwrap();
        strings.title()
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let strings = string_cache.get(&StringGroup::ConfirmExit).unwrap();
        let mut content: Vec<Element<application::Message>> = vec![
            // Message
            column![text(strings.string(Index::ConfirmExit as usize).as_str())]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
            text(" ").height(Length::Fill).into(), // Paragraph separation
        ];

        // Buttons
        #[allow(unused_mut)]
        let mut buttons: Vec<Element<application::Message>> = vec![
            button(text(strings.string(Index::Exit as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::Exit)
                .into(),
            button(text(strings.string(Index::Cancel as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::Close(id))
                .into(),
        ];
        if localisation.layout_data().reverse_words {
            buttons.reverse();
        }
        content.push(
            column![row(buttons).spacing(10)]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
        );
        if localisation.layout_data().reverse_lines {
            content.reverse();
        }
        column(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(20)
            .into()
    }

    fn is_reusable(&self) -> bool {
        true
    }

    fn is_global_disable(&self) -> bool {
        true
    }
}

pub fn display(
    application: &mut application::State,
    parent: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    if !application
        .string_cache
        .exists(&StringGroup::ConfirmExit)
    {
        application.string_cache.insert(
            StringGroup::ConfirmExit,
            Box::new(Strings::try_new(&application.localisation)?),
        );
    }
    let state: Box<dyn AnyWindowTrait> = match application.manager.use_reusable(WindowType::ConfirmExit) {
        None => Box::new(State::new()),
        Some(value) => value,
    };
    Ok(application
        .manager
        .try_spawn(&mut application.session, state, parent)?)
}
