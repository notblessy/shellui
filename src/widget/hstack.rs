//! Horizontal stack widget.

use crate::core::element::Element;
use crate::core::layout::Layout;
use crate::layout::{Limits, Node};
use crate::core::length::Length;
use crate::core::mouse::Cursor;
use crate::core::tree::Tree;
use crate::core::widget::Widget;
use crate::layout::{Rectangle, Size};
use crate::view::Alignment;

/// Horizontal stack of widgets.
#[derive(Debug)]
pub struct HStack<'a, Message, Renderer> {
    pub(crate) spacing: f32,
    pub(crate) alignment: Alignment,
    pub(crate) children: Vec<Element<'a, Message, Renderer>>,
}

impl<'a, Message, Renderer> HStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    pub fn new() -> Self {
        Self {
            spacing: 0.0,
            alignment: Alignment::Default,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn push(mut self, child: Element<'a, Message, Renderer>) -> Self {
        self.children.push(child);
        self
    }
}

impl<'a, Message, Renderer> Default for HStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    fn default() -> Self {
        Self::new()
    }
}

impl<'a, Message, Renderer> Widget<Message, Renderer> for HStack<'a, Message, Renderer>
where
    Renderer: crate::core::renderer::Renderer,
{
    fn size(&self) -> Size<Length> {
        Size::new_generic(Length::Shrink, Length::Shrink)
    }

    fn layout(
        &mut self,
        tree: &mut Tree,
        renderer: &Renderer,
        limits: &Limits,
    ) -> Node {
        if self.children.is_empty() {
            return Node::new(Rectangle::new(0.0, 0.0, 0.0, 0.0));
        }

        // Ensure tree has enough children
        tree.diff_children(&self.children);

        let total_spacing = self.spacing * (self.children.len().saturating_sub(1)) as f32;
        let mut child_nodes: Vec<Node> = Vec::with_capacity(self.children.len());
        let mut main_sum = 0.0f32;
        let mut cross_max = 0.0f32;

        for (child, child_tree) in self.children.iter_mut().zip(tree.children.iter_mut()) {
            let node = child.as_widget_mut().layout(child_tree, renderer, limits);
            let s = node.size();
            main_sum += s.width;
            cross_max = cross_max.max(s.height);
            child_nodes.push(node);
        }

        let main_size = main_sum + total_spacing;
        let cross_size = cross_max;

        let total_width = main_size.min(limits.max_width).max(limits.min_width);
        let total_height = cross_size.min(limits.max_height).max(limits.min_height);

        let align = self.alignment.factor();
        let mut main_cursor = 0.0f32;

        let positioned: Vec<Node> = child_nodes
            .into_iter()
            .map(|mut node| {
                let (mw, mh) = (node.bounds.width, node.bounds.height);
                let cross_offset = (total_height - mh) * align;
                let x = main_cursor;
                main_cursor += mw + self.spacing;
                node.bounds = Rectangle::new(x, cross_offset, mw, mh);
                node
            })
            .collect();

        Node::with_children(
            Rectangle::new(0.0, 0.0, total_width, total_height),
            positioned,
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
        let mut children_iter = layout.children();
        for (child, child_tree) in self.children.iter().zip(tree.children.iter()) {
            if let Some(child_layout) = children_iter.next() {
                child.as_widget().draw(child_tree, renderer, child_layout, cursor, viewport);
            }
        }
    }

    fn children(&self) -> Vec<Tree> {
        self.children.iter().map(|child| Tree::new(child.as_widget())).collect()
    }

    fn diff(&self, tree: &mut Tree) {
        tree.diff_children(&self.children);
    }
}
