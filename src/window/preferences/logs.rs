// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

#![allow(clippy::single_match)]

use crate::{
    application::{self, log::LogLevel, session::Settings, StringGroup},
    core::{
        localisation::{Localisation, StringCache},
        traits::{AnyLocalisedTrait, TabTrait},
    },
    localisation::preferences::{Index, Strings},
    window::preferences::{self, Setting},
};

#[allow(unused_imports)]
use iced::{
    widget::{button, column, combo_box, row, scrollable, text, Column, Row},
    window, Alignment, Task, Element, Length, Point, Size,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone, PartialEq)]
pub enum Message {
    LogLevelSelectedDefault(String),
    LogLevelSelectedApplication(String),
    LogLevelSelectedOther(String),
    LogLevelSelectedIced(String),
    LogLevelSelectedI18n(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum LogSetting {
    LevelDefault(LogLevel),
    LevelApplication(LogLevel),
    LevelOther(LogLevel),
    LevelIced(LogLevel),
    LevelI18n(LogLevel),
}

pub struct Tab {
    pub list: combo_box::State<String>,
    pub original_default: LogLevel,
    pub selected_default: LogLevel,
    pub selected_default_string: Option<String>,
    pub original_application: LogLevel,
    pub selected_application: LogLevel,
    pub selected_application_string: Option<String>,
    pub original_other: LogLevel,
    pub selected_other: LogLevel,
    pub selected_other_string: Option<String>,
    pub original_iced: LogLevel,
    pub selected_iced: LogLevel,
    pub selected_iced_string: Option<String>,
    pub original_i18n: LogLevel,
    pub selected_i18n: LogLevel,
    pub selected_i18n_string: Option<String>,
}

impl Tab {
    pub fn new(
        strings: &Box<dyn AnyLocalisedTrait>,
        settings: &Settings,
    ) -> Self {
        let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
        let original_default = settings.log_levels.default.clone();
        let selected_default = original_default.clone();
        let selected_default_string = actual
            .log_map_to_string(&settings.log_levels.default)
            .map(|x| x.to_string());
        let original_application = settings.log_levels.application.clone();
        let selected_application = original_application.clone();
        let selected_application_string = actual
            .log_map_to_string(&settings.log_levels.application)
            .map(|x| x.to_string());
        let original_other = settings.log_levels.application.clone();
        let selected_other = original_other.clone();
        let selected_other_string = actual
            .log_map_to_string(&settings.log_levels.other)
            .map(|x| x.to_string());
        let original_iced = settings.log_levels.iced.clone();
        let selected_iced = original_iced.clone();
        let selected_iced_string = actual
            .log_map_to_string(&settings.log_levels.iced)
            .map(|x| x.to_string());
        let original_i18n = settings.log_levels.i18n.clone();
        let selected_i18n = original_i18n.clone();
        let selected_i18n_string = actual
            .log_map_to_string(&settings.log_levels.i18n)
            .map(|x| x.to_string());
        Tab {
            list: combo_box::State::new(actual.log_list().to_vec()),
            original_default,
            selected_default,
            selected_default_string,
            original_application,
            selected_application,
            selected_application_string,
            original_other,
            selected_other,
            selected_other_string,
            original_iced,
            selected_iced,
            selected_iced_string,
            original_i18n,
            selected_i18n,
            selected_i18n_string,
        }
    }

    // Update localised combo box selection strings
    pub fn update(&mut self, actual: &Strings) {
        self.selected_default_string = actual
            .log_map_to_string(&self.selected_default)
            .map(|x| x.to_string());
        self.selected_application_string = actual
            .log_map_to_string(&self.selected_application)
            .map(|x| x.to_string());
        self.selected_other_string = actual
            .log_map_to_string(&self.selected_other)
            .map(|x| x.to_string());
        self.selected_iced_string = actual
            .log_map_to_string(&self.selected_iced)
            .map(|x| x.to_string());
        self.selected_i18n_string = actual
            .log_map_to_string(&self.selected_i18n)
            .map(|x| x.to_string());
    }

    pub fn selected(&mut self, message: Message, string_cache: &StringCache) {
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        let actual = strings.as_any().downcast_ref::<Strings>().unwrap();
        match message {
            Message::LogLevelSelectedDefault(log_level) => {
                self.selected_default = *actual.log_map_to_level(&log_level).unwrap();
                self.selected_default_string = Some(log_level);
            }
            Message::LogLevelSelectedApplication(log_level) => {
                self.selected_application = *actual.log_map_to_level(&log_level).unwrap();
                self.selected_application_string = Some(log_level);
            }
            Message::LogLevelSelectedOther(log_level) => {
                self.selected_other = *actual.log_map_to_level(&log_level).unwrap();
                self.selected_other_string = Some(log_level);
            }
            Message::LogLevelSelectedIced(log_level) => {
                self.selected_iced = *actual.log_map_to_level(&log_level).unwrap();
                self.selected_iced_string = Some(log_level);
            }
            Message::LogLevelSelectedI18n(log_level) => {
                self.selected_i18n = *actual.log_map_to_level(&log_level).unwrap();
                self.selected_i18n_string = Some(log_level);
            }
        }
    }

    pub fn check_change(
        &self,
        changed_settings: &mut Vec<Setting>,
    ) {
        if self.original_default != self.selected_default {
            changed_settings.push(Setting::Log(LogSetting::LevelDefault(
                self.selected_default,
            )));
        }
        if self.original_application != self.selected_application {
            changed_settings.push(Setting::Log(LogSetting::LevelApplication(
                self.selected_application,
            )));
        }
        if self.original_other != self.selected_other {
            changed_settings.push(Setting::Log(LogSetting::LevelOther(self.selected_other)));
        }
        if self.original_iced != self.selected_iced {
            changed_settings.push(Setting::Log(LogSetting::LevelIced(self.selected_iced)));
        }
        if self.original_i18n != self.selected_i18n {
            changed_settings.push(Setting::Log(LogSetting::LevelI18n(self.selected_i18n)));
        }
    }
}

impl TabTrait for Tab {
    fn title<'a>(&self, string_cache: &'a StringCache) -> String {
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();
        String::from(strings.string(Index::Logs as usize))
    }

    fn content<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, application::Message> {
        let reverse_words = localisation.layout_data().reverse_words;
        let strings = string_cache.get(&StringGroup::Preferences).unwrap();

        #[allow(unused_mut)]
        let mut settings: Vec<Element<application::Message>> =
            Vec::<Element<application::Message>>::new();

        // Default log level
        let mut setting: Vec<Element<application::Message>> = vec![
            text(strings.string(Index::LogLevelDefault as usize)).into(),
            text("").width(Length::Fill).into(),
            combo_box(
                &self.list,
                strings.string(Index::LogPlaceholder as usize),
                self.selected_default_string.as_ref(),
                move |string| {
                    application::Message::Preferences(
                        id,
                        preferences::Message::Log(Message::LogLevelSelectedDefault(string)),
                    )
                },
            )
            .width(100)
            .into(),
        ];
        if reverse_words {
            setting.reverse();
        }
        settings.push(row(setting).into());

        // All other log settings if default is not set to Off
        if self.selected_default != LogLevel::Off {
            // application log level
            let mut setting: Vec<Element<application::Message>> = vec![
                text(strings.string(Index::LogLevelApplication as usize)).into(),
                text("").width(Length::Fill).into(),
                combo_box(
                    &self.list,
                    strings.string(Index::LogPlaceholder as usize),
                    self.selected_application_string.as_ref(),
                    move |string| {
                        application::Message::Preferences(
                            id,
                            preferences::Message::Log(Message::LogLevelSelectedApplication(string)),
                        )
                    },
                )
                .width(100)
                .into(),
            ];
            if reverse_words {
                setting.reverse();
            }
            settings.push(row(setting).into());

            // other crates log level
            let mut setting: Vec<Element<application::Message>> = vec![
                text(strings.string(Index::LogLevelOther as usize)).into(),
                text("").width(Length::Fill).into(),
                combo_box(
                    &self.list,
                    strings.string(Index::LogPlaceholder as usize),
                    self.selected_other_string.as_ref(),
                    move |string| {
                        application::Message::Preferences(
                            id,
                            preferences::Message::Log(Message::LogLevelSelectedOther(string)),
                        )
                    },
                )
                .width(100)
                .into(),
            ];
            if reverse_words {
                setting.reverse();
            }
            settings.push(row(setting).into());

            // iced crate log level
            let mut setting: Vec<Element<application::Message>> = vec![
                text(strings.string(Index::LogLevelIced as usize)).into(),
                text("").width(Length::Fill).into(),
                combo_box(
                    &self.list,
                    strings.string(Index::LogPlaceholder as usize),
                    self.selected_iced_string.as_ref(),
                    move |string| {
                        application::Message::Preferences(
                            id,
                            preferences::Message::Log(Message::LogLevelSelectedIced(string)),
                        )
                    },
                )
                .width(100)
                .into(),
            ];
            if reverse_words {
                setting.reverse();
            }
            settings.push(row(setting).into());

            // i18n crate log level
            let mut setting: Vec<Element<application::Message>> = vec![
                text(strings.string(Index::LogLevelI18n as usize)).into(),
                text("").width(Length::Fill).into(),
                combo_box(
                    &self.list,
                    strings.string(Index::LogPlaceholder as usize),
                    self.selected_i18n_string.as_ref(),
                    move |string| {
                        application::Message::Preferences(
                            id,
                            preferences::Message::Log(Message::LogLevelSelectedI18n(string)),
                        )
                    },
                )
                .width(100)
                .into(),
            ];
            if reverse_words {
                setting.reverse();
            }
            settings.push(row(setting).into());
        }
        Column::new()
            .push(column(settings))
            .width(Length::Fill)
            .height(Length::Fill)
            .into()

    }
}
