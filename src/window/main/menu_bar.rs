// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.


use crate::{
    application::{StringGroup, WindowType},
    core::localisation::StringCache,
    localisation::{main, main_common},
};
use iced::{
    alignment,
    border::Radius,
    widget::{button, container, text},
    window,
    Border, Color, Element, Length
};

#[cfg(not(feature = "iced_aw"))]
use iced_aw::{
    menu::{self, Item, Menu},
    menu_bar, menu_items,
    widgets::InnerBounds,
    quad,
    style::{menu_bar::primary, Status},
};

#[cfg(feature = "iced_aw")]
use crate::{
    iced_aw::{
        widgets::{
            menu::{self, Item, Menu},
            InnerBounds,
            quad,
        },
        style::{menu_bar::primary, Status},
    },
    menu_bar,
    menu_items,
};

#[allow(unused_imports)]
use log::{debug, error, info, trace, warn};

#[derive(Debug, Clone)]
pub enum Message {
    None, // Used for the menu bar button, and buttons that open sub menus to the side.
    New(WindowType),
    //Open(WindowType),
    Close(window::Id),
    CloseAll,
    Preferences,
    About,
}

pub fn view(id: window::Id, string_cache: &StringCache) -> Element<'_, Message> {
    let main = string_cache.get(&StringGroup::Main).unwrap();
    let common = string_cache.get(&StringGroup::MainCommon).unwrap();
    let menu_type_1 = |items| Menu::new(items).max_width(180.0).offset(15.0).spacing(5.0);

    /*
    let menu_type_2 =
    |items| Menu::new(items).max_width(180.0).offset(0.0).spacing(5.0);
    */

    let bar = menu_bar!(
        // File menu
        (
            labeled_button(common.string(main_common::Index::File as usize), Message::None),
            menu_type_1(menu_items!(
                (labeled_button(
                    common.string(main_common::Index::New as usize),
                    Message::New(WindowType::Main)
                ))
                /*
                (labeled_button(
                    common.string(main_common::Index::Open as usize),
                    Message::Open(WindowType::Main)
                ))
                */
                (separator())
                (labeled_button(main.string(main::Index::Close as usize), Message::Close(id)))
                (labeled_button(main.string(main::Index::CloseAll as usize), Message::CloseAll))
            ))
        )

        // Edit menu
        (
            labeled_button(common.string(main_common::Index::Edit as usize), Message::None),
            menu_type_1(menu_items!(
                (labeled_button(common.string(main_common::Index::Preferences as usize), Message::Preferences))
            ))
        )

        // Help menu
        (
            labeled_button(common.string(main_common::Index::Help as usize), Message::None),
            menu_type_1(menu_items!(
                (labeled_button(common.string(main_common::Index::About as usize), Message::About))
            ))
        )
    )
    .draw_path(menu::DrawPath::Backdrop)
    .style(|theme:&iced::Theme, status: Status | menu::Style{
        path_border: Border{
            radius: Radius::new(6.0),
            ..Default::default()
        },
        ..primary(theme, status)
    });

    container(bar).into()
}

fn base_button<'a>(
    content: impl Into<Element<'a, Message, iced::Theme, iced::Renderer>>,
    message: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    button(content)
        .padding([4, 8])
        //.style(iced::theme::Button::Custom(Box::new(ButtonStyle {})))
        .on_press(message)
}

fn labeled_button<'a>(
    label: &'a str,
    message: Message,
) -> button::Button<'a, Message, iced::Theme, iced::Renderer> {
    base_button(
        text(label)
            //.width( Length::Fill )
            //.height( Length::Fill )
            .align_y(alignment::Vertical::Center),
        message,
    )
}

fn separator() -> quad::Quad {
    quad::Quad {
        quad_color: Color::from([0.5; 3]).into(),
        quad_border: Border {
            radius: Radius::new(4.0),
            ..Default::default()
        },
        inner_bounds: InnerBounds::Ratio(0.98, 0.2),
        height: Length::Fixed(5.0),
        ..Default::default()
    }
}
