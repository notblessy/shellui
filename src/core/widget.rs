//! Create custom widgets and operate on them.

use crate::core::event::{Event, Status};
use crate::core::layout::Layout;
use crate::layout::{Limits, Node};
use crate::core::length::Length;
use crate::core::mouse::{Cursor, Interaction};
use crate::core::overlay;
use crate::core::shell::Shell;
use crate::core::tree::Tree;
use crate::core::Vector;
use crate::layout::{Rectangle, Size};

/// A component that displays information and allows interaction.
///
/// If you want to build your own widgets, you will need to implement this
/// trait.
pub trait Widget<Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    /// Returns the [`Size`] of the [`Widget`] in lengths.
    fn size(&self) -> Size<Length>;

    /// Returns a [`Size`] hint for laying out the [`Widget`].
    ///
    /// This hint may be used by some widget containers to adjust their sizing strategy
    /// during construction.
    fn size_hint(&self) -> Size<Length> {
        self.size()
    }

    /// Returns the [`layout::Node`] of the [`Widget`].
    ///
    /// This [`layout::Node`] is used by the runtime to compute the [`Layout`] of the
    /// user interface.
    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node;

    /// Draws the [`Widget`] using the associated `Renderer`.
    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    );

    /// Returns the [`Tag`] of the [`Widget`].
    ///
    /// [`Tag`]: crate::core::tree::Tag
    fn tag(&self) -> crate::core::tree::Tag {
        crate::core::tree::Tag::stateless()
    }

    /// Returns the [`State`] of the [`Widget`].
    ///
    /// [`State`]: crate::core::tree::State
    fn state(&self) -> crate::core::tree::State {
        crate::core::tree::State::None
    }

    /// Returns the state [`Tree`] of the children of the [`Widget`].
    fn children(&self) -> Vec<Tree> {
        Vec::new()
    }

    /// Reconciles the [`Widget`] with the provided [`Tree`].
    fn diff(&self, tree: &mut Tree) {
        tree.children.clear();
    }

    /// Processes a runtime [`Event`].
    ///
    /// By default, it does nothing.
    fn update(
        &mut self,
        tree: &mut Tree,
        event: &Event,
        layout: Layout<'_>,
        cursor: Cursor,
        renderer: &Renderer,
        shell: &mut Shell<'_, Message>,
        viewport: &Rectangle,
    ) -> Status {
        let _ = (tree, event, layout, cursor, renderer, shell, viewport);
        Status::Ignored
    }

    /// Returns the current [`mouse::Interaction`] of the [`Widget`].
    ///
    /// [`mouse::Interaction`]: crate::core::mouse::Interaction
    fn mouse_interaction(
        &self,
        tree: &Tree,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
        renderer: &Renderer,
    ) -> Interaction {
        let _ = (tree, layout, cursor, viewport, renderer);
        Interaction::Idle
    }

    /// Returns the overlay of the [`Widget`], if any.
    fn overlay<'b>(
        &'b mut self,
        tree: &'b mut Tree,
        layout: Layout<'b>,
        renderer: &Renderer,
        viewport: &Rectangle,
        translation: Vector,
    ) -> Option<overlay::Element<'b, Message, Renderer>> {
        let _ = (tree, layout, renderer, viewport, translation);
        None
    }
}
