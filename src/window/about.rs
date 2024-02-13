// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::{
    core::{
        application::{
            ApplicationMessage, ApplicationThread, WindowType
        },
        error::ApplicationError,
        traits::{ AnyWindowTrait, WindowTrait }
    },
    APPLICATION_NAME,
    APPLICATION_NAME_SHORT,
    AUTHORS,
    VERSION,
};
use iced::{
    widget::{ button, column, container, row, scrollable, text },
    window,
    Alignment,
    Command,
    Element,
    Length,
    Point,
    Size
};
use std::any::Any;

#[cfg( feature = "i18n" )]
use crate::core::{
    localisation::{
        Localisation,
        ScriptData,
        //Direction, //used for layout flow direction test
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
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 300f32, 250f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

pub struct AboutLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    title: LString,
    contributors: LString,
    ok: LString,
    #[cfg( feature = "i18n" )] localisation_contributors: LString,
}

impl AboutLocalisation {
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
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "application".to_string(),
                PlaceholderValue::String( APPLICATION_NAME_SHORT.to_string() ),
            );
            values.insert(
                "window".to_string(), 
                PlaceholderValue::TaggedString(
                    localisation.localiser().literal_with_defaults(
                        "word", "about_i",
                    )?
                )
            );
            localisation.localiser().format_with_defaults(
                "application", "window_title_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let title = format!( "{} - About", APPLICATION_NAME_SHORT );

        #[cfg( feature = "i18n" )]
        let contributors = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "phrase".to_string(),
                PlaceholderValue::TaggedString(
                    localisation.localiser().literal_with_defaults(
                        "word", "contributors_ip"
                    )?
                ),
            );
            localisation.localiser().format_with_defaults(
                "application", "add_colon_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let contributors = "Contributors:".to_string();

        #[cfg( feature = "i18n" )]
        let localisation_contributors = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "phrase".to_string(),
                PlaceholderValue::TaggedString(
                    localisation.localiser().literal_with_defaults(
                        "application", "localisation_contributors"
                    )?
                ),
            );
            localisation.localiser().format_with_defaults(
                "application", "add_colon_format", &values
            )?
        };

        #[cfg( feature = "i18n" )]
        let ok = {
            localisation.localiser().literal_with_defaults(
                "word", "ok_i"
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let ok = "OK".to_string();

        Ok( AboutLocalisation {
            #[cfg( feature = "i18n" )] language,
            /* just here to test other script direction flow of objects. text is only partially supported in iced.
            #[cfg( feature = "i18n" )] script_data: ScriptData { //faking directions to test layout
                flow_line: Direction::BottomToTop,
                flow_word: Direction::RightToLeft,
                reverse_lines: true,
                reverse_words: true,
                align_lines_start: Alignment::End,
                align_lines_end: Alignment::Start,
                align_words_start: Alignment::End,
                align_words_end: Alignment::Start,
            },
            */
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
            title,
            contributors,
            ok,
            #[cfg( feature = "i18n" )] localisation_contributors,
        } )
    }
}

pub struct About {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: AboutLocalisation,
    contributors: Vec<String>,
    #[cfg( feature = "i18n" )] localisation_contributors: Vec<String>,
}

impl About {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        let about_localisation = AboutLocalisation::try_new(
            #[cfg( feature = "i18n" )] localisation,
        )?;
        let mut split = AUTHORS.split( ',' );
        let mut contributors = Vec::<String>::new();
        while let Some( author ) = split.next() {
            contributors.push( author.trim().to_string() );
        }
        #[cfg( feature = "i18n" )]
        {
            let details = localisation.localiser().localisation_provider().repository_details()?;
            let localisation_contributors = details.contributors.clone();
            Ok( About {
                enabled: true,
                parent: Some( WindowType::Main ),
                localisation: about_localisation,
                contributors,
                localisation_contributors,
            } )
        }

        #[cfg( not( feature = "i18n" ) )]
        Ok( About {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation: about_localisation,
            contributors,
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

    fn title( &self ) -> &LString {
        &self.localisation.title
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        #[cfg( feature = "i18n" )]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg( not( feature = "i18n" ) )]
        let align_start = Alignment::Start;

        let mut content: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        // Header
        let mut header: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();
        header.push( APPLICATION_NAME.into() );
        header.push( VERSION.into() );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            header.reverse();
        }

        content.push( column( header ).width( Length::Fill ).align_items( Alignment::Center ).into() );

        // Body - scrollable
        let mut body: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        let mut contributors: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();
        let mut iterator = self.contributors.iter();
        while let Some( author ) = iterator.next() {
            #[allow( unused_mut )]
            let mut contributor: Vec<Element<ApplicationMessage>> = vec![
                text( "  " ).into(),
                text( author.clone() ).into(),
            ];

            #[cfg( feature = "i18n" )]
            if self.localisation.script_data.reverse_words {
                contributor.reverse();
            }

            contributors.push( row( contributor ).into() );
        }

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            contributors.reverse();
        }

        body.push( text( self.localisation.contributors.as_str() ).into() );
        body.push( column( contributors ).width( Length::Fill ).align_items( align_start ).into() );

        #[cfg( feature = "i18n" )]
        {
            let mut localisations: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();
            let mut iterator = self.localisation_contributors.iter();
            while let Some( language ) = iterator.next() {
                let mut contributor: Vec<Element<ApplicationMessage>> = vec![
                    text( "  " ).into(), // Indentation space
                    text( language.clone() ).into(),
                ];
                if self.localisation.script_data.reverse_words {
                    contributor.reverse();
                }
                localisations.push( row( contributor ).into() );
            }
            if self.localisation.script_data.reverse_lines {
                localisations.reverse();
            }
            body.push( " ".into() ); // Paragraph separation
            body.push( text( self.localisation.localisation_contributors.as_str() ).into() );
            body.push( column( localisations ).width( Length::Fill ).align_items( align_start ).into() );
        }

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            body.reverse();
        }

        content.push(
            scrollable(
                column( body ).width( Length::Fill ).align_items( align_start )
            ).width( Length::Fill ).height( Length::Fill ).into()
        );
        content.push( " ".into() ); // Paragraph separation

        // OK button
        content.push( column![
            button( text( self.localisation.ok.as_str() ) )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Close( id.clone() ) )
        ].width( Length::Fill ).align_items( Alignment::Center ).into() );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        container( column( content ).width( Length::Fill ) ).padding( 2 )
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

            self.localisation = AboutLocalisation::try_new( localisation, )?;
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
            Box::new( About::try_new(
                #[cfg( feature = "i18n" )] &application.localisation,
            )? )
        );
    } else {
        #[cfg( feature = "i18n" )]
        {
            let window = application.windows.get_mut( &WindowType::About ).unwrap();
            window.try_update_localisation( &application.localisation, &application.environment, )?;
        }
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
