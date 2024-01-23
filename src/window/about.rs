// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::
    core::{
        application::{
            WindowType,
            ApplicationMessage,
            ApplicationThread,
        },
        error::ApplicationError,
        localisation::Localisation,
        environment::Environment,
        traits::{ AnyWindowTrait, WindowTrait },
    }
;
use i18n::{
    pattern::PlaceholderValue, provider::LocalisationProvider, utility::TaggedString
};
use iced::{
    widget::{ button, column, container, scrollable, row, text, Column, },
    window, Alignment, Command, Element, Length, Point, Size
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
//const SIZE_MIN: ( f32, f32 ) = ( 150f32, 100f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 300f32, 250f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct AboutLocalisation {
    language: RefCount<String>,

    // Strings
    title: TaggedString,
    contributors: TaggedString,
    language_contributors: TaggedString,
    ok: TaggedString,
}

impl AboutLocalisation {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let language = localisation.localiser().default_language();
        let name = localisation.localiser().literal_with_defaults(
                "word", "about_i",
        )?;
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
        values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::TaggedString( localisation.localiser().literal_with_defaults(
                "word", "contributors_ip"
            )? ),
        );
        let contributors = localisation.localiser().format_with_defaults(
            "application",
            "add_colon_format",
            &values
        )?;
        let mut values = HashMap::<String, PlaceholderValue>::new();
        values.insert(
            "phrase".to_string(),
            PlaceholderValue::TaggedString( localisation.localiser().literal_with_defaults(
                "application", "language_contributors"
            )? ),
        );
        let language_contributors = localisation.localiser().format_with_defaults(
            "application",
            "add_colon_format",
            &values
        )?;
        Ok( AboutLocalisation {
            language,
            title,
            contributors,
            language_contributors,
            ok: localisation.localiser().literal_with_defaults(
                "word", "ok_i"
            )?,
        } )
    }
}

pub struct About {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: AboutLocalisation,
    application_short_name: String,
    version: String,
    contributors: Vec<String>,
    language_contributors: Vec<String>,
}

impl About {
    pub fn try_new(
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let details = localisation.localiser().localisation_provider().repository_details()?;
        let language_contributors = details.contributors.clone();
        let localisation = AboutLocalisation::try_new( localisation, environment )?;
        let mut split = env!( "CARGO_PKG_AUTHORS" ).split( ',' );
        let mut contributors = Vec::<String>::new();
        while let Some( author ) = split.next() {
            contributors.push( author.trim().to_string() );
        }
        Ok( About {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation,
            application_short_name: environment.application_short_name.clone(),
            version: env!( "CARGO_PKG_VERSION" ).to_string(),
            contributors,
            language_contributors,
        } )
    }
}

impl AnyWindowTrait for About {}

impl WindowTrait for About {
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
        let mut contributors = Column::new();
        let mut iterator = self.contributors.iter();
        while let Some( author ) = iterator.next() {
            contributors = contributors.push( row![
                text( "  " ),
                text( author.clone() ).width( Length::Fill ),
            ] );
        }
        let mut languages = Column::new();
        let mut iterator = self.language_contributors.iter();
        while let Some( contributor ) = iterator.next() {
            languages = languages.push( row![
                text( "  " ),
                text( contributor.clone() ).width( Length::Fill ),
            ] );
        }
        let body = column![
            text( self.localisation.contributors.as_str() ),
            contributors,
            text( " " ),
            text( self.localisation.language_contributors.as_str() ),
            languages,
        ];
        let content = column![
            column![
                text( self.application_short_name.as_str() ),
                text( self.version.as_str() ),
            ].width( Length::Fill ).align_items( Alignment::Center ),
            scrollable( body ).width( Length::Fill ).height( Length::Fill ),
            column![
                button( text( self.localisation.ok.as_str() ) )
                .padding( [ 5, 10 ] )
                .on_press( ApplicationMessage::Close( id.clone() ) ),
            ].align_items( Alignment::Center ),
        ].spacing( 10 ).align_items( Alignment::Center );
        container( content ).padding( 2 )
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
            self.localisation = AboutLocalisation::try_new( localisation, environment )?;
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

pub fn display_about(
    application: &mut ApplicationThread,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if !application.windows.contains_key( &WindowType::About ) {
        application.windows.insert(
            WindowType::About,
            Box::new( About::try_new( &application.localisation, &application.environment )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::About ).unwrap();
        window.try_update_localisation( &application.localisation, &application.environment, )?;
    }
    let size = application.session.settings.ui.about.size;
    let option = &application.session.settings.ui.about.position;
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
    application.spawn_with_disable( settings, &WindowType::About, &WindowType::Main )
}
