// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

#![allow(clippy::single_match)]

use crate::{
    core::{
        application::{ApplicationMessage, ApplicationThread, WindowType},
        error::ApplicationError,
        session::Settings,
        traits::{AnyWindowTrait, WindowTrait},
    },
    widget::event_control,
    window::{fatal_error::display_fatal_error, main::Main},
    APPLICATION_NAME_SHORT,
};
#[allow(unused_imports)]
use iced::{
    widget::{button, column, combo_box, row, scrollable, text, Column},
    window, Alignment, Command, Element, Length, Point, Size,
};
use std::any::Any;

#[cfg(feature = "i18n")]
use crate::core::{
    environment::Environment,
    localisation::{Localisation, ScriptData},
    session::Session,
};

#[cfg(feature = "i18n")]
use i18n::utility::{LanguageTag, PlaceholderValue, TaggedString as LString};

#[cfg(not(feature = "i18n"))]
use std::string::String as LString;

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(all(feature = "i18n", feature = "sync"))]
use std::sync::Arc as RefCount;

#[cfg(all(feature = "i18n", not(feature = "sync")))]
use std::rc::Rc as RefCount;

#[cfg(feature = "log")]
use crate::core::log::{update_logger, LogLevel};

#[cfg(feature = "i18n")]
use std::collections::HashMap;

use std::collections::hash_map;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 400f32, 300f32 );
pub const SIZE_DEFAULT: (f32, f32) = (500f32, 300f32);
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

#[derive(Debug, Clone, PartialEq)]
pub enum PreferencesMessage {
    Accept(window::Id),
    Cancel(window::Id),
    #[cfg(feature = "i18n")]
    LanguageSelected(String),
    #[cfg(feature = "log")]
    LogLevelSelectedDefault(String),
    #[cfg(feature = "log")]
    LogLevelSelectedApplication(String),
    #[cfg(feature = "log")]
    LogLevelSelectedOther(String),
    #[cfg(feature = "log")]
    LogLevelSelectedIced(String),
    #[cfg(all(feature = "log", feature = "i18n"))]
    LogLevelSelectedI18n(String),
}

#[derive(PartialEq, Clone, Debug)]
pub enum Setting {
    Language(String),
    #[cfg(feature = "log")]
    LogLevelDefault(LogLevel),
    #[cfg(feature = "log")]
    LogLevelApplication(LogLevel),
    #[cfg(feature = "log")]
    LogLevelOther(LogLevel),
    #[cfg(feature = "log")]
    LogLevelIced(LogLevel),
    #[cfg(all(feature = "log", feature = "i18n"))]
    LogLevelI18n(LogLevel),
}

struct PreferencesLocalisation {
    #[cfg(feature = "i18n")]
    language: RefCount<LanguageTag>,
    #[cfg(feature = "i18n")]
    script_data: ScriptData,

    // Strings
    title: LString,
    accept: LString,
    cancel: LString,

    // i18n settings
    #[cfg(feature = "i18n")]
    languages_with_percentage: Vec<LString>,
    #[cfg(feature = "i18n")]
    ui_language: LString,
    #[cfg(feature = "i18n")]
    placeholder_language: LString,

    // log settings
    #[cfg(feature = "log")]
    log_level_default: LString,
    #[cfg(feature = "log")]
    log_level_application: LString,
    #[cfg(feature = "log")]
    log_level_other: LString,
    #[cfg(feature = "log")]
    log_level_iced: LString,
    #[cfg(feature = "log")]
    log_level_i18n: LString,
    #[cfg(feature = "log")]
    placeholder_log_level: LString,
    #[cfg(feature = "log")]
    default: LString,
    #[cfg(feature = "log")]
    off: LString,
    #[cfg(feature = "log")]
    error: LString,
    #[cfg(feature = "log")]
    warning: LString,
    #[cfg(feature = "log")]
    information: LString,
    #[cfg(feature = "log")]
    debug: LString,
    #[cfg(feature = "log")]
    trace: LString,
}

