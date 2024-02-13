// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use super::{
    environment::Environment,
    error::ApplicationError,
};
use i18n::{
    icu::{ IcuDataProvider, DataProvider },
    localiser::Localiser,
    pattern::CommandRegistry,
    utility::LanguageTagRegistry,
    provider_sqlite3::LocalisationProviderSqlite3,
};
use icu_locid_transform::{ LocaleDirectionality, Direction as IcuDirection };
use icu_locid::Locale;
use iced::Alignment;

#[cfg( feature = "log" )]
#[allow( unused_imports )]
use log::{ error, warn, info, debug, trace };

#[cfg( feature = "sync" )]
use std::sync::Arc as RefCount;

#[cfg( not( feature = "sync" ) )]
use std::rc::Rc as RefCount;

/// Script text flow directions.
/// 
/// Currently ICU library only provides word flow direction for top to bottom line flow.
/// ICU4X developers are planning to complete support for all flow directions, to support the many types of scripts
/// available.
/// 
/// This framework provides data for all flow directions in the meantime, in order to avoid redesigning this framework
/// to support the other flow directions later when both ICU library (provides information) and `iced` library (renders
/// the glyphs to a screen region) supports vertical text.
/// 
/// Most common is top to bottom line flow.
pub const SCRIPT_TTB_LTR: ScriptData = ScriptData {
    flow_line: Direction::TopToBottom,
    flow_word: Direction::LeftToRight,
    reverse_lines: false,
    reverse_words: false,
    align_lines_start: Alignment::Start,
    align_lines_end: Alignment::End,
    align_words_start: Alignment::Start,
    align_words_end: Alignment::End,
};
pub const SCRIPT_TTB_RTL: ScriptData = ScriptData {
    flow_line: Direction::TopToBottom,
    flow_word: Direction::RightToLeft,
    reverse_lines: false,
    reverse_words: true,
    align_lines_start: Alignment::Start,
    align_lines_end: Alignment::End,
    align_words_start: Alignment::End,
    align_words_end: Alignment::Start,
};

/// Commonly known as vertical texts, for various eastern asia scripts.
pub const SCRIPT_RTL_TTB: ScriptData = ScriptData {
    flow_line: Direction::RightToLeft,
    flow_word: Direction::TopToBottom,
    reverse_lines: true,
    reverse_words: false,
    align_lines_start: Alignment::End,
    align_lines_end: Alignment::Start,
    align_words_start: Alignment::Start,
    align_words_end: Alignment::End,
};
pub const SCRIPT_LTR_TTB: ScriptData = ScriptData {
    flow_line: Direction::LeftToRight,
    flow_word: Direction::TopToBottom,
    reverse_lines: false,
    reverse_words: false,
    align_lines_start: Alignment::Start,
    align_lines_end: Alignment::End,
    align_words_start: Alignment::Start,
    align_words_end: Alignment::End,
};

/// The bottom to top line flow is very, though some have been seen on monuments.
pub const SCRIPT_BTT_LTR: ScriptData = ScriptData {
    flow_line: Direction::BottomToTop,
    flow_word: Direction::LeftToRight,
    reverse_lines: true,
    reverse_words: false,
    align_lines_start: Alignment::End,
    align_lines_end: Alignment::Start,
    align_words_start: Alignment::Start,
    align_words_end: Alignment::End,
};
pub const SCRIPT_BTT_RTL: ScriptData = ScriptData {
    flow_line: Direction::BottomToTop,
    flow_word: Direction::RightToLeft,
    reverse_lines: true,
    reverse_words: true,
    align_lines_start: Alignment::End,
    align_lines_end: Alignment::Start,
    align_words_start: Alignment::End,
    align_words_end: Alignment::Start,
};
pub const SCRIPT_RTL_BTT: ScriptData = ScriptData {
    flow_line: Direction::RightToLeft,
    flow_word: Direction::BottomToTop,
    reverse_lines: true,
    reverse_words: true,
    align_lines_start: Alignment::End,
    align_lines_end: Alignment::Start,
    align_words_start: Alignment::End,
    align_words_end: Alignment::Start,
};
pub const SCRIPT_LTR_BTT: ScriptData = ScriptData {
    flow_line: Direction::LeftToRight,
    flow_word: Direction::BottomToTop,
    reverse_lines: false,
    reverse_words: true,
    align_lines_start: Alignment::Start,
    align_lines_end: Alignment::End,
    align_words_start: Alignment::End,
    align_words_end: Alignment::Start,
};

// Initialise the i18n message system for the UI, using ICU4X internal data.
pub struct Localisation {
    localiser: Localiser,
    directionality: LocaleDirectionality, // ICU4X
}

impl Localisation {
    pub fn try_new<T: AsRef<str>>(
        environment: &Environment,
        language: T,
    ) -> Result<Localisation, ApplicationError> {
        let language_tag_registry = RefCount::new( LanguageTagRegistry::new() );
        let path = environment.application_path.join( "l10n" );
        let localisation_provider = LocalisationProviderSqlite3::try_new(
            path, &language_tag_registry, false
        )?;
        let icu_data_provider = RefCount::new( IcuDataProvider::try_new( DataProvider::Internal )? );
        let command_registry = RefCount::new( CommandRegistry::new() );
        let localiser = Localiser::try_new(
            &icu_data_provider,
            &language_tag_registry,
            Box::new( localisation_provider ),
            &command_registry,
            true,
            true,
            language,
        )?;
        let directionality = LocaleDirectionality::new();
        Ok( Localisation {                
            localiser,
            directionality,
        } )
    }

