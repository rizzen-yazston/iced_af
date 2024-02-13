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
    widget::{ button, column, row, container, text },
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
//const SIZE_MIN: ( f32, f32 ) = ( 150f32, 100f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 320f32, 120f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct ConfirmExitLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    title: LString,
    confirm_exit: LString,
    exit: LString,
    cancel: LString,
}

impl ConfirmExitLocalisation {
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
        let title = {
            #[cfg( target_os = "macos" )]
            let name = localisation.localiser().literal_with_defaults(
                "application", "confirm_quit",
            )?;
    
            #[cfg( not( target_os = "macos" ) )]
            let name = localisation.localiser().literal_with_defaults(
                "application", "confirm_exit",
            )?;
    
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString( name )
            );
            localisation.localiser().format_with_defaults(
                "application", "window_title_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let title = {
            #[cfg(  target_os = "macos" )]
            let name = "Confirm quit".to_string();
    
            #[cfg( not( target_os = "macos" ) )]
            let name = "Confirm exit".to_string();
    
            format!(
                "{} - {}",
                APPLICATION_NAME_SHORT,
                name,
            )    
        };

        #[cfg( feature = "i18n" )]
        let confirm_exit = {
            #[cfg(  target_os = "macos" )]
            {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert(
                    "short_name".to_string(),
                    PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
                );
                localisation.localiser().format_with_defaults(
                    "application", "confirm_quit_question", &values
                )?
            }

            #[cfg( not( target_os = "macos" ) )]
            localisation.localiser().literal_with_defaults(
                "application", "confirm_exit_question"
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let confirm_exit = {
            #[cfg(  target_os = "macos" )]
            format!( "Are you sure you want to quit {}", APPLICATION_NAME_SHORT );

            #[cfg( not( target_os = "macos" ) )]
            "Are you sure you want to exit?".to_string()
        };

        #[cfg( feature = "i18n" )]
        let exit = {
            #[cfg(  target_os = "macos" )]
            {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                values.insert(
                    "short_name".to_string(),
                    PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
                );
                localisation.localiser().format_with_defaults(
                    "application", "quit_macos", &values
                )?
            }

            #[cfg( not( target_os = "macos" ) )]
            localisation.localiser().literal_with_defaults(
                "word", "exit_i"
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let exit = {
            #[cfg(  target_os = "macos" )]
            format!( "Quit {}", APPLICATION_NAME_SHORT );

            #[cfg( not( target_os = "macos" ) )]
            "Exit".to_string()
        };

        #[cfg( feature = "i18n" )]
        let cancel = localisation.localiser().literal_with_defaults(
            "word", "cancel_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let cancel = "Cancel".to_string();

        Ok( ConfirmExitLocalisation {
            #[cfg( feature = "i18n" )] language,
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
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
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        let localisation = ConfirmExitLocalisation::try_new(
            #[cfg( feature = "i18n" )] localisation,
        )?;
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

    fn title( &self ) -> &LString {
        &self.localisation.title
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        /*
        #[cfg( feature = "i18n" )]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg( not( feature = "i18n" ) )]
        let align_start = Alignment::Start;
        */

        let mut content: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        // Message
        content.push(
            column![ text( self.localisation.confirm_exit.as_str() ) ]
            .width( Length::Fill )
            .align_items( Alignment::Center )
            .into()
        );
        content.push( text( " " ).height( Length::Fill ).into() ); // Paragraph separation

        // Buttons
        let mut buttons: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();
        buttons.push(
            button( text( self.localisation.exit.as_str() ) )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Exit )
            .into()
        );
        buttons.push(
            button( text( self.localisation.cancel.as_str() ) )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Close( id.clone() ) )
            .into()
        );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_words {
            buttons.reverse();
        }

        content.push(
            column![ row( buttons ).spacing( 10 ) ]
            .width( Length::Fill )
            .align_items( Alignment::Center )
            .into()
        );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        container( column( content )
        .width( Length::Fill ) )
        .height( Length::Fill )
        .padding( 20 )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove( &mut self ) -> Option<WindowType> {
        self.parent.clone() // Always WindowType::Main, thus just faking remove.
    }

    #[cfg( feature = "i18n" )]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        _environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg( feature = "log" )]
            info!( "Updating localisation." );

            self.localisation = ConfirmExitLocalisation::try_new( localisation, )?;
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
    #[cfg( feature = "log" )] message: &str
) -> Result<Command<ApplicationMessage>, ApplicationError> {

    #[cfg( feature = "log" )]
    info!( "{}", message );

    if !application.windows.contains_key( &WindowType::ConfirmExit ) {
        application.windows.insert(
            WindowType::ConfirmExit,
            Box::new( ConfirmExit::try_new(
                #[cfg( feature = "i18n" )] &application.localisation,
            )? )
        );

    } else {
        #[cfg( feature = "i18n" )]
        {
            let window = application.windows.get_mut( &WindowType::ConfirmExit ).unwrap();
            window.try_update_localisation( &application.localisation, &application.environment, )?;
        }
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
