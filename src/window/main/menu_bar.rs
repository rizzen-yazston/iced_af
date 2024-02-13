// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use crate::core::error::ApplicationError;
use iced::{
    widget::{ button, container, text, /* following are temp until 0.12 */ column, row },
    alignment,
    Element,
    Length,
    Alignment
};
/* commented out until `menu` of `iced_aw` has been upgraded to work with latest commit of `iced`.
use iced_aw::{
    helpers::{ menu_bar, menu_tree },
    menu::{
        //MenuBar,
        MenuTree,
        CloseCondition,
        PathHighlight
    }, 
    quad,
    menu_tree,
};
// */

#[cfg( feature = "i18n" )]
use std::collections::HashMap;

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

#[derive( Debug, Clone )]
pub enum MainMenuBarMessage {
    None, // Used for the menu bar button, and buttons that open sub menus to the side.
    Exit,
    Preferences,
    About,
}

pub struct MainMenuBarLocalisation {
    #[cfg( feature = "i18n" )] language: RefCount<String>,
    #[cfg( feature = "i18n" )] script_data: ScriptData,

    // Strings
    file: LString,
    exit: LString,
    edit: LString,
    preferences: LString,
    help: LString,
    about: LString,
}

impl MainMenuBarLocalisation {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        #[cfg( feature = "i18n" )]
        let language = localisation.localiser().default_language();

        #[cfg( feature = "i18n" )]
        let locale = localisation.localiser().language_tag_registry().locale(
            language.as_str()
        )?;

        // File menu
        #[cfg( feature = "i18n" )]
        let file = localisation.localiser().literal_with_defaults(
            "word", "file_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let file = "File".to_string();

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

        // Edit menu
        #[cfg( feature = "i18n" )]
        let edit = localisation.localiser().literal_with_defaults(
            "word", "edit_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let edit = "Edit".to_string();

        #[cfg( feature = "i18n" )]
        let preferences = {
            let mut values = HashMap::<String, PlaceholderValue>::new();
            values.insert(
                "phrase".to_string(),
                PlaceholderValue::TaggedString( localisation.localiser().literal_with_defaults(
                    "word", "preferences_i"
                )? ),
            );
            localisation.localiser().format_with_defaults(
                "application", "add_elipsis_format", &values
            )?
        };

        #[cfg( not( feature = "i18n" ) )]
        let preferences = "Preferences…".to_string();

        // Help menu
        #[cfg( feature = "i18n" )]
        let help = localisation.localiser().literal_with_defaults(
            "word", "help_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let help = "Help".to_string();

        #[cfg( feature = "i18n" )]
        let about = localisation.localiser().literal_with_defaults(
            "word", "about_i"
        )?;

        #[cfg( not( feature = "i18n" ) )]
        let about = "About".to_string();

        Ok( MainMenuBarLocalisation {
            #[cfg( feature = "i18n" )] language,
            #[cfg( feature = "i18n" )] script_data: ScriptData::new( localisation, &locale ),
            file,
            exit,
            edit,
            preferences,
            help,
            about,
        } )
    }
}

pub struct MainMenuBar {
    localisation: MainMenuBarLocalisation,
}

impl MainMenuBar {
    pub fn try_new(
        #[cfg( feature = "i18n" )] localisation: &Localisation,
    ) -> Result<Self, ApplicationError> {
        Ok( MainMenuBar {
            localisation: MainMenuBarLocalisation::try_new(
                #[cfg( feature = "i18n" )] localisation,
            )?,
        } )
    }

    #[cfg( feature = "i18n" )]
    pub fn try_update_localisation(
        &mut self,
        localisation: &Localisation,
        _environment: &Environment,
    ) -> Result<(), ApplicationError> {
        if self.localisation.language != localisation.localiser().default_language() {
            #[cfg( feature = "log" )]
            info!( "Updating localisation." );

            self.localisation = MainMenuBarLocalisation::try_new( localisation, )?;
        }
        Ok( () )
    }

    pub fn view( &self ) -> Element<MainMenuBarMessage> {
        // faking a menubar, by just displaying the buttons in rows.
        let bar = column![ row![
            labeled_button( self.localisation.file.as_str(), MainMenuBarMessage::None ),
            labeled_button( self.localisation.exit.as_str(), MainMenuBarMessage::Exit ),
            text( "" ).width( Length::Fill ),
        ].spacing( 0 ).align_items( Alignment::Start ),
        row![
            labeled_button( self.localisation.edit.as_str(), MainMenuBarMessage::None ),
            labeled_button( self.localisation.preferences.as_str(), MainMenuBarMessage::Preferences ),
            text( "" ).width( Length::Fill ),
        ].spacing( 0 ).align_items( Alignment::Start ),
        row![
            labeled_button( self.localisation.help.as_str(), MainMenuBarMessage::None ),
            labeled_button( self.localisation.about.as_str(), MainMenuBarMessage::About ),
            text( "" ).width( Length::Fill ),
        ].spacing( 0 ).align_items( Alignment::Start ),
        ].align_items( Alignment::Start );

        /* commenting out until `menu` of `iced_aw` has been upgraded. this will need fixing.
        let mut menu_roots = Vec::<MenuTree<MainMenuBarMessage, Renderer<Theme>>>::new();
        let mut menu_bar = menu_bar( menu_roots );

        let bar = menu_bar!(
            menu_database( self ),
            //menu_edit( self ),
            //menu_3( self ),
            //menu_4( self )
        )
        .spacing( 4.0 )
        .bounds_expand( 30 )
        .main_offset( 13 )
        .cross_offset( 16 )
        .path_highlight( Some( PathHighlight::MenuActive ) )
        .close_condition( CloseCondition {
            leave: true,
            click_outside: false,
            click_inside: false,
        } );
        */
        container( bar )
        .into()
    }
}

fn base_button<'a>(
    content: impl Into<Element<'a, MainMenuBarMessage, iced::Renderer>>,
    message: MainMenuBarMessage,
) -> button::Button<'a, MainMenuBarMessage, iced::Renderer> {
    button( content )
    .padding( [ 4, 8 ] )
    //.style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
    .on_press( message )
}

