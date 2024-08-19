// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    application::{
        constants::{TAB_HEADER_SIZE, TAB_PADDING},
        error::ApplicationError,
        Message, WindowType,
    },
    core::{
        error::CoreError,
        localisation::{Localisation, StringCache},
    },
    iced_aw::widgets::sidebar::TabLabel,
};
use core::fmt::Debug;
use i18n::utility::LanguageTag;
use iced::{
    alignment::{Horizontal, Vertical},
    widget::{Column, Container, Text},
    window, Task, Element, Length, Renderer, Theme,
};
//use iced_aw::sidebar::TabLabel;
use std::{
    any::Any,
    rc::Rc as RefCount,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

//
// ----- Window state traits
//

/// Supertrait for Any
pub trait AnyWindowTrait: Any + WindowTrait {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;
}

/// Trait for basic window methods.
pub trait WindowTrait {
    // `iced` specific methods
    /// Returns a string for the window title bar (contains the windows decorations).
    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String;

    /// The update method called by the applications main `update()` function to handle messages
    /// and update the state. Also supports errors being returned.
    #[allow(unused_variables)]
    fn try_update(
        &mut self,
        message: Message,
        string_cache: &StringCache,
    ) -> Result<Task<Message>, ApplicationError> {
        Ok(Task::none())
    }

    /// The view method called by the applications main `view()` function to build the window
    /// of widgets.
    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, Message, Theme, Renderer>;

    /// The scaling factor to be used for the window.
    fn scale_factor(&self) -> f64 {
        1.0
    }

    // `iced_af` specific methods
    /// Obtains the type of the window.
    fn window_type(&self) -> WindowType;

    /// Indicates whether the state can be reused, in that it can be cached for later reuse,
    /// thus saving the need to recreate the state. Useful for frequent used windows.
    fn is_reusable(&self) -> bool {
        false
    }

    /// Indicates whether the window being displayed disables all windows or just the parent
    /// window of the window thread.
    fn is_global_disable(&self) -> bool {
        false
    }

    /// Try to update dynamic localised strings stored in the state itself.
    /// 
    /// Note: All data must be present within the state, that is required for the updating
    /// of the localised strings 
    #[allow(unused_variables)]
    fn try_localise(
        &mut self,
        localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }
}

/// Trait of methods to be implemented for window states having saveable data.
pub trait SaveDataTrait {
    /// Instruct the window state to save the data of the state.
    fn try_save(&mut self) -> Result<(), ApplicationError>;

    /// Indicates whether there is unsaved data of the state.
    fn is_unsaved(&self) -> bool;

    /// Identifier for the window. Usually it is a filename or database name.
    fn name(&self) -> &str;
}

//
// ----- Localisation traits
//

/// Supertrait for Any
pub trait AnyLocalisedTrait: Any + LocalisedTrait + Debug {
    // Used to obtain a vec of strings, such as combo box selection list.
    fn as_any(&self) -> &dyn Any;
}

/// Trait for localisations of the windows.
pub trait LocalisedTrait {
    #[allow(unused_variables)]
    fn try_update(&mut self, localisation: &Localisation) -> Result<(), CoreError> {
        Ok(())
    }

    fn title(&self) -> &String;

    fn string(&self, index: usize) -> &String;

    fn language_tag(&self) -> &RefCount<LanguageTag>;
}

//
// ----- Tab traits
//

pub trait TabTrait {
    fn title<'a>(&self, string_cache: &'a StringCache) -> String;

    fn tab_label<'a>(&self, string_cache: &'a StringCache) -> TabLabel {
        TabLabel::Text(self.title(string_cache))
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, Message, Theme, Renderer> {
        let column = Column::new()
            .spacing(20)
            .push(Text::new(self.title(string_cache)).size(TAB_HEADER_SIZE))
            .push(self.content(id, localisation, string_cache))
            .align_x(iced::Alignment::Center);
        Container::new(column)
            .width(Length::Fill)
            .height(Length::Fill)
            .align_x(Horizontal::Center)
            .align_y(Vertical::Center)
            .padding(TAB_PADDING)
            .into()
    }

    fn content<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, Message, Theme, Renderer>;
}
