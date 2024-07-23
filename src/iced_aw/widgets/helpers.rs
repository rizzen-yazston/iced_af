//! widget Helpers.
//!
//!
#[allow(unused_imports)]
use iced::{self, advanced::renderer, Color, Element};

#[allow(unused_imports)]
use std::{borrow::Cow, fmt::Display, hash::Hash, ops::RangeBounds};

/// Creates a vec of menu items
///
/// [`Item`]: crate::menu::Item
///
/// Syntax:
/// ```
/// menu_items!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu_items {
    ($($x:tt)+) => {
        {
            macro_rules! wrap_item {
                (($i:expr , $m:expr)) => (
                    $crate::iced_aw::widgets::menu::Item::with_menu($i, $m)
                );
                (($i:expr)) => (
                    $crate::iced_aw::widgets::menu::Item::new($i)
                );
            }

            vec![ $( wrap_item!($x) ),+ ]
        }
    }
}

/// Creates a [`Menu`] with the given items.
///
/// [`Menu`]: crate::menu::Menu
///
/// Syntax:
/// ```
/// menu!(
///     (widget)
///     (widget)
///     (widget, menu)
///     (widget)
///     (widget, menu)
///     (widget)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu {
    ($($x:tt)+) => {
        $crate::menu::Menu::new( $crate::menu_items!( $($x)+ ) )
    }
}

/// Creates a [`MenuBar`] with the given children.
///
/// [`MenuBar`]: crate::menu::MenuBar
///
/// Syntax:
/// ```
/// menu_bar!(
///     (widget, menu)
///     (widget, menu)
///     (widget, menu)
///     ...
/// )
/// ```
#[macro_export]
macro_rules! menu_bar {
    ($(($x:expr, $m:expr))+) => (
        $crate::iced_aw::widgets::menu::MenuBar::new(vec![ $( Item::with_menu($x, $m) ),+ ])
    );
}
