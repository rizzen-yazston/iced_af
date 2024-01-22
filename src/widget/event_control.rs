// This file is part of `iced_af` crate. For the terms of use, please see the file
// called LICENSE-BSD-3-Clause at the top level of the `iced_af` crate.

use iced::{
    advanced::{
        layout,
        mouse,
        renderer,
        widget::{ self, Operation, Tree, tree },
        Clipboard,
        Layout,
        Shell,
        Widget,
    },
    alignment,
    event,
    overlay,
    widget::container::{ Appearance, StyleSheet },
    Alignment,
    Background,
    Color,
    Element,
    Event,
    Length,
    Padding,
    Pixels,
    //Point,
    Rectangle,
    Size,
};
use std::hash::Hash;

/// A container widget (based on original iced::widget::Container) to include a boolean to toggle the handling of
/// events. When container is in disabled mode, the events generated on the various children are simply ignored, and
/// thus are not queued. When contain is in enabled mode, the events are handled and queued as normal.
/// 
/// This widget is primary used as the root widget of windows which require all widgets of a window to be disabled
/// while a popup window is displayed.
#[allow(missing_debug_implementations)]
pub struct Container<'a, Message, Renderer = iced::Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    id: Option<Id>,
    padding: Padding,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    style: <Renderer::Theme as StyleSheet>::Style,
    content: Element<'a, Message, Renderer>,
    events_enabled: bool,
}

impl<'a, Message, Renderer> Container<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    /// Creates an empty [`Container`].
    pub fn new<T>( content: T, events_enabled: bool ) -> Self
    where
        T: Into<Element<'a, Message, Renderer>>,
    {
        Container {
            id: None,
            padding: Padding::ZERO,
            width: Length::Shrink,
            height: Length::Shrink,
            max_width: f32::INFINITY,
            max_height: f32::INFINITY,
            horizontal_alignment: alignment::Horizontal::Left,
            vertical_alignment: alignment::Vertical::Top,
            style: Default::default(),
            content: content.into(),
            events_enabled,
        }
    }

    /// Sets the [`Id`] of the [`Container`].
    pub fn id( mut self, id: Id ) -> Self {
        self.id = Some( id );
        self
    }

    /// Sets the [`Padding`] of the [`Container`].
    pub fn padding<P: Into<Padding>>( mut self, padding: P ) -> Self {
        self.padding = padding.into();
        self
    }

    /// Sets the width of the [`Container`].
    pub fn width( mut self, width: impl Into<Length> ) -> Self {
        self.width = width.into();
        self
    }

    /// Sets the height of the [`Container`].
    pub fn height( mut self, height: impl Into<Length> ) -> Self {
        self.height = height.into();
        self
    }

    /// Sets the maximum width of the [`Container`].
    pub fn max_width( mut self, max_width: impl Into<Pixels> ) -> Self {
        self.max_width = max_width.into().0;
        self
    }

    /// Sets the maximum height of the [`Container`].
    pub fn max_height( mut self, max_height: impl Into<Pixels> ) -> Self {
        self.max_height = max_height.into().0;
        self
    }

    /// Sets the content alignment for the horizontal axis of the [`Container`].
    pub fn align_x( mut self, alignment: alignment::Horizontal ) -> Self {
        self.horizontal_alignment = alignment;
        self
    }

    /// Sets the content alignment for the vertical axis of the [`Container`].
    pub fn align_y( mut self, alignment: alignment::Vertical ) -> Self {
        self.vertical_alignment = alignment;
        self
    }

    /// Centers the contents in the horizontal axis of the [`Container`].
    pub fn center_x( mut self ) -> Self {
        self.horizontal_alignment = alignment::Horizontal::Center;
        self
    }


    /// Centers the contents in the vertical axis of the [`Container`].
    pub fn center_y( mut self ) -> Self {
        self.vertical_alignment = alignment::Vertical::Center;
        self
    }

    /// Sets the style of the [`Container`].
    pub fn style(
        mut self,
        style: impl Into<<Renderer::Theme as StyleSheet>::Style>,
    ) -> Self {
        self.style = style.into();
        self
    }
}


