// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{self, ApplicationError, constants::APPLICATION_NAME_SHORT, WindowType, StringGroup},
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, LocalisedTrait, WindowTrait},
    },
    localisation::information::{Index, Strings},
};
use i18n::utility::PlaceholderValue;
use iced::{
    widget::{button, column, scrollable, text},
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

pub struct State {
    information_type: InformationType,
    title: RefCount<String>,
    message: String,
}

impl State {
    pub fn try_information(
        localisation: &Localisation,
        title: String,
        message: String,
        strings: &Strings,
    ) -> Result<State, ApplicationError> {
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            values.insert(
                "type".to_string(),
                PlaceholderValue::String(strings.string(Index::Information as usize).to_string()),
            );
            values.insert("window".to_string(), PlaceholderValue::String(title));
            localisation.format_with_defaults("application", "window_type_title_format", &values)?
        }.0;
        Ok(State {
            information_type: InformationType::Information,
            title,
            message,
        })
    }

    pub fn try_warning(
        localisation: &Localisation,
        title: String,
        message: String,
        strings: &Strings,
    ) -> Result<State, ApplicationError> {
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            values.insert(
                "type".to_string(),
                PlaceholderValue::String(strings.string(Index::Warning as usize).to_string()),
            );
            values.insert("window".to_string(), PlaceholderValue::String(title));
            localisation.format_with_defaults("application", "window_type_title_format", &values)?
        }.0;
        Ok(State {
            information_type: InformationType::Warning,
            title,
            message,
        })
    }

    pub fn try_error(
        localisation: &Localisation,
        title: String,
        message: String,
        strings: &Strings,
    ) -> Result<State, ApplicationError> {
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            values.insert(
                "type".to_string(),
                PlaceholderValue::String(strings.string(Index::Warning as usize).to_string()),
            );
            values.insert("window".to_string(), PlaceholderValue::String(title));
            localisation.format_with_defaults("application", "window_type_title_format", &values)?
        }.0;
        Ok(State {
            information_type: InformationType::Error,
            title,
            message,
        })
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
        WindowType::Information
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
        let align_start = localisation.layout_data().align_words_start;
        let reverse_lines = localisation.layout_data().reverse_lines;
        let strings = string_cache.get(&StringGroup::Information).unwrap();

        #[allow(unused_mut)]
        let mut content: Vec<Element<application::Message>> = vec![
            // Message
            scrollable(
                column![text(self.message.as_str())]
                    .width(Length::Fill)
                    .align_x(align_start),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
            " ".into(), // Paragraph separation
            // Close button
            column![button(text(strings.string(Index::Close as usize)))
                .padding([5, 10])
                .on_press(application::Message::Close(id))]
            .width(Length::Fill)
            .align_x(Alignment::Center)
            .into(),
        ];
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

#[allow(dead_code)]
#[derive(PartialEq)]
pub enum InformationType {
    Error,
    Warning,
    Information, // General purpose
}

pub fn display(
    application: &mut application::State,
    title: String,
    message: String,
    information_type: InformationType,
    parent: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    if !application
        .string_cache
        .exists(&StringGroup::Information)
    {
        application.string_cache.insert(
            StringGroup::Information,
            Box::new(Strings::try_new(&application.localisation)?),
        );
    }
    let strings = application
        .string_cache
        .get(&StringGroup::Information)
        .unwrap();
    let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
    let state = match information_type {
        InformationType::Information => {
            State::try_information(&application.localisation, title, message, actual)
        }
        InformationType::Warning => {
            State::try_warning(&application.localisation, title, message, actual)
        }
        InformationType::Error => {
            State::try_error(&application.localisation, title, message, actual)
        }
    }?;
    Ok(application
        .manager
        .try_create_window(&mut application.session, Box::new(state), parent)?)
}