impl PreferencesLocalisation {
    pub fn try_new(
        #[cfg(feature = "i18n")] localisation: &Localisation,
        #[cfg(feature = "i18n")] languages_available: &[(RefCount<LanguageTag>, f32)],
    ) -> Result<Self, ApplicationError> {
        #[cfg(feature = "i18n")]
        let language = localisation.localiser().default_language();

        #[cfg(feature = "i18n")]
        let language_identifier = localisation
            .localiser()
            .language_tag_registry()
            .identifier(language.as_str())?;

        #[cfg(feature = "i18n")]
        let title = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
            );
            values.insert(
                "window".to_string(),
                PlaceholderValue::TaggedString(
                    localisation
                        .localiser()
                        .literal_with_defaults("word", "preferences_i")?,
                ),
            );
            localisation.localiser().format_with_defaults(
                "application",
                "window_title_format",
                &values,
            )?
        };

        #[cfg(not(feature = "i18n"))]
        let title = format!("{} - Preferences", APPLICATION_NAME_SHORT);

        #[cfg(feature = "i18n")]
        let accept = localisation
            .localiser()
            .literal_with_defaults("word", "accept_i")?;

        #[cfg(not(feature = "i18n"))]
        let accept = "Accept".to_string();

        #[cfg(feature = "i18n")]
        let cancel = localisation
            .localiser()
            .literal_with_defaults("word", "cancel_i")?;

        #[cfg(not(feature = "i18n"))]
        let cancel = "Cancel".to_string();

        // i18n settings
        #[cfg(feature = "i18n")]
        let mut languages_with_percentage = Vec::<LString>::new();

        #[cfg(feature = "i18n")]
        {
            let iterator = languages_available.iter();
            for language in iterator {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                let language_string = language.0.as_str().to_string();
                values.insert(
                    "language".to_string(),
                    PlaceholderValue::String(language_string.clone()),
                );
                values.insert(
                    "percent".to_string(),
                    PlaceholderValue::Unsigned((language.1 * 100f32) as u128),
                );
                let text = localisation.localiser().format_with_defaults(
                    "application",
                    "language_percent_format",
                    &values,
                )?;
                languages_with_percentage.push(text);
            }
        }

        #[cfg(feature = "i18n")]
        let ui_language = localisation
            .localiser()
            .literal_with_defaults("application", "ui_language")?;

        #[cfg(feature = "i18n")]
        let placeholder_language = localisation
            .localiser()
            .literal_with_defaults("application", "placeholder_language")?;

        // log settings
        #[cfg(all(feature = "log", feature = "i18n"))]
        let log_level_default = localisation
            .localiser()
            .literal_with_defaults("application", "log_level_default")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let log_level_default = "Default log level";

        #[cfg(all(feature = "log", feature = "i18n"))]
        let log_level_application = localisation
            .localiser()
            .literal_with_defaults("application", "log_level_application")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let log_level_application = "Application log level";

        #[cfg(all(feature = "log", feature = "i18n"))]
        let log_level_other = localisation
            .localiser()
            .literal_with_defaults("application", "log_level_other")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let log_level_other = "Other components' log level";

        #[cfg(all(feature = "log", feature = "i18n"))]
        let log_level_iced = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "component".to_string(),
                PlaceholderValue::String("iced".to_string()),
            );
            localisation.localiser().format_with_defaults(
                "application",
                "log_level_component",
                &values,
            )?
        };

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let log_level_iced = "Log level for ‘iced’.";

        #[cfg(all(feature = "log", feature = "i18n"))]
        let log_level_i18n = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "component".to_string(),
                PlaceholderValue::String("i18n".to_string()),
            );
            localisation.localiser().format_with_defaults(
                "application",
                "log_level_component",
                &values,
            )?
        };

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let log_level_i18n = "Log level for ‘i18n’.";

        #[cfg(all(feature = "log", feature = "i18n"))]
        let placeholder_log_level = localisation
            .localiser()
            .literal_with_defaults("application", "placeholder_log_level")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let placeholder_log_level = "Type a log level…".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let default = localisation
            .localiser()
            .literal_with_defaults("word", "default_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let default = "Default".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let off = localisation
            .localiser()
            .literal_with_defaults("word", "off_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let off = "Off".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let error = localisation
            .localiser()
            .literal_with_defaults("word", "error_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let error = "Error".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let warning = localisation
            .localiser()
            .literal_with_defaults("word", "warning_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let warning = "Warning".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let information = localisation
            .localiser()
            .literal_with_defaults("word", "information_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let information = "Information".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let debug = localisation
            .localiser()
            .literal_with_defaults("word", "debug_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let debug = "Debug".to_string();

        #[cfg(all(feature = "log", feature = "i18n"))]
        let trace = localisation
            .localiser()
            .literal_with_defaults("word", "trace_i")?;

        #[cfg(all(feature = "log", not(feature = "i18n")))]
        let trace = "Trace".to_string();

        Ok(PreferencesLocalisation {
            #[cfg(feature = "i18n")]
            language,
            #[cfg(feature = "i18n")]
            script_data: ScriptData::new(localisation, &language_identifier),
            title,
            accept,
            cancel,

            // i18n settings
            #[cfg(feature = "i18n")]
            languages_with_percentage,
            #[cfg(feature = "i18n")]
            ui_language,
            #[cfg(feature = "i18n")]
            placeholder_language,

            // log settings
            #[cfg(feature = "log")]
            log_level_default,
            #[cfg(feature = "log")]
            log_level_application,
            #[cfg(feature = "log")]
            log_level_other,
            #[cfg(feature = "log")]
            log_level_iced,
            #[cfg(feature = "log")]
            log_level_i18n,
            #[cfg(feature = "log")]
            placeholder_log_level,
            #[cfg(feature = "log")]
            default,
            #[cfg(feature = "log")]
            off,
            #[cfg(feature = "log")]
            error,
            #[cfg(feature = "log")]
            warning,
            #[cfg(feature = "log")]
            information,
            #[cfg(feature = "log")]
            debug,
            #[cfg(feature = "log")]
            trace,
        })
    }
}

pub struct Preferences {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: PreferencesLocalisation,
    settings: Settings,
    changed_settings: Option<Vec<Setting>>,
    #[cfg(feature = "first_use")]
    first_use: bool,

    // i18n settings
    #[cfg(feature = "i18n")]
    ui_language: RefCount<LanguageTag>,
    #[cfg(feature = "i18n")]
    languages_available: Vec<(RefCount<LanguageTag>, f32)>,
    #[cfg(feature = "i18n")]
    language_list: combo_box::State<String>,
    #[cfg(feature = "i18n")]
    language_map_to_tag: HashMap<String, RefCount<LanguageTag>>,
    #[cfg(feature = "i18n")]
    language_map_to_string: HashMap<RefCount<LanguageTag>, String>,
    #[cfg(feature = "i18n")]
    language_selected: Option<String>,
    #[cfg(feature = "i18n")]
    language_changed: bool,

    // log settings
    //#[cfg(feature = "log")]
    //log_levels_default: combo_box::State<String>,
    #[cfg(feature = "log")]
    log_levels_other: combo_box::State<String>,
    #[cfg(feature = "log")]
    log_level_map_to_level: HashMap<String, LogLevel>,
    #[cfg(feature = "log")]
    log_level_map_to_string: HashMap<LogLevel, String>,
    #[cfg(feature = "log")]
    log_level_selected_default: Option<String>,
    #[cfg(feature = "log")]
    log_level_selected_application: Option<String>,
    #[cfg(feature = "log")]
    log_level_selected_other: Option<String>,
    #[cfg(feature = "log")]
    log_level_selected_iced: Option<String>,
    #[cfg(feature = "log")]
    log_level_selected_i18n: Option<String>,
}

impl Preferences {
    pub fn try_new(
        #[cfg(feature = "i18n")] localisation: &Localisation,
        settings: &Settings,
        #[cfg(feature = "first_use")] first_use: bool,
    ) -> Result<Self, ApplicationError> {
        // Get settings needed for localisation.
        #[cfg(feature = "i18n")]
        let ui_language = localisation
            .localiser()
            .language_tag_registry()
            .tag(settings.ui.language.as_str())?;

        // Build language list for localisation.
        #[cfg(feature = "i18n")]
        let mut languages_available = Vec::<(RefCount<LanguageTag>, f32)>::new();

        #[cfg(feature = "i18n")]
        {
            let binding = localisation
                .localiser()
                .localisation_provider()
                .component_details("application")?;
            let iterator = binding.languages.iter();
            for language_data in iterator {
                languages_available.push((language_data.0.clone(), language_data.1.ratio));
            }
            #[cfg(feature = "log")]
            debug!("Got component details");
        }

        // Create localisation strings
        let localisation = PreferencesLocalisation::try_new(
            #[cfg(feature = "i18n")]
            localisation,
            #[cfg(feature = "i18n")]
            &languages_available,
        )?;

        // i18n settings
        #[cfg(feature = "i18n")]
        let mut language_map_to_tag = HashMap::<String, RefCount<LanguageTag>>::new();
        #[cfg(feature = "i18n")]
        let mut language_map_to_string = HashMap::<RefCount<LanguageTag>, String>::new();
        #[cfg(feature = "i18n")]
        let mut language_selected: Option<String> = None;
        #[cfg(feature = "i18n")]
        let mut language_list = Vec::<String>::new();
        #[cfg(feature = "i18n")]
        for (index, language_data) in languages_available.iter().enumerate() {
            let display_string = localisation.languages_with_percentage[index]
                .as_str()
                .to_string();
            if settings.ui.language.as_str() == language_data.0.as_str() {
                language_selected = Some(display_string.to_string());
            }

            language_map_to_tag.insert(display_string.clone(), RefCount::clone(&language_data.0));
            language_map_to_string
                .insert(RefCount::clone(&language_data.0), display_string.clone());
            language_list.push(display_string);
        }

        // log settings
        #[cfg(feature = "log")]
        let (
            //log_levels_default,
            log_levels_other,
            log_level_map_to_level,
            log_level_map_to_string,
            log_level_selected_default,
            log_level_selected_application,
            log_level_selected_other,
            log_level_selected_iced,
            log_level_selected_i18n,
        ) = Self::build_log_combo_boxes(&localisation, settings);

        Ok(Preferences {
            enabled: true,
            parent: Some(WindowType::Main),
            localisation,
            settings: settings.clone(),
            changed_settings: None,
            #[cfg(feature = "first_use")]
            first_use,

            // i18n settings
            #[cfg(feature = "i18n")]
            ui_language,
            #[cfg(feature = "i18n")]
            languages_available,
            #[cfg(feature = "i18n")]
            language_list: combo_box::State::new(language_list),
            #[cfg(feature = "i18n")]
            language_map_to_tag,
            #[cfg(feature = "i18n")]
            language_map_to_string,
            #[cfg(feature = "i18n")]
            language_selected,
            #[cfg(feature = "i18n")]
            language_changed: false,

            // log settings
            //#[cfg(feature = "log")]
            //log_levels_default,
            #[cfg(feature = "log")]
            log_levels_other,
            #[cfg(feature = "log")]
            log_level_map_to_level,
            #[cfg(feature = "log")]
            log_level_map_to_string,
            #[cfg(feature = "log")]
            log_level_selected_default,
            #[cfg(feature = "log")]
            log_level_selected_application,
            #[cfg(feature = "log")]
            log_level_selected_other,
            #[cfg(feature = "log")]
            log_level_selected_iced,
            #[cfg(feature = "log")]
            log_level_selected_i18n,
        })
    }

    pub fn result_vector(&mut self) -> Option<Vec<Setting>> {
        self.changed_settings.take()
    }

    pub fn update_settings(&mut self, settings: &Settings) {
        self.settings = settings.clone();
    }

    #[cfg(feature = "i18n")]
    pub fn clear_language_changed(&mut self) -> bool {
        let previous = self.language_changed;
        self.language_changed = false;
        previous
    }

    #[cfg(feature = "i18n")]
    pub fn language_selected(&self) -> &RefCount<LanguageTag> {
        self.language_map_to_tag
            .get(self.language_selected.as_ref().unwrap().as_str())
            .unwrap()
    }

    #[cfg(feature = "first_use")]
    pub fn end_first_use(&mut self) {
        self.first_use = false;
    }

    #[cfg(feature = "log")]
    #[allow(clippy::type_complexity)]
    fn build_log_combo_boxes(
        localisation: &PreferencesLocalisation,
        settings: &Settings,
    ) -> (
        //combo_box::State<String>,
        combo_box::State<String>,
        HashMap<String, LogLevel>,
        HashMap<LogLevel, String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
        Option<String>,
    ) {
        // Build combox box lists
        /* Excludes Default variant
        let log_levels_default = combo_box::State::new(vec![
            localisation.off.as_str().to_string(),
            localisation.error.as_str().to_string(),
            localisation.warning.as_str().to_string(),
            localisation.information.as_str().to_string(),
            localisation.debug.as_str().to_string(),
            localisation.trace.as_str().to_string(),
        ]);
        */
        let log_levels_other = combo_box::State::new(vec![
            localisation.default.as_str().to_string(),
            localisation.off.as_str().to_string(),
            localisation.error.as_str().to_string(),
            localisation.warning.as_str().to_string(),
            localisation.information.as_str().to_string(),
            localisation.debug.as_str().to_string(),
            localisation.trace.as_str().to_string(),
        ]);
        let log_level_map_to_level = {
            let mut map = HashMap::<String, LogLevel>::new();
            map.insert(localisation.default.as_str().to_string(), LogLevel::Default);
            map.insert(localisation.off.as_str().to_string(), LogLevel::Off);
            map.insert(localisation.error.as_str().to_string(), LogLevel::Error);
            map.insert(localisation.warning.as_str().to_string(), LogLevel::Warn);
            map.insert(
                localisation.information.as_str().to_string(),
                LogLevel::Info,
            );
            map.insert(localisation.debug.as_str().to_string(), LogLevel::Debug);
            map.insert(localisation.trace.as_str().to_string(), LogLevel::Trace);
            map
        };
        #[cfg(feature = "log")]
        let log_level_map_to_string = {
            let mut map = HashMap::<LogLevel, String>::new();
            map.insert(LogLevel::Default, localisation.default.as_str().to_string());
            map.insert(LogLevel::Off, localisation.off.as_str().to_string());
            map.insert(LogLevel::Error, localisation.error.as_str().to_string());
            map.insert(LogLevel::Warn, localisation.warning.as_str().to_string());
            map.insert(
                LogLevel::Info,
                localisation.information.as_str().to_string(),
            );
            map.insert(LogLevel::Default, localisation.default.as_str().to_string());
            map.insert(LogLevel::Trace, localisation.trace.as_str().to_string());
            map
        };
        let log_level_selected_default = Some(
            log_level_map_to_string
                .get(&settings.log_levels.default)
                .unwrap()
                .clone(),
        );
        let log_level_selected_application = Some(
            log_level_map_to_string
                .get(&settings.log_levels.application)
                .unwrap()
                .clone(),
        );
        let log_level_selected_other = Some(
            log_level_map_to_string
                .get(&settings.log_levels.other)
                .unwrap()
                .clone(),
        );
        let log_level_selected_iced = Some(
            log_level_map_to_string
                .get(&settings.log_levels.iced)
                .unwrap()
                .clone(),
        );
        let log_level_selected_i18n = Some(
            log_level_map_to_string
                .get(&settings.log_levels.i18n)
                .unwrap()
                .clone(),
        );
        (
            //log_levels_default,
            log_levels_other,
            log_level_map_to_level,
            log_level_map_to_string,
            log_level_selected_default,
            log_level_selected_application,
            log_level_selected_other,
            log_level_selected_iced,
            log_level_selected_i18n,
        )
    }
}

impl AnyWindowTrait for Preferences {}

impl WindowTrait for Preferences {
    fn as_any(&self) -> &dyn Any {
        self
    }

    fn as_any_mut(&mut self) -> &mut dyn Any {
        self
    }

    fn title(&self) -> &LString {
        &self.localisation.title
    }

    fn try_update(
        &mut self,
        message: ApplicationMessage,
    ) -> Result<Command<ApplicationMessage>, ApplicationError> {
        let command = Command::none();
        match message {
            ApplicationMessage::Preferences(inner_message) => {
                match inner_message {
                    PreferencesMessage::Cancel(_id) =>
                    {
                        #[cfg(feature = "i18n")]
                        if self.language_changed {
                            self.language_selected = Some(
                                self.language_map_to_string
                                    .get(&self.ui_language)
                                    .unwrap()
                                    .to_string(),
                            );
                        }
                    }

                    #[cfg(feature = "i18n")]
                    PreferencesMessage::LanguageSelected(language) => {
                        self.language_selected = Some(language.clone());
                        self.language_changed = true;
                    }

                    #[cfg(feature = "log")]
                    PreferencesMessage::LogLevelSelectedDefault(log_level) => {
                        self.log_level_selected_default = Some(log_level);
                    }

                    #[cfg(feature = "log")]
                    PreferencesMessage::LogLevelSelectedApplication(log_level) => {
                        self.log_level_selected_application = Some(log_level);
                    }

                    #[cfg(feature = "log")]
                    PreferencesMessage::LogLevelSelectedOther(log_level) => {
                        self.log_level_selected_other = Some(log_level);
                    }

                    #[cfg(feature = "log")]
                    PreferencesMessage::LogLevelSelectedIced(log_level) => {
                        self.log_level_selected_iced = Some(log_level);
                    }

                    #[cfg(feature = "log")]
                    PreferencesMessage::LogLevelSelectedI18n(log_level) => {
                        self.log_level_selected_i18n = Some(log_level);
                    }

                    PreferencesMessage::Accept(_id) => {
                        #[cfg(feature = "i18n")]
                        let language_selected_tag = self
                            .language_map_to_tag
                            .get(&self.language_selected.clone().unwrap())
                            .unwrap();

                        #[cfg(feature = "log")]
                        info!("Current language: {:?}", language_selected_tag);

                        #[allow(unused_mut)]
                        let mut changed_settings = Vec::<Setting>::new();

                        #[cfg(feature = "i18n")]
                        if self.settings.ui.language.as_str() != language_selected_tag.as_str() {
                            changed_settings.push(Setting::Language(
                                language_selected_tag.as_str().to_string(),
                            ));
                        }

                        #[cfg(feature = "log")]
                        {
                            let log_level_selected_default =
                                self.log_level_selected_default.clone().unwrap();
                            if !log_level_selected_default.eq(&self
                                .settings
                                .log_levels
                                .default
                                .to_string())
                            {
                                changed_settings.push(Setting::LogLevelDefault(
                                    *self
                                        .log_level_map_to_level
                                        .get(&log_level_selected_default)
                                        .unwrap(),
                                ));
                            }
                            let log_level_selected_application =
                                self.log_level_selected_application.clone().unwrap();
                            if !log_level_selected_application.eq(&self
                                .settings
                                .log_levels
                                .application
                                .to_string())
                            {
                                changed_settings.push(Setting::LogLevelApplication(
                                    *self
                                        .log_level_map_to_level
                                        .get(&log_level_selected_application)
                                        .unwrap(),
                                ));
                            }
                            let log_level_selected_other =
                                self.log_level_selected_other.clone().unwrap();
                            if !log_level_selected_other.eq(&self
                                .settings
                                .log_levels
                                .other
                                .to_string())
                            {
                                changed_settings.push(Setting::LogLevelOther(
                                    *self
                                        .log_level_map_to_level
                                        .get(&log_level_selected_other)
                                        .unwrap(),
                                ));
                            }
                            let log_level_selected_iced =
                                self.log_level_selected_iced.clone().unwrap();
                            if !log_level_selected_iced.eq(&self
                                .settings
                                .log_levels
                                .iced
                                .to_string())
                            {
                                changed_settings.push(Setting::LogLevelIced(
                                    *self
                                        .log_level_map_to_level
                                        .get(&log_level_selected_iced)
                                        .unwrap(),
                                ));
                            }
                            let log_level_selected_i18n =
                                self.log_level_selected_i18n.clone().unwrap();
                            if !log_level_selected_i18n.eq(&self
                                .settings
                                .log_levels
                                .i18n
                                .to_string())
                            {
                                changed_settings.push(Setting::LogLevelI18n(
                                    *self
                                        .log_level_map_to_level
                                        .get(&log_level_selected_i18n)
                                        .unwrap(),
                                ));
                            }
                        }

                        // Insert additional settings above.

                        if !changed_settings.is_empty() {
                            self.changed_settings = Some(changed_settings);
                        }
                    }
                }
            }
            _ => {}
        }
        Ok(command)
    }

    fn view(&self, id: &window::Id) -> Element<ApplicationMessage> {
        #[cfg(feature = "i18n")]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg(not(feature = "i18n"))]
        let align_start = Alignment::Start;

        #[cfg(feature = "i18n")]
        let align_end = self.localisation.script_data.align_words_end;

        #[cfg(not(feature = "i18n"))]
        let align_end = Alignment::End;

        let mut content: Vec<Element<ApplicationMessage>> =
            Vec::<Element<ApplicationMessage>>::new();

        // Preferences - scrollable
        #[allow(unused_mut)]
        let mut preferences: Vec<Element<ApplicationMessage>> =
            Vec::<Element<ApplicationMessage>>::new();

        #[cfg(feature = "i18n")]
        {
            // Language
            let mut setting: Vec<Element<ApplicationMessage>> = vec![
                text(self.localisation.ui_language.as_str()).into(),
                text("").width(Length::Fill).into(),
                combo_box(
                    &self.language_list,
                    self.localisation.placeholder_language.as_str(),
                    self.language_selected.as_ref(),
                    |string| {
                        ApplicationMessage::Preferences(PreferencesMessage::LanguageSelected(
                            string,
                        ))
                    },
                )
                .width(100)
                .into(),
            ];
            if self.localisation.script_data.reverse_words {
                setting.reverse();
            }
            preferences.push(row(setting).into());
        }

        #[cfg(feature = "first_use")]
        let display = !self.first_use;

        #[cfg(not(feature = "first_use"))]
        let display = true;

        if display {
            #[cfg(feature = "log")]
            {
                // Default log level
                let mut setting: Vec<Element<ApplicationMessage>> = vec![
                    text(self.localisation.log_level_default.as_str()).into(),
                    text("").width(Length::Fill).into(),
                    combo_box(
                        &self.log_levels_other, // Could change to log_levels_default to exclude Default variant.
                        self.localisation.placeholder_log_level.as_str(),
                        self.log_level_selected_default.as_ref(),
                        |string| {
                            ApplicationMessage::Preferences(
                                PreferencesMessage::LogLevelSelectedDefault(string),
                            )
                        },
                    )
                    .width(100)
                    .into(),
                ];

                #[cfg(feature = "i18n")]
                if self.localisation.script_data.reverse_words {
                    setting.reverse();
                }

                preferences.push(row(setting).into());

                // Children log levels only appear when default log level is not `off`
                let default = self
                    .log_level_map_to_level
                    .get(self.log_level_selected_default.as_ref().unwrap())
                    .unwrap();
                if *default != LogLevel::Off {
                    // application log level
                    let mut setting: Vec<Element<ApplicationMessage>> = vec![
                        text(self.localisation.log_level_application.as_str()).into(),
                        text("").width(Length::Fill).into(),
                        combo_box(
                            &self.log_levels_other,
                            self.localisation.placeholder_log_level.as_str(),
                            self.log_level_selected_application.as_ref(),
                            |string| {
                                ApplicationMessage::Preferences(
                                    PreferencesMessage::LogLevelSelectedApplication(string),
                                )
                            },
                        )
                        .width(100)
                        .into(),
                    ];

                    #[cfg(feature = "i18n")]
                    if self.localisation.script_data.reverse_words {
                        setting.reverse();
                    }

                    preferences.push(row(setting).into());

                    // other crates log level
                    let mut setting: Vec<Element<ApplicationMessage>> = vec![
                        text(self.localisation.log_level_other.as_str()).into(),
                        text("").width(Length::Fill).into(),
                        combo_box(
                            &self.log_levels_other,
                            self.localisation.placeholder_log_level.as_str(),
                            self.log_level_selected_other.as_ref(),
                            |string| {
                                ApplicationMessage::Preferences(
                                    PreferencesMessage::LogLevelSelectedOther(string),
                                )
                            },
                        )
                        .width(100)
                        .into(),
                    ];

                    #[cfg(feature = "i18n")]
                    if self.localisation.script_data.reverse_words {
                        setting.reverse();
                    }

                    preferences.push(row(setting).into());

                    // iced crate log level
                    let mut setting: Vec<Element<ApplicationMessage>> = vec![
                        text(self.localisation.log_level_iced.as_str()).into(),
                        text("").width(Length::Fill).into(),
                        combo_box(
                            &self.log_levels_other,
                            self.localisation.placeholder_log_level.as_str(),
                            self.log_level_selected_iced.as_ref(),
                            |string| {
                                ApplicationMessage::Preferences(
                                    PreferencesMessage::LogLevelSelectedIced(string),
                                )
                            },
                        )
                        .width(100)
                        .into(),
                    ];

                    #[cfg(feature = "i18n")]
                    if self.localisation.script_data.reverse_words {
                        setting.reverse();
                    }

                    preferences.push(row(setting).into());

                    // i18n crate log level
                    let mut setting: Vec<Element<ApplicationMessage>> = vec![
                        text(self.localisation.log_level_i18n.as_str()).into(),
                        text("").width(Length::Fill).into(),
                        combo_box(
                            &self.log_levels_other,
                            self.localisation.placeholder_log_level.as_str(),
                            self.log_level_selected_i18n.as_ref(),
                            |string| {
                                ApplicationMessage::Preferences(
                                    PreferencesMessage::LogLevelSelectedI18n(string),
                                )
                            },
                        )
                        .width(100)
                        .into(),
                    ];

                    #[cfg(feature = "i18n")]
                    if self.localisation.script_data.reverse_words {
                        setting.reverse();
                    }

                    preferences.push(row(setting).into());
                }
            }
        }

        // Add additional preferences above this comment.

        #[cfg(feature = "i18n")]
        if self.localisation.script_data.reverse_lines {
            preferences.reverse();
        }

        content.push(
            scrollable(
                column(preferences)
                    .width(Length::Fill)
                    .align_items(align_start),
            )
            .width(Length::Fill)
            .height(Length::Fill)
            .into(),
        );
        content.push(" ".into()); // Paragraph separation

        // Buttons
        let mut buttons: Vec<Element<ApplicationMessage>> =
            Vec::<Element<ApplicationMessage>>::new();
        buttons.push(
            button(self.localisation.accept.as_str())
                .padding([5, 10])
                .on_press(ApplicationMessage::Preferences(PreferencesMessage::Accept(
                    *id,
                )))
                .into(),
        );

        #[cfg(feature = "first_use")]
        if !self.first_use {
            buttons.push(
                button(self.localisation.cancel.as_str())
                    .padding([5, 10])
                    .on_press(ApplicationMessage::Preferences(PreferencesMessage::Cancel(
                        *id,
                    )))
                    .into(),
            );
        }

        #[cfg(not(feature = "first_use"))]
        buttons.push(
            button(self.localisation.cancel.as_str())
                .padding([5, 10])
                .on_press(ApplicationMessage::Preferences(PreferencesMessage::Cancel(
                    *id,
                )))
                .into(),
        );

        #[cfg(feature = "i18n")]
        if self.localisation.script_data.reverse_words {
            buttons.reverse();
        }

        content.push(
            column![row(buttons).spacing(10)]
                .width(Length::Fill)
                .align_items(align_end)
                .into(),
        );

        #[cfg(feature = "i18n")]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        event_control::Container::new(column(content).width(Length::Fill), self.enabled)
            .height(Length::Fill)
            .padding(2)
            .into()
    }

    fn parent(&self) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove(&mut self) -> Option<WindowType> {
        self.parent.clone() // Always WindowType::Main, thus just faking remove.
    }

    #[allow(unused_variables)]
    #[cfg(feature = "i18n")]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
        session: &Session,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg(feature = "log")]
            info!("Updating localisation.");

            self.localisation =
                PreferencesLocalisation::try_new(localisation, &self.languages_available)?;

            // Rebuild combox box lists
            #[cfg(feature = "log")]
            {
                (
                    //self.log_levels_default,
                    self.log_levels_other,
                    self.log_level_map_to_level,
                    self.log_level_map_to_string,
                    self.log_level_selected_default,
                    self.log_level_selected_application,
                    self.log_level_selected_other,
                    self.log_level_selected_iced,
                    self.log_level_selected_i18n,
                ) = Self::build_log_combo_boxes(&self.localisation, &self.settings);
            }
        }
        Ok(())
    }

    fn enable(&mut self) {
        self.enabled = true;
    }

    fn disable(&mut self) {
        self.enabled = false;
    }

    fn is_enabled(&self) -> bool {
        self.enabled
    }
}

pub fn display_preferences(
    application: &mut ApplicationThread,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if let hash_map::Entry::Vacant(_e) = application.windows.entry(WindowType::Preferences) {
        #[cfg(feature = "i18n")]
        _e.insert(Box::new(Preferences::try_new(
            &application.localisation,
            &application.session.settings,
            #[cfg(feature = "first_use")]
            false,
        )?));

        #[cfg(not(feature = "i18n"))]
        application.windows.insert(
            WindowType::Preferences,
            Box::new(Preferences::try_new(
                &application.session.settings,
                #[cfg(feature = "first_use")]
                false,
            )?),
        );
    } else {
        let window = application
            .windows
            .get_mut(&WindowType::Preferences)
            .unwrap();
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.update_settings(&application.session.settings);

        #[cfg(feature = "i18n")]
        window.try_update_localisation(
            &application.localisation,
            &application.environment,
            &application.session,
        )?;
    }
    let size = application.session.settings.ui.preferences.size;
    let option = &application.session.settings.ui.preferences.position;
    let position = if option.is_none() {
        window::Position::Centered
    } else {
        let value = option.as_ref().unwrap();
        window::Position::Specific(Point {
            x: value.0,
            y: value.1,
        })
    };
    let settings = window::Settings {
        size: Size::new(size.0, size.1),
        resizable: RESIZABLE,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    application.spawn_with_disable(settings, &WindowType::Preferences, &WindowType::Main)
}

pub fn update_preferences(
    application: &mut ApplicationThread,
    message: ApplicationMessage,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    let mut command = Command::none();
    match message {
        ApplicationMessage::Preferences(ref preferences_message) => {
            let Some(window) = application.windows.get_mut(&WindowType::Preferences) else {
                return Ok(display_fatal_error(
                    application,
                    ApplicationError::WindowTypeNotFound(WindowType::Preferences),
                ));
            };
            command = window.try_update(message.clone())?;

            // Post internal update
            match preferences_message {
                #[cfg(feature = "i18n")]
                PreferencesMessage::LanguageSelected(_string) => {
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                    application.localisation.localiser().defaults(
                        Some(
                            application
                                .localisation
                                .localiser()
                                .language_tag_registry()
                                .tag(actual.language_selected().as_str())
                                .unwrap(),
                        ),
                        None,
                        None,
                    )?;
                    window.try_update_localisation(
                        &application.localisation,
                        &application.environment,
                        &application.session,
                    )?;
                }
                PreferencesMessage::Accept(id) => {
                    #[cfg(feature = "i18n")]
                    let mut localisation_update = false;
                    #[cfg(feature = "log")]
                    let mut logging_update = false;

                    let mut _changed_settings: Option<Vec<Setting>> = None;
                    {
                        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                        _changed_settings = actual.result_vector();

                        #[cfg(feature = "i18n")]
                        if actual.clear_language_changed() {
                            // Reset Preference localisation to settings.ui.language, in case language setting is
                            // not present in Vec<Setting>.
                            application.localisation.localiser().defaults(
                                Some(
                                    application
                                        .localisation
                                        .localiser()
                                        .language_tag_registry()
                                        .tag(actual.language_selected().as_str())
                                        .unwrap(),
                                ),
                                None,
                                None,
                            )?;
                            window.try_update_localisation(
                                &application.localisation,
                                &application.environment,
                                &application.session,
                            )?;
                        }
                    }
                    #[cfg(feature = "log")]
                    debug!("{:?}", _changed_settings);

                    // Handle all the changed settings, where necessary update components that require immediate
                    // effect.
                    if _changed_settings.is_some() {
                        let binding = _changed_settings.unwrap();
                        let iterator = binding.iter();
                        for setting in iterator {
                            match setting {
                                #[cfg(feature = "i18n")]
                                Setting::Language(language) => {
                                    application.session.settings.ui.language = language.clone();
                                    application.localisation.localiser().defaults(
                                        Some(
                                            application
                                                .localisation
                                                .localiser()
                                                .language_tag_registry()
                                                .tag(
                                                    application
                                                        .session
                                                        .settings
                                                        .ui
                                                        .language
                                                        .as_str(),
                                                )
                                                .unwrap(),
                                        ),
                                        None,
                                        None,
                                    )?;
                                    localisation_update = true;
                                }

                                #[cfg(feature = "log")]
                                Setting::LogLevelDefault(log_level) => {
                                    application.session.settings.log_levels.default = *log_level;
                                    trace!(
                                        "Default: {}",
                                        application.session.settings.log_levels.default
                                    );
                                    logging_update = true;
                                }

                                #[cfg(feature = "log")]
                                Setting::LogLevelApplication(log_level) => {
                                    application.session.settings.log_levels.application =
                                        *log_level;
                                    trace!(
                                        "Application: {}",
                                        application.session.settings.log_levels.application
                                    );
                                    logging_update = true;
                                }

                                #[cfg(feature = "log")]
                                Setting::LogLevelOther(log_level) => {
                                    application.session.settings.log_levels.other = *log_level;
                                    trace!(
                                        "Other: {}",
                                        application.session.settings.log_levels.other
                                    );
                                    logging_update = true;
                                }

                                #[cfg(feature = "log")]
                                Setting::LogLevelIced(log_level) => {
                                    application.session.settings.log_levels.iced = *log_level;
                                    trace!(
                                        "iced: {}",
                                        application.session.settings.log_levels.iced
                                    );
                                    logging_update = true;
                                }

                                #[cfg(feature = "log")]
                                Setting::LogLevelI18n(log_level) => {
                                    application.session.settings.log_levels.i18n = *log_level;
                                    trace!(
                                        "i18n: {}",
                                        application.session.settings.log_levels.i18n
                                    );
                                    logging_update = true;
                                }

                                #[allow(unreachable_patterns)]
                                _ => {}
                            }
                        }
                    }

                    #[cfg(feature = "i18n")]
                    if localisation_update {
                        //loop through all windows to update localisation
                        let iterator = application.windows.iter_mut();
                        for (_window_type, window) in iterator {
                            window.try_update_localisation(
                                &application.localisation,
                                &application.environment,
                                &application.session,
                            )?;
                        }
                    }

                    #[cfg(feature = "log")]
                    if logging_update {
                        update_logger(
                            &mut application.environment.logger,
                            &application.session.settings.log_levels,
                        )
                    }

                    command = close(application, *id)?
                }
                PreferencesMessage::Cancel(id) => {
                    #[cfg(feature = "i18n")]
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                    #[cfg(feature = "i18n")]
                    if actual.clear_language_changed() {
                        // Reset Preference localisation to settings.ui.language.
                        application.localisation.localiser().defaults(
                            Some(
                                application
                                    .localisation
                                    .localiser()
                                    .language_tag_registry()
                                    .tag(application.session.settings.ui.language.as_str())
                                    .unwrap(),
                            ),
                            None,
                            None,
                        )?;
                        window.try_update_localisation(
                            &application.localisation,
                            &application.environment,
                            &application.session,
                        )?;
                    }

                    command = application.close(*id)?
                }
                #[allow(unreachable_patterns)]
                _ => {}
            }
        }
        _ => {}
    }
    Ok(command)
}

pub fn close(
    application: &mut ApplicationThread,
    id: window::Id,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if id != window::Id::MAIN {
        return application.close(id);
    }

    #[cfg(feature = "first_use")]
    {
        let Some(window) = application.windows.get_mut(&WindowType::Preferences) else {
            return Ok(display_fatal_error(
                application,
                ApplicationError::WindowTypeNotFound(WindowType::Preferences),
            ));
        };
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.end_first_use();
    }

    application.windows.insert(
        WindowType::Main,
        Box::new(Main::try_new(
            #[cfg(feature = "i18n")]
            &application.localisation,
        )?),
    );
    application
        .window_ids
        .insert(window::Id::MAIN, WindowType::Main);
    let size = application.session.settings.ui.main.size;
    let commands = vec![window::resize(
        window::Id::MAIN,
        Size {
            width: size.0,
            height: size.1,
        },
    )];
    Ok(Command::batch(commands))
}
