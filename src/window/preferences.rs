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
        session::Settings,
    },
    widget::event_control,
    window::{
        fatal_error::display_fatal_error,
        main::Main,
    },
    APPLICATION_NAME_SHORT,
};
#[allow( unused_imports )]
use iced::{
    window,
    Command,
    widget::{ button, column, row, text, combo_box, scrollable, Column },
    Alignment,
    Element,
    Length,
    Size,
    Point,
    //advanced::iced_graphics::iced_core::Element
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

#[cfg( all( feature = "i18n", feature = "sync" ) )]
use std::sync::Arc as RefCount;

#[cfg( all( feature = "i18n", not( feature = "sync" ) ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "log" )]
use crate::core::log::LogLevelConverter;

#[cfg( feature = "log" )]
use std::str::FromStr;

#[cfg( feature = "i18n" )]
use std::collections::HashMap;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 400f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 500f32, 300f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

#[derive( Debug, Clone, PartialEq )]
pub enum PreferencesMessage {
    Accept( window::Id ),
    Cancel( window::Id ),
    #[cfg( feature = "i18n" )] LanguageSelected( String ),
    #[cfg( feature = "log" )] LogLevelSelected( String ),
}

#[derive( PartialEq, Clone, Debug )]
pub enum Setting {
    Language( String ),
    #[cfg( feature = "log" )] LogLevel( String ),
}

struct PreferencesLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    title: LString,
    accept: LString,
    cancel: LString,
    #[cfg( feature = "i18n" )] languages_with_percentage: Vec<LString>,
    #[cfg( feature = "i18n" )] ui_language: LString,
    #[cfg( feature = "i18n" )] placeholder_language: LString,
    #[cfg( feature = "log" )] log_level: LString,
    #[cfg( feature = "log" )] placeholder_log_level: LString,
}

