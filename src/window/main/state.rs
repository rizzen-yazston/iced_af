// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, WindowType, StringGroup},
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, WindowTrait},
    },
    localisation,
    window::{about, fatal_error, main::menu_bar, preferences},
};
use iced::{
    widget::{column, text},
    window, Task, Element, Length,
};
use std::any::Any;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone)]
pub enum Message {
    MenuBar(menu_bar::Message),
    FatalError,
}

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
        WindowType::Main
    }

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String {
        let strings = string_cache.get(&StringGroup::Main).unwrap();
        strings.title()
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let reverse_lines = localisation.layout_data().reverse_lines;
        //let common = string_cache.get(&WindowType::MainCommon).unwrap();
        //let common_actual = common.as_any().downcast_ref::<main_common::Strings>().unwrap();
        //let strings = string_cache.get(&WindowType::Placeholder).unwrap();
        //let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
        let mut content: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();

        // Menubar
        content.push(
            menu_bar::view(id, string_cache).map(move |message: menu_bar::Message| {
                application::Message::Main(id, Message::MenuBar(message))
            }),
        );

        // Content
        content.push(text("Test message - Temporary.").into());

        // Temporary testing content
        // Uncomment to test FatalError window
        /*
        content.push(
            column![button(text(strings.string(Index::Exit as usize)))
            .padding([5, 10])
            .on_press(application::Message::Main(id, Message::FatalError))]
            .width(Length::Fill)
            .align_items(Alignment::Center)
            .into(),
        );
        */

        if reverse_lines {
            content.reverse();
        }
        column(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .into()
    }

    fn reusable(&self) -> bool {
        true
    }
}

pub fn display(
    application: &mut application::State,
) -> Result<Task<application::Message>, ApplicationError> {
    if !application.string_cache.exists(&StringGroup::Main) {
        application.string_cache.insert(
            StringGroup::Main,
            Box::new(
                localisation::main::Strings::try_new(&application.localisation)?
            ),
        );
    }
    if !application.string_cache.exists(&StringGroup::MainCommon) {
        application.string_cache.insert(
            StringGroup::MainCommon,
            Box::new(
                localisation::main_common::Strings::try_new(&application.localisation)?
            ),
        );
    }
    let state: Box<dyn AnyWindowTrait> = match application.manager.use_reusable(WindowType::Main) {
        None => Box::new(State::new()),
        Some(value) => value,
    };
    Ok(application.manager.try_spawn_new_thread(&mut application.session, state)?)
}

pub fn try_update(
    application: &mut application::State,
    message: application::Message,
) -> Result<Task<application::Message>, ApplicationError> {
    let mut tasks = Task::none();
    match message {
        application::Message::Main(id, main_message) => match main_message {
            Message::MenuBar(menubar_message) => match menubar_message {
                menu_bar::Message::None => {} // No action.
                menu_bar::Message::Open(
                    id,
                    window_type
                ) => tasks = application.open_thread(id, window_type)?,
                menu_bar::Message::Close(id) => tasks = application.close_thread(id)?,
                menu_bar::Message::Preferences => tasks = preferences::display(application, id)?,
                menu_bar::Message::About => tasks = about::display(application, id)?,
            },
            Message::FatalError => {
                tasks = fatal_error::display(application, ApplicationError::DatabaseAlreadyOpen)
            }
        },
        _ => {}
    }
    Ok(tasks)
}
