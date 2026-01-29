//! A generic widget wrapper.

use crate::core::event::{Event, Status};
use crate::core::layout::Layout;
use crate::core::length::Length;
use crate::core::mouse::{Cursor, Interaction};
use crate::layout::{Limits, Node};
use crate::core::overlay;
use crate::core::shell::Shell;
use crate::core::tree::Tree;
use crate::core::Vector;
use crate::core::widget::Widget;
use crate::layout::{Rectangle, Size};

use std::borrow::Borrow;

/// A generic [`Widget`].
///
/// It is useful to build composable user interfaces that do not leak
/// implementation details in their __view logic__.
pub struct Element<'a, Message, Renderer> {
    widget: Box<dyn Widget<Message, Renderer> + 'a>,
}

impl<'a, Message, Renderer> std::fmt::Debug for Element<'a, Message, Renderer> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("Element").finish_non_exhaustive()
    }
}

impl<'a, Message, Renderer> Element<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    /// Creates a new [`Element`] containing the given [`Widget`].
    pub fn new(widget: impl Widget<Message, Renderer> + 'a) -> Self {
        Self {
            widget: Box::new(widget),
        }
    }

    /// Returns a reference to the [`Widget`] of the [`Element`],
    pub fn as_widget(&self) -> &dyn Widget<Message, Renderer> {
        self.widget.as_ref()
    }

    /// Returns a mutable reference to the [`Widget`] of the [`Element`],
    pub fn as_widget_mut(&mut self) -> &mut dyn Widget<Message, Renderer> {
        self.widget.as_mut()
    }

    /// Applies a transformation to the produced message of the [`Element`].
    ///
    /// This method is useful when you want to decouple different parts of your
    /// UI and make them __composable__.
    pub fn map<B>(self, f: impl Fn(Message) -> B + 'a) -> Element<'a, B, Renderer>
    where
        Message: 'a,
        B: 'a,
        Renderer: 'a,
    {
        Element::new(Map::new(self.widget, f))
    }
}

impl<'a, Message, Renderer> Borrow<dyn Widget<Message, Renderer> + 'a>
    for Element<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    fn borrow(&self) -> &(dyn Widget<Message, Renderer> + 'a) {
        self.widget.as_ref()
    }
}

struct Map<'a, A, B, Renderer> {
    widget: Box<dyn Widget<A, Renderer> + 'a>,
    mapper: Box<dyn Fn(A) -> B + 'a>,
}

impl<'a, A, B, Renderer> Map<'a, A, B, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    pub fn new<F>(
        widget: Box<dyn Widget<A, Renderer> + 'a>,
        mapper: F,
    ) -> Map<'a, A, B, Renderer>
    where
        F: 'a + Fn(A) -> B,
    {
        Map {
            widget,
            mapper: Box::new(mapper),
        }
    }
}

impl<'a, A, B, Renderer> Widget<B, Renderer> for Map<'a, A, B, Renderer>
where
    Renderer: crate::core::renderer::Renderer + 'a,
    A: 'a,
    B: 'a,
{
    fn tag(&self) -> crate::core::tree::Tag {
        self.widget.tag()
    }

    fn state(&self) -> crate::core::tree::State {
        self.widget.state()
    }

    fn children(&self) -> Vec<Tree> {
        self.widget.children()
    }

    fn diff(&self, tree: &mut Tree) {
        self.widget.diff(tree);
    }

    fn size(&self) -> Size<Length> {
        self.widget.size()
    }

    fn size_hint(&self) -> Size<Length> {
        self.widget.size_hint()
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        self.widget.layout(tree, renderer, limits)
    }

    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, B>,
        viewport: &Rectangle,
    ) -> Status {
        let mut local_messages = Vec::new();
        let mut local_shell = Shell::new(&mut local_messages);

        let status = self.widget.update(
            tree,
            event,
            layout,
            cursor,
            renderer,
            &mut local_shell,
            viewport,
        );

        shell.merge(local_shell, &self.mapper);
        status
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.widget.draw(tree, renderer, layout, cursor, viewport);
    }

    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        self.widget.mouse_interaction(tree, layout, cursor, viewport, renderer)
    }

    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, B, Renderer>> {
        // Note: Overlay mapping is more complex and would require a wrapper
        // For now, return None
        let _ = (tree, layout, renderer, viewport, translation);
        None
    }
}
