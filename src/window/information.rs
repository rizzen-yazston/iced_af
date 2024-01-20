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
    widget::{ button, column, container, text },
    alignment,
    Alignment,
    Element,
    Length,
    Size,
    Point,
};
use log::error;
use std::{
    collections::HashMap,
    any::Any,
};

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 400f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 400f32, 300f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

struct InformationLocalisation {
    language: RefCount<String>,

    // Strings
    information: TaggedString,
    error: TaggedString,
    warning: TaggedString,
    close: TaggedString,

    // Dynamic strings
    title: TaggedString,
    message: TaggedString,
}

impl InformationLocalisation {
    pub fn try_new(
        localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        let language = localisation.localiser().default_language();
        Ok( InformationLocalisation {
            information: localisation.localiser().literal_with_defaults(
                "word", "information_i"
            )?,
            language: RefCount::clone( &language ),
            error: localisation.localiser().literal_with_defaults(
                "word", "error_i"
            )?,
            warning: localisation.localiser().literal_with_defaults(
                "word", "warn_i"
            )?,
            close: localisation.localiser().literal_with_defaults(
                "word", "close_i"
            )?,
            title: TaggedString::new( "", &language ),
            message: TaggedString::new( "", &language ),
        } )
    }
}

pub struct Information {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: InformationLocalisation,
    information_type: InformationType,
    application_short_name: String,
}

impl Information {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let localisation = InformationLocalisation::try_new( localisation )?;
        Ok( Information {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation,
            information_type: InformationType::Information,
            application_short_name: environment.application_short_name.clone(),
        } )
    }

    pub fn information(
        &mut self,
        parent: &WindowType,
        title: TaggedString,
        message: TaggedString,
        localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Information;
        self.localisation.message = message;
        let name = localisation.localiser().literal_with_defaults(
            "application", title.as_str()
        )?;
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String( self.application_short_name.clone() ),
        );
        values.insert(
            "type".to_string(), 
            PlaceholderValue::TaggedString( self.localisation.information.clone() ),
        );
        values.insert(
            "window".to_string(), 
            PlaceholderValue::TaggedString( name ),
        );
        self.localisation.title = localisation.localiser().format_with_defaults(
            "application",
            "window_type_title_format",
            &values
        )?;
        Ok( () )
    }

    pub fn warning(
        &mut self,
        parent: &WindowType,
        title: TaggedString,
        message: TaggedString,
        localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Information;
        self.localisation.message = message;
        let name = localisation.localiser().literal_with_defaults(
            "application", title.as_str()
        )?;
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String( self.application_short_name.clone() ),
        );
        values.insert(
            "type".to_string(), 
            PlaceholderValue::TaggedString( self.localisation.warning.clone() ),
        );
        values.insert(
            "window".to_string(), 
            PlaceholderValue::TaggedString( name ),
        );
        self.localisation.title = localisation.localiser().format_with_defaults(
            "application",
            "window_type_title_format",
            &values
        )?;
        Ok( () )
    }

    pub fn error(
        &mut self,
        parent: &WindowType,
        title: TaggedString,
        message: TaggedString,
        localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Information;
        self.localisation.message = message;
        let name = localisation.localiser().literal_with_defaults(
            "application", title.as_str()
        )?;
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "application".to_string(),
            PlaceholderValue::String( self.application_short_name.clone() ),
        );
        values.insert(
            "type".to_string(), 
            PlaceholderValue::TaggedString( self.localisation.error.clone() ),
        );
        values.insert(
            "window".to_string(), 
            PlaceholderValue::TaggedString( name ),
        );
        self.localisation.title = localisation.localiser().format_with_defaults(
            "application",
            "window_type_title_format",
            &values
        )?;
        Ok( () )
    }
}

impl AnyWindowTrait for Information {}

impl WindowTrait for Information {
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
            text( self.localisation.title.as_str() ),
            text( self.localisation.message.as_str() ),
            button( self.localisation.close.as_str() )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Close( id.clone() ) ),
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
        self.parent.take()
    }

    #[allow(unused_variables)]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            error!( "Updating localisation." );
            self.localisation = InformationLocalisation::try_new( localisation )?;
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

#[allow( dead_code )]
#[derive( PartialEq )]
pub enum InformationType {
    Error,
    Warning,
    Information, // General purpose
}

pub fn display_information(
    application: &mut ApplicationThread,
    title: TaggedString,
    message: TaggedString,
    information_type: InformationType,
    parent: WindowType,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if !application.windows.contains_key( &WindowType::Information ) {
        application.windows.insert(
            WindowType::Information,
            Box::new( Information::try_new( &application.localisation, &application.environment )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::Information ).unwrap();
        window.try_update_localisation( &application.localisation, &application.environment, )?;
    }
    {
        let window = application.windows.get_mut( &WindowType::Information ).unwrap();
        let actual = window.as_any_mut().downcast_mut::<Information>().unwrap();
        match information_type {
            InformationType::Information => actual.information( &parent, title, message, &application.localisation )?,
            InformationType::Warning => actual.warning( &parent, title, message, &application.localisation )?,
            InformationType::Error => actual.error( &parent, title, message, &application.localisation )?,
        }
    }
    let size = application.session.settings.ui.information.size;
    let option = &application.session.settings.ui.information.position;
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
    application.spawn_with_disable( settings, &WindowType::Information, &WindowType::Main )
}