    pub fn localiser( &self ) -> &Localiser {
        &self.localiser
    }

    pub fn directionality( &self ) -> &LocaleDirectionality {
        &self.directionality
    }
}

/// Text flow data of scripts.
/// 
/// Field meaning:
/// 
/// * `flow_line`: The direction of the line stack goes in.
/// 
/// * `flow_word`: The direction of the words within the line.
/// 
/// * `reverse_lines`: Normally used to indicate the page elements of a [`Vec`] needs to be reversed before placement.
/// 
/// * `reverse_words`: Normally used to indicate the line elements of a [`Vec`] needs to be reversed before placement. 
/// 
/// * `align_lines_start`: Align the stack of lines to the start direction. Horizontal taken as top, and vertical
/// taken as left.
/// 
/// * `align_lines_end`: Align the stack of lines to the end direction.
/// 
/// * `align_words_start`: Align the words of the lines to the start direction. Horizontal taken as left, and vertical
/// taken as top.
/// 
/// * `align_words_end`: Align the words of the lines to the end direction.
/// 
/// `iced` horizontal layout flow is top to bottom for lines/rows and left to right for words/columns. Currently `iced`
/// has not vertical layout support.
//#[allow( dead_code )]
pub struct ScriptData {
    pub flow_line: Direction,
    pub flow_word: Direction,
    pub reverse_lines: bool,
    pub reverse_words: bool,
    pub align_lines_start: Alignment,
    pub align_lines_end: Alignment,
    pub align_words_start: Alignment,
    pub align_words_end: Alignment,
}

impl ScriptData {
    /// Currently supports only two script flow types: top to bottom left to right, and top to bottom right to left.
    pub fn new(
        localisation: &Localisation,
        locale: &RefCount<Locale>
    ) -> Self {
        let direction = match localisation.directionality().get( locale.as_ref() ) {
            None => ScriptDirection::default(),
            Some( icu_direction ) => match icu_direction {
                IcuDirection::LeftToRight => ScriptDirection::TopToBottomLeftToRight,
                IcuDirection::RightToLeft => ScriptDirection::TopToBottomRightToLeft,
                _ => ScriptDirection::default(), // needed for #[non_exhaustive] defined on IcuDirection enum.
            },
        };
        match direction {
            ScriptDirection::TopToBottomLeftToRight => SCRIPT_TTB_LTR,
            ScriptDirection::TopToBottomRightToLeft => SCRIPT_TTB_RTL,
            ScriptDirection::RightToLeftTopToBottom => SCRIPT_RTL_TTB,
            ScriptDirection::RightToLeftBottomToTop => SCRIPT_LTR_BTT,
            ScriptDirection::LeftToRightTopToBottom => SCRIPT_LTR_TTB,
            ScriptDirection::LeftToRightBottomToTop => SCRIPT_LTR_BTT,
            ScriptDirection::BottomToTopLeftToRight => SCRIPT_BTT_LTR,
            ScriptDirection::BottomToTopRightToLeft => SCRIPT_BTT_RTL,
        }
    }
}

// ------------------------------------------------------------------------------------------
// These are taken from local dev branch of the `i18n` project. Next version of `i18n` will include these.

pub enum Direction {
    TopToBottom,
    BottomToTop,
    LeftToRight,
    RightToLeft,
}

pub enum ScriptDirection {
    TopToBottomLeftToRight,
    TopToBottomRightToLeft,
    BottomToTopLeftToRight,
    BottomToTopRightToLeft,
    LeftToRightTopToBottom,
    LeftToRightBottomToTop,
    RightToLeftTopToBottom,
    RightToLeftBottomToTop,
}

impl ScriptDirection {
    pub fn directions( &self ) -> ( Direction, Direction ) {
        match self {
            ScriptDirection::TopToBottomLeftToRight => ( Direction::TopToBottom, Direction::LeftToRight ),
            ScriptDirection::TopToBottomRightToLeft => ( Direction::TopToBottom, Direction::RightToLeft ),
            ScriptDirection::RightToLeftTopToBottom => ( Direction::RightToLeft, Direction::TopToBottom ),
            ScriptDirection::RightToLeftBottomToTop => ( Direction::RightToLeft, Direction::BottomToTop ),
            ScriptDirection::LeftToRightTopToBottom => ( Direction::LeftToRight, Direction::TopToBottom ),
            ScriptDirection::LeftToRightBottomToTop => ( Direction::LeftToRight, Direction::BottomToTop ),
            ScriptDirection::BottomToTopLeftToRight => ( Direction::BottomToTop, Direction::LeftToRight ),
            ScriptDirection::BottomToTopRightToLeft => ( Direction::BottomToTop, Direction::RightToLeft ),
        }
    }
}

impl Default for ScriptDirection {
    fn default() -> Self {
        ScriptDirection::TopToBottomLeftToRight
    }
}
