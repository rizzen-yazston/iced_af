// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
        core::{
        application::{
            WindowType,
            ApplicationMessage,
            ApplicationThread,
        },
        error::ApplicationError,
        traits::{ AnyWindowTrait, WindowTrait },
    },
    APPLICATION_NAME_SHORT,
};
use iced::{
    window,
    Command,
    widget::{ button, column, container, text, scrollable },
    Alignment,
    Element,
    Length,
    Size,
    Point,
};
use std::any::Any;

#[cfg( feature = "i18n" )]
use crate::core::{
    localisation::{
        Localisation,
        ScriptData,
    },
    environment::Environment,
};

#[cfg( feature = "i18n" )]
use i18n::utility::{ TaggedString as LString, PlaceholderValue, };

#[cfg( not( feature = "i18n" ) )]
use std::string::String as LString;

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

#[cfg( feature = "i18n" )]
use std::collections::HashMap;

#[cfg( all( feature = "i18n", feature = "sync" ) )]
use std::sync::Arc as RefCount;

#[cfg( all( feature = "i18n", not( feature = "sync" ) ) )]
use std::rc::Rc as RefCount;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 400f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 600f32, 200f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

struct InformationLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    information: LString,
    error: LString,
    warning: LString,
    close: LString,

    // Dynamic strings
    title: LString,
    message: LString,
}

impl InformationLocalisation {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        #[cfg( feature = "i18n" )]
        let language = localisation.localiser().default_language();

        #[cfg( feature = "i18n" )]
        let locale = localisation.localiser().language_tag_registry().locale(
            language.as_str()
        )?;

        #[cfg( feature = "i18n" )]
        let information = localisation.localiser().literal_with_defaults(
            "word", "information_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let information = "Information".to_string();

        #[cfg( feature = "i18n" )]
        let warning = localisation.localiser().literal_with_defaults(
            "word", "warning_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let warning = "Warning".to_string();

        #[cfg( feature = "i18n" )]
        let error = localisation.localiser().literal_with_defaults(
            "word", "error_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let error = "Error".to_string();

        #[cfg( feature = "i18n" )]
        let close = localisation.localiser().literal_with_defaults(
            "word", "close_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let close = "Close".to_string();

        #[cfg( feature = "i18n" )]
        let title = LString::new( "", &language );

        #[cfg( not( feature = "i18n" ) )]
        let title = "".to_string();

        #[cfg( feature = "i18n" )]
        let message = LString::new( "", &language );

        #[cfg( not( feature = "i18n" ) )]
        let message = "".to_string();

        Ok( InformationLocalisation {
            #[cfg( feature = "i18n" )] language,
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
            information,
            error,
            warning,
            close,
            title,
            message,
        } )
    }
}

pub struct Information {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: InformationLocalisation,
    information_type: InformationType,
}

impl Information {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        let localisation = InformationLocalisation::try_new(
            #[cfg( feature = "i18n" )] localisation
        )?;
        Ok( Information {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation,
            information_type: InformationType::Information,
        } )
    }
    
    pub fn information(
        &mut self,
        parent: &WindowType,
        title: LString,
        message: LString,
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Information;
        self.localisation.message = message;

        #[cfg( feature = "i18n" )]
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "type".to_string(), 
                PlaceholderValue::TaggedString( self.localisation.information.clone() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString( title ),
            );
            self.localisation.title = localisation.localiser().format_with_defaults(
                "application",
                "window_type_title_format",
                &values
            )?;
        }

        #[cfg( not( feature = "i18n" ) )]
        {
            self.localisation.title = format!( "{} - Information: {}", APPLICATION_NAME_SHORT, title );
        }

        Ok( () )
    }

    pub fn warning(
        &mut self,
        parent: &WindowType,
        title: LString,
        message: LString,
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Warning;
        self.localisation.message = message;

        #[cfg( feature = "i18n" )]
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "type".to_string(), 
                PlaceholderValue::TaggedString( self.localisation.warning.clone() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString( title ),
            );
            self.localisation.title = localisation.localiser().format_with_defaults(
                "application",
                "window_type_title_format",
                &values
            )?;
        }

        #[cfg( not( feature = "i18n" ) )]
        {
            self.localisation.title = format!( "{} - Warning: {}", APPLICATION_NAME_SHORT, title );
        }

        Ok( () )
    }

    pub fn error(
        &mut self,
        parent: &WindowType,
        title: LString,
        message: LString,
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<(), ApplicationError> {
        self.parent = Some( parent.clone() );
        self.information_type = InformationType::Error;
        self.localisation.message = message;

        #[cfg( feature = "i18n" )]
        {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "type".to_string(), 
                PlaceholderValue::TaggedString( self.localisation.error.clone() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString( title ),
            );
            self.localisation.title = localisation.localiser().format_with_defaults(
                "application",
                "window_type_title_format",
                &values
            )?;
        }

        #[cfg( not( feature = "i18n" ) )]
        {
            self.localisation.title = format!( "{} - Error: {}", APPLICATION_NAME_SHORT, title );
        }

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

    fn title( &self ) -> &LString {
        &self.localisation.title
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        #[cfg( feature = "i18n" )]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg( not( feature = "i18n" ) )]
        let align_start = Alignment::Start;

        let mut content: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        // Message
        content.push(
            scrollable(
                column![ text( self.localisation.message.as_str() ) ]
                .width( Length::Fill )
                .align_items( align_start )
            ).width( Length::Fill ).height( Length::Fill ).into()
        );
        content.push( " ".into() ); // Paragraph separation

        // Close button
        content.push( column![
            button( text( self.localisation.close.as_str() ) )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Close( id.clone() ) )
        ].width( Length::Fill ).align_items( Alignment::Center ).into() );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        container( column( content ).width( Length::Fill ) ).height( Length::Fill ).padding( 2 )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove( &mut self ) -> Option<WindowType> {
        self.parent.take()
    }

    #[allow(unused_variables)]
    #[cfg( feature = "i18n" )]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg( feature = "log" )]
            info!( "Updating localisation." );

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
    title: LString,
    message: LString,
    information_type: InformationType,
    parent: WindowType,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if !application.windows.contains_key( &WindowType::Information ) {
        
        application.windows.insert(
            WindowType::Information,
            Box::new( Information::try_new(
                #[cfg( feature = "i18n" )] &application.localisation,
            )? )
        );
    } else {
        #[cfg( feature = "i18n" )]
        {
            let window = application.windows.get_mut( &WindowType::Information ).unwrap();
            window.try_update_localisation( &application.localisation, &application.environment, )?;
        }
    }
    {
        let window = application.windows.get_mut( &WindowType::Information ).unwrap();
        let actual = window.as_any_mut().downcast_mut::<Information>().unwrap();
        match information_type {
            InformationType::Information => actual.information( 
                &parent,
                title,
                message,
                #[cfg( feature = "i18n" )] &application.localisation
            )?,
            InformationType::Warning => actual.warning(
                &parent,
                title,
                message,
                #[cfg( feature = "i18n" )] &application.localisation
            )?,
            InformationType::Error => actual.error(
                &parent,
                title,
                message,
                #[cfg( feature = "i18n" )] &application.localisation
            )?,
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
    application.spawn_with_disable( settings, &WindowType::Information, &parent )
}
