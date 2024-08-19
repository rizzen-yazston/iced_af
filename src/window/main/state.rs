// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.


use crate::{
    application::{self, ApplicationError, constants::APPLICATION_NAME_SHORT, StringGroup, WindowType},
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, SaveDataTrait, WindowTrait},
    },
    localisation,
    window::{about, main::menu_bar, preferences, unsaved_data},
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
use chrono::prelude::*;

#[cfg(not(feature = "sync"))]
use std::rc::Rc as RefCount;

#[cfg(feature = "sync")]
#[cfg(target_has_atomic = "ptr")]
use std::sync::Arc as RefCount;

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone)]
pub enum Message {
    MenuBar(menu_bar::Message),

    // temp
    Toggle,
}

pub struct State {
    unsaved: bool,
    path: String, // Change to PathBuf in actual program using files.
    title: RefCount<String>,
}

impl State {
    pub fn try_new(
        localisation: &Localisation,
    ) -> Result<State, ApplicationError> {
        let local: DateTime<Local> = Local::now();
        let name = local.format("%s").to_string();
        let title = localise(localisation, name.clone())?.pop().unwrap();
        Ok(State {
            unsaved: false,
            path: name,
            title,
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
        WindowType::Main
    }

    fn title<'a>(&'a self, _string_cache: &'a StringCache) -> &String {
        &self.title
    }

    fn try_update(
        &mut self,
        message: application::Message,
        _string_cache: &StringCache,
    ) -> Result<Task<application::Message>, ApplicationError> {
        let tasks = Task::none();
        match message {
            application::Message::Main(_id, ref main_message) => {
                match main_message {
                    Message::Toggle => self.unsaved = !self.unsaved,
                    _ => {}
                }
            }
            _ => {}
        }
        Ok(tasks)
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let reverse_lines = localisation.layout_data().reverse_lines;
        let common = string_cache.get(&StringGroup::MainCommon).unwrap();
        //let common_actual = common.as_any().downcast_ref::<main_common::Strings>().unwrap();
        //let strings = string_cache.get(&StringGroup::Main).unwrap();
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
        let unsaved = if self.unsaved {
            "Unsaved data."
        } else {
            "All data saved."
        };
        content.push(text("Test message - Temporary.").into());
        content.push(
            column![row![
                text("Button to the right toggles unsaved data."),
                button(text(common.string(localisation::main_common::Index::Help as usize)))
                .padding([5, 10])
                .on_press(application::Message::Main(id, Message::Toggle)),
                text(unsaved)
            ]]
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

    fn try_localise(
        &mut self,
        localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        println!("updating localised strings for Main");
        self.title = localise(localisation, self.path.clone())?.pop().unwrap();
        Ok(())
    }
}

impl SaveDataTrait for State {
    fn try_save(&mut self) -> Result<(), ApplicationError> {
        println!("Saving data");
        self.unsaved = false;
        Ok(())
    }

    fn is_unsaved(&self) -> bool {
        self.unsaved
    }

    fn name(&self) -> &str {
        self.path.as_str()
    }
}

pub fn display(
    application: &mut application::State,
) -> Result<Task<application::Message>, ApplicationError> {
    let state: Box<dyn AnyWindowTrait> = Box::new(State::try_new(&application.localisation)?);
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
    Ok(application.manager.try_spawn_new_thread(&mut application.session, state)?)
}

pub fn try_update(
    application: &mut application::State,
    message: application::Message,
) -> Result<Task<application::Message>, ApplicationError> {
    let mut tasks = Task::none();
    match message {
        application::Message::Main(id, ref main_message) => {
            let Some(&mut ref mut state) = application.manager.state_mut(&id) else {
                return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
            };
            tasks = state.try_update(message.clone(), &application.string_cache)?;

            // Post internal update
            match main_message {
                Message::MenuBar(menubar_message) => match menubar_message {
                    menu_bar::Message::None => {} // No action.
                    menu_bar::Message::New(window_type) => tasks = application.open_thread(window_type.clone())?,
                    //menu_bar::Message::Open(window_type) => tasks = application.open_thread(window_type.clone())?,
                    menu_bar::Message::Close(id) => tasks = try_to_close(application, *id)?,
                    menu_bar::Message::CloseAll => tasks = application.close_all()?,
                    menu_bar::Message::Preferences => tasks = preferences::display(application, id)?,
                    menu_bar::Message::About => tasks = about::display(application, id)?,
                },
                _ => {}
            };
        },
        _ => {}
    }
    Ok(tasks)
}

fn localise(
    localisation: &Localisation,
    name: String,
) -> Result<Vec<RefCount<String>>, CoreError> {
    let title = {
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
        );
        let localised = localisation.literal_with_defaults("word", "main_i")?;
        values.insert(
            "window".to_string(),
            PlaceholderValue::Localised(localised.0, localised.1),
        );
        values.insert(
            "name".to_string(),
            PlaceholderValue::String(name),
        );
        localisation.format_with_defaults("application", "window_title_name_format", &values)?
    }.0;
    Ok(vec![title])
}

pub fn try_to_close(
    application: &mut application::State,
    id: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    let Some(state) = application.manager.state(&id) else {
        return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
    };
    let actual = state.as_any().downcast_ref::<State>().unwrap();
    if actual.is_unsaved() {
        return  Ok(unsaved_data::display(application, id, actual.name().to_string().as_str())?);
    }
    Ok(application.close_thread(id)?)
}
