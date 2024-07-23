//! Stateless, pure widgets for iced
//use iced_widget::{renderer, style};
pub mod helpers;
#[allow(unused_imports)]
pub use helpers::*;

//pub mod overlay;

pub mod common;
pub use common::InnerBounds;

pub mod menu;
pub mod quad;
pub mod sidebar;
/// A sidebar to show tabs on the side.
pub type Sidebar<'a, Message, TabId, Theme, Renderer> =
    sidebar::Sidebar<'a, Message, TabId, Theme, Renderer>;
/// A [`SidebarWithContent`] widget for showing a [`Sidebar`](super::sidebar::SideBar)
pub type SidebarWithContent<'a, Message, TabId, Theme, Renderer> =
    sidebar::SidebarWithContent<'a, Message, TabId, Theme, Renderer>;
