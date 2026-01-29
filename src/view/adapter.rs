//! Adapter to convert View enum to Element for backward compatibility.

use crate::core::element::Element;
use crate::core::widget::Widget;
use crate::core::tree::Tree;
use crate::core::layout::Layout;
use crate::layout::{Limits, Node};
use crate::core::length::Length;
use crate::core::mouse::Cursor;
use crate::layout::{Rectangle, Size};
use crate::view::View;
use crate::layout::{layout, TextMeasurer};

/// Adapter that wraps a View enum variant as a Widget.
struct ViewAdapter {
    view: View,
}

impl<Message, R> Widget<Message, R> for ViewAdapter
where
    R: crate::core::renderer::Renderer + TextMeasurer,
{
    fn size(&self) -> Size<Length> {
        Size::new_generic(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        _tree: &mut Tree,
        renderer: &R,
        limits: &Limits,
    ) -> Node {
        layout(&self.view, Limits {
            min_width: limits.min_width,
            min_height: limits.min_height,
            max_width: limits.max_width,
            max_height: limits.max_height,
        }, renderer)
    }

    fn draw(
        &self,
        _tree: &Tree,
        _renderer: &mut R,
        _layout: Layout<'_>,
        _cursor: Cursor,
        _viewport: &Rectangle,
    ) {
        // Drawing is handled by the old renderer.draw() method at the window level
        // This adapter is mainly for layout compatibility
    }
}

impl From<View> for Element<'static, (), crate::render::Renderer> {
    fn from(view: View) -> Self {
        Element::new(ViewAdapter { view })
    }
}
