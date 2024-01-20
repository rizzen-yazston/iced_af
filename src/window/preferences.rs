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
        localisation::Localisation,
        environment::Environment,
        traits::{ AnyWindowTrait, WindowTrait },
        session::Settings,
    },
    widget::event_control,
    window::{
        fatal_error::display_fatal_error,
        main::Main,
    },
};
use i18n::{
    pattern::PlaceholderValue,
    provider::LocalisationProvider,
    utility::TaggedString,
};
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
use log::error;
use std::{
    collections::HashMap,
    any::Any,
};

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

#[cfg( feature = "log" )]
use crate::core::log::LogLevelConverter;

#[cfg( feature = "log" )]
use std::str::FromStr;

// Constants
//const SIZE_MIN: ( f32, f32 ) = ( 400f32, 300f32 );
pub const SIZE_DEFAULT: ( f32, f32 ) = ( 400f32, 300f32 );
const RESIZABLE: bool = false;
//const MAXIMISE: bool = false;

#[derive( Debug, Clone, PartialEq )]
pub enum PreferencesMessage {
    Accept( window::Id ),
    Cancel( window::Id ),
    LanguageSelected( String ),

    #[cfg( feature = "log" )]
    LogLevelSelected( String ),
}

#[derive( PartialEq, Clone, Debug )]
pub enum Setting {
    Language( String ),

    #[cfg( feature = "log" )]
    LogLevel( String ),
}

struct PreferencesLocalisation {
    language: RefCount<String>,
    right_to_left: bool,

    // Strings
    accept: TaggedString,
    cancel: TaggedString,
    languages_with_percentage: Vec<TaggedString>,
    title: TaggedString,
    ui_language: TaggedString,
    placeholder_language: TaggedString,

    #[cfg( feature = "log" )]
    log_level: TaggedString,

    #[cfg( feature = "log" )]
    placeholder_log_level: TaggedString,
}

impl PreferencesLocalisation {
    pub fn try_new(
        localisation: &Localisation,
        languages_available: &Vec<( RefCount<String>, f32)>,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let language = localisation.localiser().default_language();
        let locale = localisation.localiser().language_tag_registry().locale(
            language.as_str()
        )?;
        let name = localisation.localiser().literal_with_defaults(
            "word", "preferences_i"
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
        let mut languages_with_percentage = Vec::<TaggedString>::new();
        let mut iterator = languages_available.iter();
        while let Some( language ) = iterator.next() {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            let language_string = language.0.as_str().to_string();
            values.insert(
                "language".to_string(),
                PlaceholderValue::String( language_string.clone() ),
            );
            values.insert(
                "percent".to_string(), 
                PlaceholderValue::Unsigned( ( language.1 * 100f32 ) as u128 )
            );
            let text: TaggedString = localisation.localiser().format_with_defaults(
                "application",
                "language_percent_format",
                &values
            )?;
            languages_with_percentage.push( text );
        }
        Ok( PreferencesLocalisation {
            language,
            right_to_left: localisation.directionality().is_right_to_left( locale.id.clone() ),
            accept: localisation.localiser().literal_with_defaults(
                "word", "accept_i"
            )?,
            cancel: localisation.localiser().literal_with_defaults(
                "word", "cancel_i"
            )?,
            languages_with_percentage,
            title,
            ui_language: localisation.localiser().literal_with_defaults(
                "application", "ui_language"
            )?,
            placeholder_language: localisation.localiser().literal_with_defaults(
                "application", "placeholder_language"
            )?,

            #[cfg( feature = "log" )]
            log_level: localisation.localiser().literal_with_defaults(
                "application", "log_level"
            )?,

            #[cfg( feature = "log" )]
            placeholder_log_level: localisation.localiser().literal_with_defaults(
                "application", "placeholder_log_level"
            )?,
        } )
    }
}

pub struct Preferences {
    enabled: bool,
    parent: Option<WindowType>,
    localisation: PreferencesLocalisation,
    first_time: bool,
    settings: Settings,
    changed_settings: Option<Vec<Setting>>,
    languages_available: Vec<( RefCount<String>, f32 )>,
    language_list: combo_box::State<String>,
    language_map_to_tag: HashMap<String, RefCount<String>>,
    language_map_to_string: HashMap<RefCount<String>, String>,
    language_selected: Option<String>,
    language_changed: bool,

    #[cfg( feature = "log" )]
    log_levels: combo_box::State<String>,

