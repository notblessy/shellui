//! Overlay widgets (popups, menus, tooltips).

/// An overlay element (same as [`crate::core::element::Element`]).
pub type Element<'a, Message, R> = crate::core::element::Element<'a, Message, R>;

/// A group of overlays.
#[derive(Debug)]
pub struct Group<'a, Message, R> {
    overlays: Vec<Element<'a, Message, R>>,
}

impl<'a, Message, R> Group<'a, Message, R>
where
    R: crate::core::renderer::Renderer,
{
    pub fn new() -> Self {
        Self {
            overlays: Vec::new(),
        }
    }

    pub fn push(mut self, overlay: Element<'a, Message, R>) -> Self {
        self.overlays.push(overlay);
        self
    }
}

impl<'a, Message, R> Default for Group<'a, Message, R>
where
    R: crate::core::renderer::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}
