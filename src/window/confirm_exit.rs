// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::core::{
    application::{
        WindowType,
        ApplicationMessage,
        ApplicationThread,
    },
    error::ApplicationError,
    localisation::Localisation,
    environment::Environment,
    traits::{ AnyWindowTrait, WindowTrait },
};
use i18n::{
    pattern::PlaceholderValue,
    utility::TaggedString
};
use iced::{
    window,
    Command,
    widget::{ button, column, row, container, text },
    alignment,
    Alignment,
    Element,
    Length,
    Size,
    Point,
};
use log::{ error, info };
use std::{
    collections::HashMap,
    any::Any,
};

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 150f32, 100f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 300f32, 250f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct ConfirmExitLocalisation {
    language: RefCount<String>,

    // Strings
    title: TaggedString,
    confirm_exit: TaggedString,
    exit: TaggedString,
    cancel: TaggedString,
}

impl ConfirmExitLocalisation {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let language = localisation.localiser().default_language();
        let name = if cfg!( target_os = "macos" ) {
            localisation.localiser().literal_with_defaults(
                "application", "confirm_quit",
            )?
        } else {
            localisation.localiser().literal_with_defaults(
                "application", "confirm_exit"
            )?
        };
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String( environment.application_short_name.clone() ),
        );
        values.insert(
            "window".to_string(), 
            PlaceholderValue::TaggedString( name )
        );
        let title = localisation.localiser().format_with_defaults(
            "application",
            "window_title_format",
            &values
        )?;
        let confirm_exit = if cfg!( target_os = "macos" ) {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "short_name".to_string(),
                PlaceholderValue::String( environment.application_short_name.clone() ),
            );
            localisation.localiser().format_with_defaults(
                "application", "confirm_quit_question", &values
            )?
        } else {
            localisation.localiser().literal_with_defaults(
                "application", "confirm_exit_question"
            )?
        };
        let exit = if cfg!( target_os = "macos" ) {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "short_name".to_string(),
                PlaceholderValue::String( environment.application_short_name.clone() ),
            );
            localisation.localiser().format_with_defaults(
                "application", "quit_macos", &values
            )?
        } else {
            localisation.localiser().literal_with_defaults(
                "word", "exit_i"
            )?
        };
        let cancel = localisation.localiser().literal_with_defaults(
            "word", "cancel_i"
        )?;
        Ok( ConfirmExitLocalisation {
            language,
            title,
            confirm_exit,
            exit,
            cancel            
        } )
    }
}

pub struct ConfirmExit {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: ConfirmExitLocalisation,
}

impl ConfirmExit {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let localisation = ConfirmExitLocalisation::try_new( localisation, environment )?;
        Ok( ConfirmExit {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation,
        } )
    }
}

impl AnyWindowTrait for ConfirmExit {}

impl WindowTrait for ConfirmExit {
    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }

    fn title( &self ) -> &TaggedString {
        &self.localisation.title
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        let content = column![
            column![
                text( self.localisation.confirm_exit.as_str() )
            ].width( Length::Fill ).align_items( Alignment::Start ),
            column![ row![
                button( text( self.localisation.exit.as_str() ) )
                .padding( [ 5, 10 ] )
                .on_press( ApplicationMessage::Exit ),
                button( text( self.localisation.cancel.as_str() ) )
                .padding( [ 5, 10 ] )
                .on_press( ApplicationMessage::Close( id.clone() ) ),
            ].spacing( 10 ) ].width( Length::Fill ).align_items( Alignment::Center ),//End
        ].spacing( 10 ).align_items( Alignment::Center );
        container( container( content ).width( 510 ) )
        .align_x( alignment::Horizontal::Center )
        .align_y( alignment::Vertical::Center )
        .width( Length::Fill )
        .height( Length::Fill )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove( &mut self ) -> Option<WindowType> {
        self.parent.clone() // Always WindowType::Main, thus just faking remove.
    }

    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            error!( "Updating localisation." );
            self.localisation = ConfirmExitLocalisation::try_new( localisation, environment )?;
        }
        Ok( () )
    }

    fn enable( &mut self ) {
        self.enabled = true;
    }

    fn disable( &mut self ) {
        self.enabled = false;
    }

    fn is_enabled( &self ) -> bool {
        self.enabled
    }
}

pub fn display_confirm_exit(
    application: &mut ApplicationThread,
    message: &str
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    info!( "{}", message );
    if !application.windows.contains_key( &WindowType::ConfirmExit ) {
        application.windows.insert(
            WindowType::ConfirmExit,
            Box::new( ConfirmExit::try_new( &application.localisation, &application.environment )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::ConfirmExit ).unwrap();
        window.try_update_localisation( &application.localisation, &application.environment, )?;
    }
    let size = application.session.settings.ui.confirm_exit.size;
    let option = &application.session.settings.ui.confirm_exit.position;
    let position = if option.is_none() {
        window::Position::Centered
    } else {
        let value = option.as_ref().unwrap();
        window::Position::Specific( Point { x: value.0, y: value.1 } )
    };
    let settings = window::Settings {
        size: Size::new( size.0, size.1 ),
        resizable: RESIZABLE,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    application.spawn_with_disable( settings, &WindowType::ConfirmExit, &WindowType::Main )
}