    #[cfg( feature = "log" )]
    log_level_selected: Option<String>,
}

impl Preferences {
    pub fn try_new(
        localisation: &Localisation,
        settings: &Settings,
        first_time: bool,
        environment: &Environment,
    ) -> Result<Self, ApplicationError> {
        let mut languages_available = Vec::<( RefCount<String>, f32 )>::new();
        let binding = localisation
        .localiser()
        .localisation_provider()
        .component_details( "application", )?;
        let mut iterator = binding.languages
        .iter();
        while let Some( language_data ) = iterator.next() {
            languages_available.push( ( language_data.0.clone(), language_data.1.ratio ) );
        }
        let localisation = PreferencesLocalisation::try_new(
            localisation, &languages_available, environment
        )?;
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
            first_time,
            settings: settings.clone(),
            changed_settings: None,
            languages_available,
            language_list: combo_box::State::new( language_list ),
            language_map_to_tag,
            language_map_to_string,
            language_selected,
            language_changed: false,

            #[cfg( feature = "log" )]
            log_levels: combo_box::State::new( vec![
                "off".to_string(),
                "error".to_string(),
                "warn".to_string(),
                "info".to_string(),
                "debug".to_string(),
                "trace".to_string()
            ] ),

            #[cfg( feature = "log" )]
            log_level_selected: Some( settings.log_level.clone() ),
        } )
    }

    pub fn result_vector( &mut self ) -> Option<Vec<Setting>> {
        self.changed_settings.take()
    }

    pub fn update_settings( &mut self, settings: &Settings ) {
        self.settings = settings.clone();
    }

    pub fn clear_language_changed( &mut self ) -> bool {
        let previous = self.language_changed;
        self.language_changed = false;
        previous
    }

    pub fn language_selected( &self ) -> &RefCount<String> {
        self.language_map_to_tag.get( self.language_selected.as_ref().unwrap().as_str() ).unwrap()
    }

    pub fn end_first_time( &mut self ) {
        self.first_time = false;
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

    fn title( &self ) -> &TaggedString {
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
                        if self.language_changed {
                            self.language_selected = Some(
                                self.language_map_to_string.get( &self.settings.ui.language ).unwrap().to_string()
                            );
                        }
                    },
                    PreferencesMessage::LanguageSelected( language ) => {
                        self.language_selected = Some( language.clone() );
                        self.language_changed = true;
                    },

                    #[cfg( feature = "log" )]
                    PreferencesMessage::LogLevelSelected( log_level ) => {
                        self.log_level_selected = Some( log_level );
                    },
                    PreferencesMessage::Accept( _id ) => {
                        let language_selected_tag = self.language_map_to_tag
                        .get( &self.language_selected.clone().unwrap() )
                        .unwrap();
                        //log::error!( "Current language: {}", language_selected );
                        let mut changed_settings = Vec::<Setting>::new();
                        if self.settings.ui.language.as_str() != language_selected_tag.as_str() {
                            changed_settings.push( Setting::Language( language_selected_tag.to_string() ) );
                        }

                        #[cfg( feature = "log" )]
                        let log_level_selected = self.log_level_selected.clone().unwrap();

                        #[cfg( feature = "log" )]
                        if !log_level_selected.eq( &self.settings.log_level ) {
                            changed_settings.push( Setting::LogLevel( log_level_selected ) );
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
        let mut last_row =
        Vec::<Element<ApplicationMessage>>::new();
        let mut preferences = Column::new();
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
        if self.localisation.right_to_left {
            setting.reverse();
        }
        preferences = preferences.push( row( setting ) );
        last_row.push(
            button( self.localisation.accept.as_str() )
            .padding( [ 5, 10 ] )
            .on_press( ApplicationMessage::Preferences( PreferencesMessage::Accept( *id ) ) ).into()
        );
        if !self.first_time {
            last_row.push(
                button( self.localisation.cancel.as_str() )
                .padding( [ 5, 10 ] )
                .on_press( ApplicationMessage::Preferences( PreferencesMessage::Cancel( *id ) ) ).into()
            );

            #[cfg( feature = "log" )]
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
                if self.localisation.right_to_left {
                    setting.reverse();
                }
                preferences = preferences.push( row( setting ) );
            }

            // Add additional preferences
        }
        preferences = preferences.spacing( 2 ).width( Length::Fill ).align_items( Alignment::Start );
        if self.localisation.right_to_left {
            last_row.reverse();
        }
        let last_row_final = row( last_row.into_iter() );
        event_control::Container::new(
            column![
                column![
                    scrollable( preferences ),
                ].align_items( Alignment::Start ),
                column![].height( Length::Fill ), // Spacing column to push other two columns to top and bottom
                column![
                    last_row_final,
                ].align_items( Alignment::End )
            ].width( Length::Fill ).height( Length::Fill ).align_items( Alignment::Start ),
            self.enabled
        )
        .width( Length::Fill )
        .height( Length::Fill )
        .padding( 2 )
        .into()
    }

    fn parent( &self ) -> &Option<WindowType> {
        &self.parent
    }

    fn parent_remove( &mut self ) -> Option<WindowType> {
        self.parent.clone() // Always WindowType::Main, thus just faking remove.
    }

    #[allow(unused_variables)]
    fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            error!( "Updating localisation." );
            self.localisation = PreferencesLocalisation::try_new(
                localisation, &self.languages_available, environment
            )?;
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
        application.windows.insert(
            WindowType::Preferences,
            Box::new( Preferences::try_new(
                &application.localisation,
                &application.session.settings,
                false,
                &application.environment,
            )? )
        );
    } else {
        let window = application.windows.get_mut( &WindowType::Preferences ).unwrap();
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.update_settings( &application.session.settings );
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
                PreferencesMessage::LanguageSelected( _string ) => {
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                    application.localisation.localiser().defaults(
                        Some( actual.language_selected().as_str() ), None, None
                    )?;
                    window.try_update_localisation( &application.localisation, &application.environment )?;
                },
                PreferencesMessage::Accept( id ) => {
                    let mut localisation_update = false;
                    let mut _changed_settings: Option<Vec<Setting>> = None;
                    {
                        //let window = application.windows.get_mut( &WindowType::Preferences ).unwrap();
                        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
                        _changed_settings = actual.result_vector();
                        if actual.clear_language_changed() {

                            // Reset Preference localisation to settings.ui.language, in case language setting is
                            // not present in Vec<Setting>.
                            application.localisation.localiser().defaults(
                                Some( application.session.settings.ui.language.as_str() ), None, None
                            )?;
                            window.try_update_localisation( &application.localisation, &application.environment )?;
                        }
                    }
                    error!( "{:?}", _changed_settings );

                    // Handle all the changed settings, where necessary update components that require immediate
                    // effect.
                    if _changed_settings.is_some() {
                        let binding = _changed_settings.unwrap();
                        let mut iterator = binding.iter();
                        while let Some( setting ) = iterator.next() {
                            match setting {
                                Setting::Language( language ) => {
                                    application.session.settings.ui.language = language.clone();
                                    application.localisation.localiser().defaults(
                                        Some( application.session.settings.ui.language.as_str() ), None, None
                                    )?;
                                    localisation_update = true;
                                },

                                #[cfg( feature = "log" )]
                                Setting::LogLevel( string ) => {
                                    application.session.settings.log_level = string.clone();
                                    let log_level =
                                        LogLevelConverter::from_str( string.as_str() ).unwrap();
                                    log_level.configure_logger( &application.environment.logger, );
                                }
                            }
                        }

                    }

                    if localisation_update {
                        //loop throw all windows to update localisation
                        let mut iterator = application.windows.iter_mut();
                        while let Some( ( _window_type, window ) ) = iterator.next() {
                            window.try_update_localisation( &application.localisation, &application.environment )?;
                        }
                    }
                    command = close( application, *id )?
                },
                PreferencesMessage::Cancel( id ) => {
                    let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
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
    {
        let Some( window ) =
        application.windows.get_mut( &WindowType::Preferences ) else {
            return Ok( display_fatal_error(
                application, ApplicationError::WindowTypeNotFound( WindowType::Preferences )
            ) );
        };
        let actual = window.as_any_mut().downcast_mut::<Preferences>().unwrap();
        actual.end_first_time();
    
    }
    application.windows.insert(
        WindowType::Main,
        Box::new( Main::try_new( &application.localisation, &application.environment )? )
    );
    application.window_ids.insert( window::Id::MAIN , WindowType::Main );
    let size = application.session.settings.ui.main.size;
    let mut commands =
    Vec::<Command<ApplicationMessage>>::new();
    commands.push( window::resize( window::Id::MAIN, Size { width: size.0, height: size.1 } ) );
    Ok( Command::batch( commands ) )
}
