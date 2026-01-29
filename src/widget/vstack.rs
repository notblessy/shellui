//! Vertical stack widget with SwiftUI-style API.

use crate::core::element::Element;
use crate::core::layout::Layout;
use crate::layout::{Limits, Node, Size, Rectangle};
use crate::core::length::Length;
use crate::core::mouse::Cursor;
use crate::core::tree::Tree;
use crate::core::widget::Widget;
use crate::core::Color;
use crate::view::Alignment;

/// A vertical stack container that distributes its children vertically.
/// Similar to SwiftUI's VStack.
#[derive(Debug)]
pub struct VStack<'a, Message, Renderer> {
    spacing: f32,
    padding: f32,
    alignment: Alignment,
    background: Option<Color>,
    children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> VStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    pub fn new(children: Vec<Element<'a, Message, Renderer>>) -> Self {
        Self {
            spacing: 8.0,
            padding: 0.0,
            alignment: Alignment::Default,
            background: None,
            children,
        }
    }

    /// Sets the spacing between children
    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    /// Sets the alignment of children
    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    /// Sets padding around the stack (SwiftUI style)
    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    /// Sets background color (SwiftUI style)
    pub fn background(mut self, color: Color) -> Self {
        self.background = Some(color);
        self
    }

    /// Adds a child element
    pub fn push(mut self, child: Element<'a, Message, Renderer>) -> Self {
        self.children.push(child);
        self
    }
}

impl<'a, Message, Renderer> Default for VStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    fn default() -> Self {
        Self::new(Vec::new())
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for VStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(|child| Tree::new(child.as_widget())).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }

    fn size(&self) -> Size<Length> {
        Size::new_generic(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        let padding = self.padding;
        let children_len = self.children.len();
        
        // Layout children and calculate dimensions
        let mut nodes = Vec::new();
        let mut y = padding;
        let mut max_width = 0.0f32;
        
        for (i, child) in self.children.iter_mut().enumerate() {
            let node = child.as_widget_mut().layout(&mut tree.children[i], renderer, limits);
            let size = node.size();
            
            max_width = max_width.max(size.width);
            
            // Position the child
            let mut positioned_node = node;
            positioned_node.bounds = Rectangle::new(padding, y, size.width, size.height);
            nodes.push(positioned_node);
            
            y += size.height;
            if i < children_len - 1 {
                y += self.spacing;
            }
        }
        
        let total_height = y + padding;
        let total_width = max_width + (padding * 2.0);
        
        Node::with_children(
            Rectangle::new(0.0, 0.0, total_width, total_height),
            nodes
        )
    }

    fn draw(
        &self,
        tree: &Tree,
        renderer: &mut Renderer,
        layout: Layout<'_>,
        cursor: Cursor,
        viewport: &Rectangle,
    ) {
        // Draw children
        for ((child, state), layout) in self
            .children
            .iter()
            .zip(&tree.children)
            .zip(layout.children())
        {
            child.as_widget().draw(state, renderer, layout, cursor, viewport);
        }
    }
}

impl<'a, Message, Renderer> From<VStack<'a, Message, Renderer>> for Element<'a, Message, Renderer>
where
    Message: 'a,
    Renderer: crate::core::renderer::Renderer + 'a,
{
    fn from(stack: VStack<'a, Message, Renderer>) -> Self {
        Self::new(stack)
    }
}