impl PreferencesLocalisation {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
        #[cfg( feature = "i18n" )] languages_available: &Vec<( RefCount<String>, f32)>,
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
                        "word", "preferences_i",
                    )?
                )
            );
            localisation.localiser().format_with_defaults(
                "application", "window_title_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let title = format!( "{} - Preferences", APPLICATION_NAME_SHORT );

        #[cfg( feature = "i18n" )]
        let accept = localisation.localiser().literal_with_defaults(
            "word", "accept_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let accept = "Accept".to_string();

        #[cfg( feature = "i18n" )]
        let cancel = localisation.localiser().literal_with_defaults(
            "word", "cancel_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let cancel = "Cancel".to_string();

        #[cfg( feature = "i18n" )]
        let mut languages_with_percentage = Vec::<LString>::new();

        #[cfg( feature = "i18n" )]
        {
            let mut iterator = languages_available.iter();
            while let Some( language ) = iterator.next() {
                let mut values = HashMap::<String, PlaceholderValue>::new();
                let language_string = language.0.as_str().to_string();
                println!( "language: {}", language_string );
                values.insert(
                    "language".to_string(),
                    PlaceholderValue::String( language_string.clone() ),
                );
                values.insert(
                    "percent".to_string(), 
                    PlaceholderValue::Unsigned( ( language.1 * 100f32 ) as u128 )
                );
                println!( "about to localise" );
                let text = localisation.localiser().format_with_defaults(
                    "application",
                    "language_percent_format",
                    &values
                )?;
                languages_with_percentage.push( text );
            }
        }

        #[cfg( feature = "i18n" )]
        let ui_language = localisation.localiser().literal_with_defaults(
            "application", "ui_language"
        )?;

        #[cfg( feature = "i18n" )]
        let placeholder_language = localisation.localiser().literal_with_defaults(
            "application", "placeholder_language"
        )?;

        #[cfg( all( feature = "log", feature = "i18n" ) )]
        let log_level = localisation.localiser().literal_with_defaults(
            "application", "log_level"
        )?;

        #[cfg( all( feature = "log", not( feature = "i18n" ) ) )]
        let log_level = "Log level";

        #[cfg( all( feature = "log", feature = "i18n" ) )]
        let placeholder_log_level = localisation.localiser().literal_with_defaults(
            "application", "placeholder_log_level"
        )?;

        #[cfg( all( feature = "log", not( feature = "i18n" ) ) )]
        let placeholder_log_level = "Type a log level…".to_string();

        Ok( PreferencesLocalisation {
            #[cfg( feature = "i18n" )] language,
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
            title,
            accept,
            cancel,
            #[cfg( feature = "i18n" )] languages_with_percentage,
            #[cfg( feature = "i18n" )] ui_language,
            #[cfg( feature = "i18n" )] placeholder_language,
            #[cfg( feature = "log" )] log_level,
            #[cfg( feature = "log" )] placeholder_log_level,
        } )
    }
}

pub struct Preferences {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: PreferencesLocalisation,
    settings: Settings,
    changed_settings: Option<Vec<Setting>>,
    #[cfg( feature = "first_use" )] first_use: bool,
    #[cfg( feature = "i18n" )] languages_available: Vec<( RefCount<String>, f32 )>,
    #[cfg( feature = "i18n" )] language_list: combo_box::State<String>,
    #[cfg( feature = "i18n" )] language_map_to_tag: HashMap<String, RefCount<String>>,
    #[cfg( feature = "i18n" )] language_map_to_string: HashMap<RefCount<String>, String>,
    #[cfg( feature = "i18n" )] language_selected: Option<String>,
    #[cfg( feature = "i18n" )] language_changed: bool,
    #[cfg( feature = "log" )] log_levels: combo_box::State<String>,
    #[cfg( feature = "log" )] log_level_selected: Option<String>,
}

impl Preferences {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
        settings: &Settings,
        #[cfg( feature = "first_use" )] first_use: bool,
    ) -> Result<Self, ApplicationError> {
        #[cfg( feature = "log" )]
        let log_levels = combo_box::State::new( vec![
            "off".to_string(),
            "error".to_string(),
            "warn".to_string(),
            "info".to_string(),
            "debug".to_string(),
            "trace".to_string()
        ] );

        #[cfg( feature = "log" )]
        let log_level_selected = Some( settings.log_level.clone() );


        #[cfg( feature = "i18n" )]
        let mut languages_available = Vec::<( RefCount<String>, f32 )>::new();

        #[cfg( feature = "i18n" )]
        {
            let binding = localisation
            .localiser()
            .localisation_provider()
            .component_details( "application", )?;
            //println!( "{:?}", );
            let mut iterator = binding.languages
            .iter();
            while let Some( language_data ) = iterator.next() {
                languages_available.push( ( language_data.0.clone(), language_data.1.ratio ) );
            }
            println!( "Got component details" );
        }

        let localisation = PreferencesLocalisation::try_new(
            #[cfg( feature = "i18n" )] localisation,
            #[cfg( feature = "i18n" )] &languages_available,
        )?;

        #[cfg( feature = "i18n" )]
        {
            println!( "Added localisation strings" );
            let mut language_map_to_tag = HashMap::<String, RefCount<String>>::new();
            let mut language_map_to_string = HashMap::<RefCount<String>, String>::new();
            let mut language_list = Vec::<String>::new();
            let mut language_selected: Option<String> = None;
            let mut iterator2 = languages_available.iter().enumerate();
            while let Some( ( index, language_data ) ) = iterator2.next() {
                let display_string = localisation.languages_with_percentage[ index ].as_str().to_string();
                if settings.ui.language.as_str() == language_data.0.as_str() {
                    language_selected = Some( display_string.clone() );
                }
                language_map_to_tag.insert( display_string.clone(), RefCount::clone( &language_data.0 ) );
                language_map_to_string.insert( RefCount::clone( &language_data.0 ), display_string.clone() );
                language_list.push( display_string );
                
            }
            Ok( Preferences {
                enabled: true,
                parent: Some( WindowType::Main ),
                localisation,
                settings: settings.clone(),
                changed_settings: None,
                #[cfg( feature = "first_use" )]first_use,
                languages_available,
                language_list: combo_box::State::new( language_list ),
                language_map_to_tag,
                language_map_to_string,
                language_selected,
                language_changed: false,
                #[cfg( feature = "log" )] log_levels,
                #[cfg( feature = "log" )] log_level_selected,
            } )
        }

        #[cfg( not( feature = "i18n" ) )]
        Ok( Preferences {
            enabled: true,
            parent: Some( WindowType::Main ),
            localisation,
            settings: settings.clone(),
            changed_settings: None,
            #[cfg( feature = "first_use" )]first_use,
            #[cfg( feature = "log" )] log_levels,
            #[cfg( feature = "log" )] log_level_selected,
        } )
    }

    pub fn result_vector( &mut self ) -> Option<Vec<Setting>> {
        self.changed_settings.take()
    }

    pub fn update_settings( &mut self, settings: &Settings ) {
        self.settings = settings.clone();
    }

    #[cfg( feature = "i18n" )]
    pub fn clear_language_changed( &mut self ) -> bool {
        let previous = self.language_changed;
        self.language_changed = false;
        previous
    }

    #[cfg( feature = "i18n" )]
    pub fn language_selected( &self ) -> &RefCount<String> {
        self.language_map_to_tag.get( self.language_selected.as_ref().unwrap().as_str() ).unwrap()
    }

    #[cfg( feature = "first_use" )]
    pub fn end_first_use( &mut self ) {
        self.first_use = false;
    }
}