fn labeled_button<'a>( label: &str, message: MainMenuBarMessage ) -> button::Button<'a, MainMenuBarMessage, iced::Renderer> {
    base_button(
        text (label )
        //.width( Length::Fill )
        //.height( Length::Fill )
        .vertical_alignment( alignment::Vertical::Center ),
        message,
    )
}

/* TODO: Once `iced_aw` has been updated, this to be fixed.
fn separator<'a>() -> MenuTree<'a, MainMenuBarMessage, iced::Renderer> {
    menu_tree!( quad::Quad {
        color: [ 0.5; 3 ].into(),
        border_radius: [ 4.0; 4 ],
        inner_bounds: quad::InnerBounds::Ratio( 0.98, 0.1 ),
        ..Default::default()
    } )
}

fn menu_database<'a>( main: &MainMenuBar ) -> MenuTree<'a, MainMenuBarMessage, iced::Renderer> {
    /*
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(220);
    */

    let root = menu_tree(
        labeled_button( main.localisation.database.as_str(), MenuBarMessage::None ),
        vec![
            menu_tree!(
                labeled_button( main.localisation.connect.as_str(), MenuBarMessage::Connect )
                .width( Length::Fill )
                .height( Length::Fill )
            ),
            //separator(),
            /*
            menu_tree!(
                labeled_button( main.localisation.exit.as_str(), MenuBarMessage::Exit )
                .width( Length::Fill )
                .height( Length::Fill )
            ),
            */

            /*
            sub_1,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            */
        ],
    )
    .width(110);
    root
}

fn menu_edit<'a>( main: &MenuBar ) -> MenuTree<'a, MenuBarMessage, iced::Renderer> {
    /*
    let sub_1 = debug_sub_menu(
        "A sub menu",
        vec![
            debug_item("Item"),
            debug_item("Item"),
            sub_2,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
        ],
    )
    .width(220);
    */

    let root = menu_tree(
        labeled_button( main.localisation.edit.as_str(), MenuBarMessage::None ),
        vec![
            menu_tree!( labeled_button( main.localisation.preferences.as_str(), MenuBarMessage::Preferences )
            .width( Length::Fill )
            .height( Length::Fill ) ),
            /*
            sub_1,
            debug_item("Item"),
            debug_item("Item"),
            debug_item("Item"),
            */
        ],
    )
    .width(110);
    root
}
*/
