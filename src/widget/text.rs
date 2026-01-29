//! Text widget.

use crate::core::layout::Layout;
use crate::layout::{Limits, Node};
use crate::core::length::Length;
use crate::core::mouse::Cursor;
use crate::core::renderer::Renderer as RendererTrait;
use crate::core::tree::Tree;
use crate::core::widget::Widget;
use crate::layout::{Rectangle, Size, TextMeasurer};
use crate::render::DEFAULT_FONT_SIZE;

/// A text view displaying a string.
#[derive(Debug, Clone)]
pub struct Text {
    pub(crate) string: String,
    pub(crate) size: Option<f32>,
}

impl Text {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            string: s.into(),
            size: None,
        }
    }

    /// Set the font size in pixels. If not set, uses the default size.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

impl<Message, R> Widget<Message, R> for Text
where
    R: RendererTrait + TextMeasurer,
{
    fn size(&self) -> Size<Length> {
        let font_size = self.size.unwrap_or(DEFAULT_FONT_SIZE);
        Size::new_generic(Length::Shrink, Length::Fixed(font_size * 1.25))
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &R,
        limits: &Limits,
    ) -> Node {
        let font_size = self.size.unwrap_or(DEFAULT_FONT_SIZE);
        let size = renderer.measure(&self.string, font_size);
        let w = size.width.min(limits.max_width).max(limits.min_width);
        let h = size.height.min(limits.max_height).max(limits.min_height);
        Node::new(Rectangle::new(0.0, 0.0, w, h))
    }

    fn draw(
        &self,
        _tree: &Tree,
        renderer: &mut R,
        layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        // Note: This requires the renderer to have access to the buffer
        // For now, this is a placeholder - actual drawing happens at the window level
        // The renderer needs to be extended to support drawing with Layout
        let _ = (renderer, layout);
    }
}
