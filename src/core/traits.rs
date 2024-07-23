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
    fn window_type(&self) -> WindowType;

    fn title<'a>(&'a self, string_cache: &'a StringCache) -> &String;

    #[allow(unused_variables)]
    fn try_update(
        &mut self,
        message: Message,
        string_cache: &StringCache,
    ) -> Result<Task<Message>, ApplicationError> {
        Ok(Task::none())
    }

    fn view<'a>(
        &'a self,
        id: window::Id,
        localisation: &Localisation,
        string_cache: &'a StringCache,
    ) -> Element<'_, Message, Theme, Renderer>;

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn reusable(&self) -> bool {
        false
    }

    fn global_disable(&self) -> bool {
        false
    }
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
