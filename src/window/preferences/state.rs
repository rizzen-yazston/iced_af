// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

#![allow(clippy::single_match)]

use crate::{
    application::{
        self,
        log::update_logger,
        ApplicationError, session::Settings, WindowType, StringGroup},
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
        traits::{AnyWindowTrait, TabTrait, WindowTrait},
    },
    localisation::preferences::{Index, Strings},
    window::preferences::{language, logs,},
};
use i18n::utility::LanguageTag;
use iced::{
    Alignment,
    widget::{button, column, row,},
    window, Task, Element, Length as Length,
};

#[cfg(feature = "iced_aw")]
use crate::iced_aw::widgets::sidebar::{self, SidebarWithContent, TabLabel};

#[cfg(not(feature = "iced_aw"))]
use iced_aw::widgets::sidebar::{self, SidebarWithContent, TabLabel};

use std::{
    any::Any,
    rc::Rc as RefCount,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    Accept,
    Cancel,
    TabSelected(TabId),
    TabClosed(TabId), /// To be removed, testing new sidebar widget
    Language(language::Message),
    Log(logs::Message),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Setting {
    Language(RefCount<LanguageTag>),
    Log(logs::LogSetting),
}

#[derive(PartialEq, Eq, Clone, Debug)]
pub enum TabId {
    Language,
    Logs,
}

pub struct State {
    changed_settings: Option<Vec<Setting>>,
    first_use: bool,

    // Tabs
    active_tab: TabId,
    language: language::Tab, // i18n settings
    logs: logs::Tab, // log settings
}

impl State {
    pub fn try_new(
        localisation: &Localisation,
        string_cache: &StringCache,
        settings: &Settings,
        first_use: bool,
    ) -> Result<Self, ApplicationError> {
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        Ok(State {
            changed_settings: None,
            first_use,

            // Tabs
            active_tab: TabId::Language,
            language: language::Tab::try_new(localisation, strings, settings)?,
            logs: logs::Tab::new(strings, settings),
        })
    }

    pub fn result_vector(&mut self) -> Option<Vec<Setting>> {
        self.changed_settings.take()
    }

    pub fn language_update(&self) -> bool {
        self.language.update()
    }

    pub fn language_changed(&self) -> bool {
        self.language.changed()
    }

    pub fn is_first_use(&self) -> bool {
        self.first_use
    }

    pub fn end_first_use(&mut self) {
        self.first_use = false;
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
        WindowType::Preferences
    }

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String {
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        strings.title()
    }

    fn try_update(
        &mut self,
        message: application::Message,
        string_cache: &StringCache,
    ) -> Result<Task<application::Message>, ApplicationError> {
        let task = Task::none();
        match message {
            application::Message::Preferences(_id, message) => match message {
                Message::Cancel => {
                    self.language.update_changed()
                }
                Message::TabSelected(tab_id) => self.active_tab = tab_id,
                Message::TabClosed(_tab_id) => {} // for now do nothing
                Message::Language(language) => self.language.selected(language),
                Message::Log(logs) => self.logs.selected(logs, string_cache),
                Message::Accept => {
                    let strings = string_cache.get(&StringGroup::Preferences).unwrap();
                    #[allow(unused_mut)]
                    let mut changed_settings = Vec::<Setting>::new();
                    self.language.check_change(strings, &mut changed_settings);
                    self.logs.check_change(&mut changed_settings);

                    // Insert additional settings above.

                    if !changed_settings.is_empty() {
                        self.changed_settings = Some(changed_settings);
                    }
                }
            },
            _ => {}
        }
        Ok(task)
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<application::Message> {
        let align_end = localisation.layout_data().align_words_end;
        let reverse_words = localisation.layout_data().reverse_words;
        let reverse_lines = localisation.layout_data().reverse_lines;
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        let mut content: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();

        // Preferences - scrollable
        #[allow(unused_mut)]
        let mut tabs: Vec<(TabId, TabLabel, Element<application::Message>)> =
            Vec::<(TabId, TabLabel, Element<application::Message>)>::new();

        // Language
        tabs.push((
            TabId::Language,
            self.language.tab_label(string_cache),
            self.language.view(id, localisation, string_cache),
        ));

        // Logs
        let display = !self.first_use;
        if display {
            tabs.push((
                TabId::Logs,
                self.logs.tab_label(string_cache),
                self.logs.view(id, localisation, string_cache),
            ));
        }

        // Add additional preferences above this comment.

        if reverse_lines {
            tabs.reverse();
        }
        let preferences = SidebarWithContent::new_with_tabs(tabs, move |tab_id| {
            application::Message::Preferences(id, Message::TabSelected(tab_id))
        })
        //.tab_icon_position(sidebar::Position::Start)
        .set_active_tab(&self.active_tab)
        .sidebar_position(sidebar::SidebarPosition::Start)
        .align_tabs(Alignment::Start)
        //.on_close(move |tab_id| application::Message::Preferences(id, Message::TabClosed(tab_id)))
        //.close_icon_position(sidebar::Position::End)
        .tab_label_padding(0.0);
        content.push(
            preferences
                .width(Length::Fill)
                .height(Length::Fill)
                .into(),
        );
        content.push(" ".into()); // Paragraph separation

        // Buttons
        let mut buttons: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();
        buttons.push(
            button(strings.string(Index::Accept as usize).as_str())
                .padding([5, 10])
                .on_press(application::Message::Preferences(id, Message::Accept))
                .into(),
        );
        if !self.first_use {
            buttons.push(
                button(strings.string(Index::Cancel as usize).as_str())
                    .padding([5, 10])
                    .on_press(application::Message::Preferences(id, Message::Cancel))
                    .into(),
            );
        }
        if reverse_words {
            buttons.reverse();
        }

        content.push(
            column![row(buttons).spacing(10)]
                .width(Length::Fill)
                .align_x(align_end)
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
        .exists(&StringGroup::Preferences)
    {
        application.string_cache.insert(
            StringGroup::Preferences,
            Box::new(Strings::try_new(&application.localisation)?),
        );
    }
    let state = State::try_new(
        &application.localisation,
        &application.string_cache,
        &application.session.settings,
        false, // Always false, as first use never calls `display()`
    )?;
    Ok(application
        .manager
        .try_spawn(&mut application.session, Box::new(state), parent)?)
}

pub fn try_update(
    application: &mut application::State,
    message: application::Message,
) -> Result<Task<application::Message>, ApplicationError> {
    let mut task = Task::none();
    match message {
        application::Message::Preferences(id, ref inner_message) => {
            let Some(&mut ref mut state) = application.manager.state_mut(&id) else {
                return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
            };
            task = state.try_update(message.clone(), &application.string_cache)?;

            // Post internal update
            match inner_message {
                Message::Language(language) => {
                    match language {
                        language::Message::LanguageSelected(string) => {
                            let mut _update = false;
                            {
                                let actual = state.as_any().downcast_ref::<State>().unwrap();
                                _update = actual.language_update();
                            }
                            if _update {
                                // Change the localiser default language.
                                {
                                    let strings = application
                                        .string_cache
                                        .get(&StringGroup::Preferences)
                                        .unwrap();
                                    let actual_strings = strings
                                        .as_any()
                                        .downcast_ref::<Strings>()
                                        .unwrap();
                                    let _ = application.localisation.change_default_language(
                                        RefCount::clone(
                                            actual_strings.language_map_to_tag(&string).unwrap(),
                                        ),
                                    )?;
                                }

                                // Update all windows localisation strings
                                {
                                    let _ = application
                                        .string_cache
                                        .try_update(&application.localisation)?;
                                }

                                // Update localised combo box selection strings
                                {
                                    let actual =
                                        state.as_any_mut().downcast_mut::<State>().unwrap();
                                    let strings = application
                                        .string_cache
                                        .get(&StringGroup::Preferences)
                                        .unwrap();
                                    let actual_strings = strings
                                        .as_any()
                                        .downcast_ref::<Strings>()
                                        .unwrap();
                                    actual.logs.update(actual_strings);
                                }

                                // Update main windows, usually the dynamic title strings.
                                {
                                    let list = application.manager.thread_list();
                                    for thread_id in list {
                                        let Some(&mut ref mut state) = application.manager.state_mut(&thread_id) else {
                                            return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
                                        };
                                        state.try_localise(&application.localisation)?;
                                    }
                                }
                            }
                        }
                    }
                }
                Message::Accept => {
                    let mut _changed_settings: Option<Vec<Setting>> = None;
                    {
                        let actual = state.as_any_mut().downcast_mut::<State>().unwrap();
                        _changed_settings = actual.result_vector();
                    }
                    debug!("{:?}", _changed_settings);
                    let mut logging_update = false;

                    // Handle all the changed settings, where necessary update components that require immediate
                    // effect.
                    if _changed_settings.is_some() {
                        let binding = _changed_settings.unwrap();
                        let iterator = binding.iter();
                        for setting in iterator {
                            match setting {
                                Setting::Language(language) => {
                                    application.session.settings.ui.language =
                                        language.as_str().to_string();
                                }
                                Setting::Log(log) => match log {
                                    logs::LogSetting::LevelDefault(log_level) => {
                                        application.session.settings.log_levels.default =
                                            *log_level;
                                        trace!(
                                            "Default: {}",
                                            application.session.settings.log_levels.default
                                        );
                                        logging_update = true;
                                    }
                                    logs::LogSetting::LevelApplication(log_level) => {
                                        application.session.settings.log_levels.application =
                                            *log_level;
                                        trace!(
                                            "Application: {}",
                                            application.session.settings.log_levels.application
                                        );
                                        logging_update = true;
                                    }
                                    logs::LogSetting::LevelOther(log_level) => {
                                        application.session.settings.log_levels.other = *log_level;
                                        trace!(
                                            "Other: {}",
                                            application.session.settings.log_levels.other
                                        );
                                        logging_update = true;
                                    }
                                    logs::LogSetting::LevelIced(log_level) => {
                                        application.session.settings.log_levels.iced = *log_level;
                                        trace!(
                                            "iced: {}",
                                            application.session.settings.log_levels.iced
                                        );
                                        logging_update = true;
                                    }
                                    logs::LogSetting::LevelI18n(log_level) => {
                                        application.session.settings.log_levels.i18n = *log_level;
                                        trace!(
                                            "i18n: {}",
                                            application.session.settings.log_levels.i18n
                                        );
                                        logging_update = true;
                                    }
                                },

                                #[allow(unreachable_patterns)]
                                _ => {}
                            }
                        }
                    }
                    if logging_update {
                        update_logger(
                            &mut application.environment.logger,
                            &application.session.settings.log_levels,
                        )
                    }
                    task = close(application, id)?
                }
                Message::Cancel => {
                    let mut _update = false;
                    {
                        let actual = state.as_any().downcast_ref::<State>().unwrap();
                        _update = actual.language_changed();
                    }
                    if _update {
                        // Reset back to session's language.
                        {
                            let tag = match application
                                .localisation
                                .language_tag_registry()
                                .tag(application.session.settings.ui.language.as_str())
                            {
                                Err(error) => {
                                    return Err(ApplicationError::Core(
                                        CoreError::LanguageTagRegistry(error),
                                    ))
                                }
                                Ok(value) => value,
                            };
                            let _ = application.localisation.change_default_language(tag)?;
                        }

                        // Update all windows localisation strings
                        {
                            let _ = application
                                .string_cache
                                .try_update(&application.localisation)?;
                        }

                        // Update main windows, usually the dynamic title strings.
                        {
                            let list = application.manager.thread_list();
                            for thread_id in list {
                                let Some(&mut ref mut state) = application.manager.state_mut(&thread_id) else {
                                    return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
                                };
                                state.try_localise(&application.localisation)?;
                            }
                        }
                    }
                    task = close(application, id)?
                }
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
        _ => {}
    }
    Ok(task)
}

pub fn cancel_and_close(
    application: &mut application::State,
    id: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    Ok(try_update(
        application,
        application::Message::Preferences(id, Message::Cancel),
    )?)
}

pub fn close(
    application: &mut application::State,
    id: window::Id,
) -> Result<Task<application::Message>, ApplicationError> {
    let Some(&mut ref mut state) = application.manager.state_mut(&id) else {
        return Err(CoreError::WindowIdNotFound(id, "window_states".to_string()))?;
    };
    let actual = state.as_any_mut().downcast_mut::<State>().unwrap();
    if actual.is_first_use() {
        Ok(application.close_thread(id)?)

    } else {
        Ok(application.manager.close(id)?)
    }
}
