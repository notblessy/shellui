//! Implement your own event loop to drive a user interface.

use crate::core::element::Element;
use crate::core::event::Event;
use crate::core::layout::Layout;
use crate::layout::{Limits, Node};
use crate::core::mouse::{Cursor, Interaction};
use crate::core::shell::Shell;
use crate::core::tree::Tree;
use crate::core::window::RedrawRequest;
use crate::layout::Rectangle;
use crate::layout::Size;

/// A set of interactive graphical elements with a specific [`Layout`].
///
/// It can be updated and drawn.
pub struct UserInterface<'a, Message, Renderer> {
    root: Element<'a, Message, Renderer>,
    base: Node,
    state: Tree,
    overlay: Option<Overlay>,
    bounds: Size,
}

struct Overlay {
    layout: Node,
    interaction: Interaction,
}

impl<'a, Message, Renderer> UserInterface<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    /// Builds a user interface for an [`Element`].
    ///
    /// It is able to avoid expensive computations when using a [`Cache`]
    /// obtained from a previous instance of a [`UserInterface`].
    pub fn build<E: Into<Element<'a, Message, Renderer>>>(
        root: E,
        bounds: Size,
        cache: Cache,
        renderer: &mut Renderer,
    ) -> Self {
        let mut root = root.into();

        let Cache { mut state } = cache;
        state.diff(root.as_widget());

        let base = root.as_widget_mut().layout(
            &mut state,
            renderer,
            &Limits::new(Size::new(0.0, 0.0), bounds),
        );

        UserInterface {
            root,
            base,
            state,
            overlay: None,
            bounds,
        }
    }

    /// Updates the [`UserInterface`] by processing each provided [`Event`].
    ///
    /// It returns __messages__ that may have been produced as a result of user
    /// interactions. You should feed these to your __update logic__.
    pub fn update(
        &mut self,
        events: &[Event],
        cursor: Cursor,
        renderer: &Renderer,
        messages: &mut Vec<Message>,
        viewport: &Rectangle,
    ) -> RedrawRequest {
        let mut redraw_request = RedrawRequest::Wait;

        for event in events {
            let mut shell = Shell::new(messages);

            let _status = self.root.as_widget_mut().update(
                &mut self.state,
                event,
                Layout::new(&self.base),
                cursor,
                renderer,
                &mut shell,
                viewport,
            );

            redraw_request = redraw_request.min(shell.redraw_request());

            if shell.is_layout_invalid() {
                shell.revalidate_layout(|| {
                    self.base = self.root.as_widget_mut().layout(
                        &mut self.state,
                        renderer,
                        &Limits::new(Size::new(0.0, 0.0), self.bounds),
                    );
                });
            }
        }

        redraw_request
    }

    /// Draws the [`UserInterface`] using the provided [`Renderer`].
    pub fn draw(
        &mut self,
        renderer: &mut Renderer,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        self.root.as_widget().draw(
            &self.state,
            renderer,
            Layout::new(&self.base),
            cursor,
            viewport,
        );

        // Draw overlays if any
        if let Some(ref overlay) = self.overlay {
            // Overlay drawing would go here
            let _ = (overlay, renderer, cursor, viewport);
        }
    }

    /// Returns the current [`mouse::Interaction`] of the [`UserInterface`].
    pub fn mouse_interaction(
        &self,
        cursor: Cursor,
        renderer: &Renderer,
        viewport: &Rectangle,
    ) -> Interaction {
        self.root.as_widget().mouse_interaction(
            &self.state,
            Layout::new(&self.base),
            cursor,
            viewport,
            renderer,
        )
    }

    /// Extracts the [`Cache`] from the [`UserInterface`] for reuse in the next frame.
    pub fn into_cache(self) -> Cache {
        Cache { state: self.state }
    }
}

/// A cache for a [`UserInterface`] to avoid expensive recomputations.
#[derive(Debug)]
pub struct Cache {
    pub state: Tree,
}

impl Cache {
    /// Creates a new empty [`Cache`].
    pub fn new() -> Self {
        Self {
            state: Tree::empty(),
        }
    }
}

impl Default for Cache {
    fn default() -> Self {
        Self::new()
    }
}
