// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::core::error::ApplicationError;
use iced::{
    alignment,
    widget::{button, container, text},
    Element, Renderer, Theme,
};
use iced_aw::{
    menu::{Item, Menu},
    menu_bar, menu_items,
};

#[cfg(feature = "i18n")]
use std::collections::HashMap;

#[cfg(feature = "i18n")]
use crate::core::{
    environment::Environment,
    localisation::{Localisation, ScriptData},
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

#[derive(Debug, Clone)]
pub enum MainMenuBarMessage {
    None, // Used for the menu bar button, and buttons that open sub menus to the side.
    Exit,
    Preferences,
    About,
}

pub struct MainMenuBarLocalisation {
    #[cfg(feature = "i18n")]
    language: RefCount<LanguageTag>,
    #[cfg(feature = "i18n")]
    script_data: ScriptData,

    // Strings
    file: LString,
    exit: LString,
    edit: LString,
    preferences: LString,
    help: LString,
    about: LString,
}

impl MainMenuBarLocalisation {
    pub fn try_new(
        #[cfg(feature = "i18n")] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        #[cfg(feature = "i18n")]
        let language = localisation.localiser().default_language();

        #[cfg(feature = "i18n")]
        let locale = localisation
            .localiser()
            .language_tag_registry()
            .identifier(language.as_str())?;

        // File menu
        #[cfg(feature = "i18n")]
        let file = localisation
            .localiser()
            .literal_with_defaults("word", "file_i")?;

        #[cfg(not(feature = "i18n"))]
        let file = "File".to_string();

        #[cfg(feature = "i18n")]
        let exit = {
            #[cfg(target_os = "macos")]
            {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert(
                    "short_name".to_string(),
                    PlaceholderValue::String(APPLICATION_NAME_SHORT.to_string()),
                );
                localisation.localiser().format_with_defaults(
                    "application",
                    "quit_macos",
                    &values,
                )?
            }

            #[cfg(not(target_os = "macos"))]
            localisation
                .localiser()
                .literal_with_defaults("word", "exit_i")?
        };

        #[cfg(not(feature = "i18n"))]
        let exit = {
            #[cfg(target_os = "macos")]
            format!("Quit {}", APPLICATION_NAME_SHORT);

            #[cfg(not(target_os = "macos"))]
            "Exit".to_string()
        };

        // Edit menu
        #[cfg(feature = "i18n")]
        let edit = localisation
            .localiser()
            .literal_with_defaults("word", "edit_i")?;

        #[cfg(not(feature = "i18n"))]
        let edit = "Edit".to_string();

        #[cfg(feature = "i18n")]
        let preferences = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "phrase".to_string(),
                PlaceholderValue::TaggedString(
                    localisation
                        .localiser()
                        .literal_with_defaults("word", "preferences_i")?,
                ),
            );
            localisation.localiser().format_with_defaults(
                "application",
                "add_elipsis_format",
                &values,
            )?
        };

        #[cfg(not(feature = "i18n"))]
        let preferences = "Preferences…".to_string();

        // Help menu
        #[cfg(feature = "i18n")]
        let help = localisation
            .localiser()
            .literal_with_defaults("word", "help_i")?;

        #[cfg(not(feature = "i18n"))]
        let help = "Help".to_string();

        #[cfg(feature = "i18n")]
        let about = localisation
            .localiser()
            .literal_with_defaults("word", "about_i")?;

        #[cfg(not(feature = "i18n"))]
        let about = "About".to_string();

        Ok(MainMenuBarLocalisation {
            #[cfg(feature = "i18n")]
            language,
            #[cfg(feature = "i18n")]
            script_data: ScriptData::new(localisation, &locale),
            file,
            exit,
            edit,
            preferences,
            help,
            about,
        })
    }
}

pub struct MainMenuBar {
    localisation: MainMenuBarLocalisation,
}

impl MainMenuBar {
    pub fn try_new(
        #[cfg(feature = "i18n")] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        Ok(MainMenuBar {
            localisation: MainMenuBarLocalisation::try_new(
                #[cfg(feature = "i18n")]
                localisation,
            )?,
        })
    }

    #[cfg(feature = "i18n")]
    pub fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        _environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg(feature = "log")]
            info!("Updating localisation.");

            self.localisation = MainMenuBarLocalisation::try_new(localisation)?;
        }
        Ok(())
    }

    pub fn view(&self) -> Element<MainMenuBarMessage> {
        let menu_type_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);
        let bar = menu_bar!(
            // File menu
            (
                labeled_button(self.localisation.file.as_str(), MainMenuBarMessage::None),
                menu_type_1(menu_items!(
                    (labeled_button(self.localisation.exit.as_str(), MainMenuBarMessage::Exit))
                ))
            )

            // Edit menu
            (
                labeled_button(self.localisation.edit.as_str(), MainMenuBarMessage::None),
                menu_type_1(menu_items!(
                    (labeled_button(self.localisation.preferences.as_str(), MainMenuBarMessage::Preferences))
                ))
            )

            // Help menu
            (
                labeled_button(self.localisation.help.as_str(), MainMenuBarMessage::None),
                menu_type_1(menu_items!(
                    (labeled_button(self.localisation.about.as_str(), MainMenuBarMessage::About))
                ))
            )
        );
        container(bar).into()
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, MainMenuBarMessage, Theme, Renderer>>,
    message: MainMenuBarMessage,
) -> button::Button<'a, MainMenuBarMessage, Theme, Renderer> {
    button(content).padding([4, 8]).on_press(message)
}

fn labeled_button<'a>(
    label: &str,
    message: MainMenuBarMessage,
) -> button::Button<'a, MainMenuBarMessage, Theme, Renderer> {
    base_button(
        text(label).vertical_alignment(alignment::Vertical::Center),
        message,
    )
}