impl AnyWindowTrait for Preferences {}

impl WindowTrait for Preferences {
    fn as_any( &self ) -> &dyn Any {
        self
    }

    fn as_any_mut( &mut self ) -> &mut dyn Any {
        self
    }

    fn title( &self ) -> &LString {
        &self.localisation.title
    }

    fn try_update(
        &mut self,
        message: ApplicationMessage
    ) -> Result<Command<ApplicationMessage>, ApplicationError> {
        let command = Command::none();
        match message {
            ApplicationMessage::Preferences( inner_message ) => {
                match inner_message {
                    PreferencesMessage::Cancel( _id ) => {
                        #[cfg( feature = "i18n" )]
                        if self.language_changed {
                            self.language_selected = Some(
                                self.language_map_to_string.get( &self.settings.ui.language ).unwrap().to_string()
                            );
                        }
                    },

                    #[cfg( feature = "i18n" )]
                    PreferencesMessage::LanguageSelected( language ) => {
                        self.language_selected = Some( language.clone() );
                        self.language_changed = true;
                    },

                    #[cfg( feature = "log" )]
                    PreferencesMessage::LogLevelSelected( log_level ) => {
                        self.log_level_selected = Some( log_level );
                    },
                    PreferencesMessage::Accept( _id ) => {
                        #[cfg( feature = "i18n" )]
                        let language_selected_tag = self.language_map_to_tag
                        .get( &self.language_selected.clone().unwrap() )
                        .unwrap();

                        //log::error!( "Current language: {}", language_selected );
                        #[allow( unused_mut )]
                        let mut changed_settings = Vec::<Setting>::new();

                        #[cfg( feature = "i18n" )]
                        if self.settings.ui.language.as_str() != language_selected_tag.as_str() {
                            changed_settings.push( Setting::Language( language_selected_tag.to_string() ) );
                        }

                        #[cfg( feature = "log" )]
                        {
                            let log_level_selected = self.log_level_selected.clone().unwrap();
                            if !log_level_selected.eq( &self.settings.log_level ) {
                                changed_settings.push( Setting::LogLevel( log_level_selected ) );
                            }
                        }
        
                        // Insert additional settings above.
                        if !changed_settings.is_empty() {
                            self.changed_settings = Some( changed_settings );
                        }
                    },
                }
            },
            _ => {}
        }
        Ok( command )
    }

    fn view( &self, id: &window::Id ) -> Element<ApplicationMessage> {
        #[cfg( feature = "i18n" )]
        let align_start = self.localisation.script_data.align_words_start;

        #[cfg( not( feature = "i18n" ) )]
        let align_start = Alignment::Start;

        #[cfg( feature = "i18n" )]
        let align_end = self.localisation.script_data.align_words_end;

        #[cfg( not( feature = "i18n" ) )]
        let align_end = Alignment::End;

        let mut content: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        // Preferences - scrollable
        #[allow( unused_mut )]
        let mut preferences: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();

        #[cfg( feature = "i18n" )]
        {
            let mut setting: Vec<Element<ApplicationMessage>> = vec![
                text( self.localisation.ui_language.as_str() ).into(),
                text( "" ).width( Length::Fill ).into(),
                combo_box(
                    &self.language_list,
                    self.localisation.placeholder_language.as_str(),
                    self.language_selected.as_ref(),
                    |string| ApplicationMessage::Preferences(
                        PreferencesMessage::LanguageSelected( string )
                    )
                ).width( 100 ).into(),
            ];
            if self.localisation.script_data.reverse_words {
                setting.reverse();
            }
            preferences.push( row( setting ).into() );
        }

        #[cfg( feature = "first_use" )]
        let display = !self.first_use;

        #[cfg( not( feature = "first_use" ) )]
        let display = true;

        if display {
            #[cfg( feature = "log", )]
            {
                let mut setting: Vec<Element<ApplicationMessage>> = vec![
                    text( self.localisation.log_level.as_str() ).into(),
                    text( "" ).width( Length::Fill ).into(),
                    combo_box(
                        &self.log_levels,
                        self.localisation.placeholder_log_level.as_str(),
                        self.log_level_selected.as_ref(),
                        |string| ApplicationMessage::Preferences(
                            PreferencesMessage::LogLevelSelected( string )
                        ),
                    ).width( 100 ).into(),
                ];
    
                #[cfg( feature = "i18n" )]
                if self.localisation.script_data.reverse_words {
                    setting.reverse();
                }
    
                preferences.push( row( setting ).into() );
            }
        }


        // Add additional preferences above this comment.

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            preferences.reverse();
        }