impl<'a, Message, Renderer> Widget<Message, Renderer> for Container<'a, Message, Renderer>
where
    Renderer: iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn tag( &self ) -> tree::Tag {
        self.content.as_widget().tag()
    }

    fn state( &self ) -> tree::State {
        self.content.as_widget().state()
    }

    fn children( &self ) -> Vec<Tree> {
        self.content.as_widget().children()
    }

    fn diff( &self, tree: &mut Tree ) {
        self.content.as_widget().diff( tree );
    }

    fn size( &self ) -> Size<Length> {
        Size { width: self.width, height: self.height }
    }

    fn layout(
        &self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &layout::Limits,
    ) -> layout::Node {
        layout(
            limits,
            self.width,
            self.height,
            self.max_width,
            self.max_height,
            self.padding,
            self.horizontal_alignment,
            self.vertical_alignment,
            |limits| self.content.as_widget().layout( tree, renderer, limits ),
        )
    }

    fn operate(
        &self,
        tree: &mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
        operation: &mut dyn Operation<Message>,
    ) {
        operation.container(
            self.id.as_ref().map( |id| &id.0 ),
            layout.bounds(),
            &mut |operation| {
                self.content.as_widget().operate(
                    tree,
                    layout.children().next().unwrap(),
                    renderer,
                    operation,
                );
            },
        );
    }

    fn on_event(
        &mut self,
        tree: &mut Tree,
        event: Event,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        renderer: &Renderer,
        clipboard: &mut dyn Clipboard,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> event::Status {
        if !self.events_enabled {
            return event::Status::Ignored;
        }
        self.content.as_widget_mut().on_event(
            tree,
            event,
            layout.children().next().unwrap(),
            cursor,
            renderer,
            clipboard,
            shell,
            viewport,
        )
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> mouse::Interaction {
        self.content.as_widget().mouse_interaction(
            tree,
            layout.children().next().unwrap(),
            cursor,
            viewport,
            renderer,
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        theme: &Renderer::Theme,
        renderer_style: &renderer::Style,
        layout: Layout<'_>,
        cursor: mouse::Cursor,
        viewport: &Rectangle,
    ) {
        let style = theme.appearance( &self.style );

        if let Some( viewport ) = layout.bounds().intersection( viewport ) {
            draw_background( renderer, &style, layout.bounds() );

            self.content.as_widget().draw(
                tree,
                renderer,
                theme,
                &renderer::Style {
                    text_color: style
                        .text_color
                        .unwrap_or( renderer_style.text_color ),
                },
                layout.children().next().unwrap(),
                cursor,
                &viewport,
            );
        }
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'_>,
        renderer: &Renderer,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        self.content.as_widget_mut().overlay(
            tree,
            layout.children().next().unwrap(),
            renderer,
        )
    }
}

impl<'a, Message, Renderer> From<Container<'a, Message, Renderer>>
    for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: 'a + iced::advanced::Renderer,
    Renderer::Theme: StyleSheet,
{
    fn from(
        container: Container<'a, Message, Renderer>,
    ) -> Element<'a, Message, Renderer> {
        Element::new( container )
    }
}

/// Computes the layout of a [`Container`].
pub fn layout(
    limits: &layout::Limits,
    width: Length,
    height: Length,
    max_width: f32,
    max_height: f32,
    padding: Padding,
    horizontal_alignment: alignment::Horizontal,
    vertical_alignment: alignment::Vertical,
    layout_content: impl FnOnce( &layout::Limits ) -> layout::Node,
) -> layout::Node {
    layout::positioned(
        &limits.max_width( max_width ).max_height( max_height ),
        width,
        height,
        padding,
        |limits| layout_content( &limits.loose() ),
        |content, size| {
            content.align(
                Alignment::from( horizontal_alignment ),
                Alignment::from( vertical_alignment ),
                size,
            )
        },
    )
}

/// Draws the background of a [`Container`] given its [`Appearance`] and its `bounds`.
pub fn draw_background<Renderer>(
    renderer: &mut Renderer,
    appearance: &Appearance,
    bounds: Rectangle,
) where
    Renderer: iced::advanced::Renderer,
{
    if appearance.background.is_some()
        || appearance.border.width > 0.0
        || appearance.shadow.color.a > 0.0
    {
        renderer.fill_quad(
            renderer::Quad {
                bounds,
                border: appearance.border,
                shadow: appearance.shadow,
            },
            appearance
                .background
                .unwrap_or( Background::Color( Color::TRANSPARENT ) ),
        );
    }
}

/// The identifier of a [`Container`].
#[derive( Debug, Clone, PartialEq, Eq, Hash )]
pub struct Id( widget::Id );

impl Id {
    /// Creates a custom [`Id`].
    pub fn new( id: impl Into<std::borrow::Cow<'static, str>> ) -> Self {
        Self( widget::Id::new( id ) )
    }

    /// Creates a unique [`Id`].
    ///
    /// This function produces a different [`Id`] every time it is called.
    pub fn unique() -> Self {
        Self( widget::Id::unique() )
    }
}

