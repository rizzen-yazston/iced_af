// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    application::{ApplicationMessage, WindowType},
    error::ApplicationError,
};
use iced::{window, Command, Element, Renderer, Theme};
use std::any::Any;

#[cfg(feature = "i18n")]
use super::{environment::Environment, localisation::Localisation, session::Session};

#[cfg(feature = "log")]
#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[cfg(feature = "i18n")]
use i18n::utility::TaggedString as LString;

#[cfg(not(feature = "i18n"))]
use std::string::String as LString;

/// Supertrait for Any
pub trait AnyWindowTrait: Any + WindowTrait {}

/// Trait for basic window methods.
pub trait WindowTrait {
    fn as_any(&self) -> &dyn Any;

    fn as_any_mut(&mut self) -> &mut dyn Any;

    fn title(&self) -> &LString;

    #[allow(unused_variables)]
    fn try_update(
        &mut self,
        message: ApplicationMessage,
    ) -> Result<Command<ApplicationMessage>, ApplicationError> {
        Ok(Command::none())
    }

    fn view(&self, id: &window::Id) -> Element<'_, ApplicationMessage, Theme, Renderer>;

    fn scale_factor(&self) -> f64 {
        1.0
    }

    fn parent(&self) -> &Option<WindowType>;

    // Some windows don't have varying parent, thus does nothing.
    #[allow(unused_variables)]
    fn parent_add(&mut self, window_type: &WindowType) {}

    // Some windows don't have varying parent, thus does nothing.
    fn parent_remove(&mut self) -> Option<WindowType> {
        None
    }

    // Some windows don't update their localisation.
    #[cfg(feature = "i18n")]
    #[allow(unused_variables)]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
        session: &Session,
    ) -> Result<(), ApplicationError> {
        Ok(())
    }

    fn enable(&mut self);

    fn disable(&mut self);

    fn is_enabled(&self) -> bool;
}
