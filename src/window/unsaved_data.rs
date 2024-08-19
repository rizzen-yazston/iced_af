// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, constants::APPLICATION_NAME_SHORT, StringGroup, WindowType},
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, SaveDataTrait, WindowTrait},
    },
    localisation::unsaved_data::{Index, Strings},
    window::main,
};
use i18n::utility::PlaceholderValue;
use iced::{
    widget::{button, column, row, text},
    window, Alignment, Task, Element, Length,
};
use std::{
    any::Any,
    collections::HashMap,
};

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Save,
    Discard,
    Cancel,
}

pub struct State {
    title: RefCount<String>,
    message: RefCount<String>,
}

impl State {
    pub fn try_new(
        localisation: &Localisation,
        name: &str,
    ) -> Result<State, ApplicationError> {
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            let localised = localisation.literal_with_defaults("application", "unsaved_data")?;
            values.insert(
                "window".to_string(),
                PlaceholderValue::Localised(localised.0, localised.1),
            );
            values.insert(
                "name".to_string(),
                PlaceholderValue::String(name.to_string()),
            );
            localisation.format_with_defaults("application", "window_title_name_format", &values)?
        }.0;
        let message = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "name".to_string(),
                PlaceholderValue::String(name.to_string()),
            );
            localisation.format_with_defaults("application", "unsaved_data_statement", &values)?
        }.0;
        Ok(State {title, message})
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
        WindowType::UnsavedData
    }

    fn title<'a>(&'a self, _string_cache: &'a StringCache) -> &String {
        &self.title
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let reverse_words = localisation.layout_data().reverse_words;
        let reverse_lines = localisation.layout_data().reverse_lines;
        let strings = string_cache.get(&StringGroup::UnsavedData).unwrap();
        let mut content: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();

        // Message
        content.push(
            column![text(self.message.as_str())]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
        );
        content.push(text(" ").height(Length::Fill).into()); // Paragraph separation

        // Buttons
        let mut buttons: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();
        buttons.push(
            button(text(strings.string(Index::Save as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::UnsavedData(id, Message::Save))
                .into(),
        );
        buttons.push(
            button(text(strings.string(Index::Discard as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::UnsavedData(id, Message::Discard))
                .into(),
        );
        buttons.push(
            button(text(strings.string(Index::Cancel as usize).as_str()))
                .padding([5, 10])
                .on_press(application::Message::UnsavedData(id, Message::Cancel))
                .into(),
        );
        if reverse_words {
            buttons.reverse();
        }
        content.push(
            column![row(buttons).spacing(10)]
                .width(Length::Fill)
                .align_x(Alignment::Center)
                .into(),
        );
        if reverse_lines {
            content.reverse();
        }
        column(content)
            .width(Length::Fill)
            .height(Length::Fill)
            .padding(2)
            .into()
    }
}

pub fn display(
    application: &mut application::State,
    parent: window::Id,
    name: &str, // Reason for &str instead of &PathBuff, is to also support database connections
) -> Result<Task<application::Message>, ApplicationError> {
    if !application
        .string_cache
        .exists(&StringGroup::UnsavedData)
    {
        application.string_cache.insert(
            StringGroup::UnsavedData,
            Box::new(Strings::try_new(&application.localisation)?),
        );
    }
    let state = State::try_new(&application.localisation, name)?;
    Ok(application
        .manager
        .try_spawn(&mut application.session, Box::new(state), parent)?)
}

pub fn try_update(
    application: &mut application::State,
    message: application::Message,
) -> Result<Task<application::Message>, ApplicationError> {
    let mut tasks = Task::none();
    match message {
        application::Message::UnsavedData(id, ref inner_message) => {
            match inner_message {
                Message::Cancel => tasks = application.manager.close(id)?,
                Message::Discard => tasks = application.close_thread(id)?,
                Message::Save => {
                    let Some(parent) = application.manager.parent(&id) else {
                        return Err(CoreError::ExpectedWindowParent(WindowType::UnsavedData))?;
                    };
                    let Some(&mut ref mut state) = application.manager.state_mut(&parent) else {
                        return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
                    };
                    match state.window_type() {
                        WindowType::Main => {
                            let actual = state.as_any_mut().downcast_mut::<main::State>().unwrap();
                            let _ = actual.try_save();
                        }
                        _ => {}
                    }
                    tasks = application.close_thread(id)?;
                }
            }
        }
        _ => {}
    }
    Ok(tasks)
}