        content.push(
            scrollable(
                column( preferences ).width( Length::Fill ).align_items( align_start )
            ).width( Length::Fill ).height( Length::Fill ).into()
        );
        content.push( " ".into() ); // Paragraph separation

        // Buttons
        let mut buttons: Vec<Element<ApplicationMessage>> = Vec::<Element<ApplicationMessage>>::new();
        buttons.push(
            button( self.localisation.accept.as_str() )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Preferences( PreferencesMessage::Accept( *id ) ) ).into()
        );

        #[cfg( feature = "first_use" )]
        if !self.first_use {
            buttons.push(
                button( self.localisation.cancel.as_str() )
                .padding( [ 5, 10 ] )
                .on_press( ApplicationMessage::Preferences( PreferencesMessage::Cancel( *id ) ) ).into()
            );
        }

        #[cfg( not( feature = "first_use" ) )]
        buttons.push(
            button( self.localisation.cancel.as_str() )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Preferences( PreferencesMessage::Cancel( *id ) ) ).into()
        );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_words {
            buttons.reverse();
        }

        content.push(
            column![ row( buttons ).spacing( 10 ) ]
            .width( Length::Fill )
            .align_items( align_end )
            .into()
        );

        #[cfg( feature = "i18n" )]
        if self.localisation.script_data.reverse_lines {
            content.reverse();
        }

        event_control::Container::new(
            column( content ).width( Length::Fill ),
            self.enabled
        ).height( Length::Fill ).padding( 2 )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove( &mut self ) -> Option<WindowType> {
        self.parent.clone() // Always WindowType::Main, thus just faking remove.
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

            self.localisation = PreferencesLocalisation::try_new( localisation, &self.languages_available, )?;
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

pub fn display_preferences(
    application: &mut ApplicationThread,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if !application.windows.contains_key( &WindowType::Preferences ) {
        #[cfg( feature = "i18n" )]
        application.windows.insert(
            WindowType::Preferences,
            Box::new( Preferences::try_new(
                &application.localisation,
                &application.session.settings,

                #[cfg( feature = "first_use" )]
                false,
            )? )
        );

        #[cfg( not( feature = "i18n" ) )]
        application.windows.insert(
            WindowType::Preferences,
            Box::new( Preferences::try_new(
                &application.session.settings,

                #[cfg( feature = "first_use" )]
                false,
            )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::Preferences ).unwrap();
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.update_settings( &application.session.settings );

        #[cfg( feature = "i18n" )]
        window.try_update_localisation( &application.localisation, &application.environment, )?;
    }
    let size = application.session.settings.ui.preferences.size;
    let option = &application.session.settings.ui.preferences.position;
    let position = if option.is_none() {
        window::Position::Centered
    } else {
        let value = option.as_ref().unwrap();
        window::Position::Specific( Point { x: value.0, y: value.1 } )
    };
    let settings = window::Settings {
        // Always centred, thus position not saved in session.
        size: Size::new( size.0, size.1 ),
        resizable: RESIZABLE,
        position,
        exit_on_close_request: false,
        ..Default::default()
    };
    application.spawn_with_disable( settings, &WindowType::Preferences, &WindowType::Main )
}

pub fn update_preferences(
    application: &mut ApplicationThread,
    message: ApplicationMessage,
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    let mut command = Command::none();
    match  message {
        ApplicationMessage::Preferences( ref preferences_message ) => {
            let Some( window ) =
            application.windows.get_mut( &WindowType::Preferences ) else {
                return Ok( display_fatal_error(
                    application, ApplicationError::WindowTypeNotFound( WindowType::Preferences )
                ) );
            };
            command = window.try_update( message.clone() )?;

            // Post internal update
            match preferences_message {
                #[cfg( feature = "i18n" )]
                PreferencesMessage::LanguageSelected( _string ) => {
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                    application.localisation.localiser().defaults(
                        Some( actual.language_selected().as_str() ), None, None
                    )?;
                    window.try_update_localisation( &application.localisation, &application.environment )?;
                },
                PreferencesMessage::Accept( id ) => {
                    #[cfg( feature = "i18n" )]
                    let mut localisation_update = false;

                    let mut _changed_settings: Option<Vec<Setting>> = None;
                    {
                        //let window = application.windows.get_mut( &WindowType::Preferences ).unwrap();
                        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                        _changed_settings = actual.result_vector();

                        #[cfg( feature = "i18n" )]
                        if actual.clear_language_changed() {

                            // Reset Preference localisation to settings.ui.language, in case language setting is
                            // not present in Vec<Setting>.
                            application.localisation.localiser().defaults(
                                Some( application.session.settings.ui.language.as_str() ),
                                None,
                                None
                            )?;
                            window.try_update_localisation( &application.localisation, &application.environment )?;
                        }
                    }
                    #[cfg( feature = "log" )]
                    error!( "{:?}", _changed_settings );

                    // Handle all the changed settings, where necessary update components that require immediate
                    // effect.
                    if _changed_settings.is_some() {
                        let binding = _changed_settings.unwrap();
                        let mut iterator = binding.iter();
                        while let Some( setting ) = iterator.next() {
                            match setting {
                                #[cfg( feature = "i18n" )]
                                Setting::Language( language ) => {
                                    application.session.settings.ui.language = language.clone();
                                    application.localisation.localiser().defaults(
                                        Some(
                                            application.session.settings.ui.language.as_str()
                                        ), None, None
                                    )?;
                                    localisation_update = true;
                                },

                                #[cfg( feature = "log" )]
                                Setting::LogLevel( string ) => {
                                    application.session.settings.log_level = string.clone();
                                    let log_level =
                                        LogLevelConverter::from_str( string.as_str() ).unwrap();
                                    log_level.configure_logger( &application.environment.logger, );
                                },

                                #[allow( unreachable_patterns )]
                                _ => {}
                            }
                        }

                    }

                    #[cfg( feature = "i18n" )]
                    if localisation_update {
                        //loop throw all windows to update localisation
                        let mut iterator = application.windows.iter_mut();
                        while let Some(
                            ( _window_type, window )
                        ) = iterator.next() {
                            window.try_update_localisation( &application.localisation, &application.environment )?;
                        }
                    }

                    command = close( application, *id )?
                },
                PreferencesMessage::Cancel( id ) => {
                    #[cfg( feature = "i18n" )]
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                    #[cfg( feature = "i18n" )]
                    if actual.clear_language_changed() {

                        // Reset Preference localisation to settings.ui.language.
                        application.localisation.localiser().defaults(
                            Some( application.session.settings.ui.language.as_str() ),
                            None,
                            None
                        )?;
                        window.try_update_localisation( &application.localisation, &application.environment )?;
                    }

                    command = application.close( *id )?
                },
                #[allow( unreachable_patterns )]
                _ => {}
            }
        },
        _ => {}
    }
    Ok( command )
}

pub fn close(
    application: &mut ApplicationThread,
    id: window::Id
) -> Result<Command<ApplicationMessage>, ApplicationError> {
    if id != window::Id::MAIN {
        return application.close( id );
    }

    #[cfg( feature = "first_use" )]
    {
        let Some( window ) =
        application.windows.get_mut( &WindowType::Preferences ) else {
            return Ok( display_fatal_error(
                application, ApplicationError::WindowTypeNotFound( WindowType::Preferences )
            ) );
        };
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.end_first_use();
    }

    application.windows.insert(
        WindowType::Main,
        Box::new( Main::try_new(
            #[cfg( feature = "i18n" )] &application.localisation,
        )? )
    );
    application.window_ids.insert( window::Id::MAIN , WindowType::Main );
    let size = application.session.settings.ui.main.size;
    let mut commands =
    Vec::<Command<ApplicationMessage>>::new();
    commands.push( window::resize( window::Id::MAIN, Size { width: size.0, height: size.1 } ) );
    Ok( Command::batch( commands ) )
}
