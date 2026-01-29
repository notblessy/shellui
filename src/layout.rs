//! Layout: Limits, Node, and layout pass for View tree.

use crate::view::{Alignment, View};

/// 2D size in logical pixels or length units.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Size<T = f32> {
    pub width: T,
    pub height: T,
}

impl Size<f32> {
    pub const ZERO: Self = Self { width: 0.0, height: 0.0 };
    pub const fn new(width: f32, height: f32) -> Self {
        Self { width, height }
    }
    pub fn max(self, other: Self) -> Self {
        Self {
            width: self.width.max(other.width),
            height: self.height.max(other.height),
        }
    }
}

impl<T> Size<T> {
    pub const fn new_generic(width: T, height: T) -> Self {
        Self { width, height }
    }
}

/// Axis-aligned rectangle (position + size).
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Rectangle {
    pub x: f32,
    pub y: f32,
    pub width: f32,
    pub height: f32,
}

impl Rectangle {
    pub const fn new(x: f32, y: f32, width: f32, height: f32) -> Self {
        Self { x, y, width, height }
    }
    pub fn size(&self) -> Size<f32> {
        Size::new(self.width, self.height)
    }
}

/// Min/max size constraints for layout.
#[derive(Debug, Clone, Copy)]
pub struct Limits {
    pub min_width: f32,
    pub min_height: f32,
    pub max_width: f32,
    pub max_height: f32,
}

impl Limits {
    pub const fn loose(max_width: f32, max_height: f32) -> Self {
        Self {
            min_width: 0.0,
            min_height: 0.0,
            max_width,
            max_height,
        }
    }

    pub fn new(min: Size<f32>, max: Size<f32>) -> Self {
        Self {
            min_width: min.width,
            min_height: min.height,
            max_width: max.width,
            max_height: max.height,
        }
    }

    pub fn constrain_width(self, w: f32) -> Self {
        Self {
            max_width: self.max_width.min(w),
            ..self
        }
    }
    pub fn constrain_height(self, h: f32) -> Self {
        Self {
            max_height: self.max_height.min(h),
            ..self
        }
    }
}

/// Result of layout: size and position of this node and its children.
#[derive(Debug, Clone)]
pub struct Node {
    pub bounds: Rectangle,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            bounds,
            children: Vec::new(),
        }
    }
    pub fn with_children(bounds: Rectangle, children: Vec<Node>) -> Self {
        Self { bounds, children }
    }
    pub fn size(&self) -> Size {
        self.bounds.size()
    }
}

/// Measures text for layout (intrinsic size). Implemented by renderer; placeholder for layout-only.
pub trait TextMeasurer: Send + Sync {
    fn measure(&self, text: &str, font_size: f32) -> Size;
}

/// Placeholder measurer (fixed size per char) when no font is loaded yet.
#[derive(Debug, Default)]
pub struct PlaceholderMeasurer;

impl TextMeasurer for PlaceholderMeasurer {
    fn measure(&self, text: &str, _font_size: f32) -> Size {
        let w = (text.len() as f32 * 8.0).max(1.0);
        Size::new(w, 20.0)
    }
}

/// Runs layout on the view tree; returns root node with bounds and children.
pub fn layout(view: &View, limits: Limits, measurer: &dyn TextMeasurer) -> Node {
    match view {
        View::Text(t) => {
            // Use DEFAULT_FONT_SIZE from render module if not specified
            use crate::render::DEFAULT_FONT_SIZE;
            let font_size = t.size.unwrap_or(DEFAULT_FONT_SIZE);
            let size = measurer.measure(&t.string, font_size);
            let w = size.width.min(limits.max_width).max(limits.min_width);
            let h = size.height.min(limits.max_height).max(limits.min_height);
            Node::new(Rectangle::new(0.0, 0.0, w, h))
        }
        View::VStack(v) => layout_stack(
            view,
            v.spacing,
            v.alignment,
            true, // vertical
            limits,
            measurer,
        ),
        View::HStack(h) => layout_stack(
            view,
            h.spacing,
            h.alignment,
            false, // horizontal
            limits,
            measurer,
        ),
    }
}

fn layout_stack(
    view: &View,
    spacing: f32,
    alignment: Alignment,
    vertical: bool,
    limits: Limits,
    measurer: &dyn TextMeasurer,
) -> Node {
    let (children_views, _) = match view {
        View::VStack(v) => (&v.children, ()),
        View::HStack(h) => (&h.children, ()),
        _ => return Node::new(Rectangle::new(0.0, 0.0, 0.0, 0.0)),
    };

    if children_views.is_empty() {
        return Node::new(Rectangle::new(0.0, 0.0, 0.0, 0.0));
    }

    let total_spacing = spacing * (children_views.len().saturating_sub(1)) as f32;
    let mut child_nodes: Vec<Node> = Vec::with_capacity(children_views.len());
    let mut main_sum = 0.0f32;
    let mut cross_max = 0.0f32;

    for child in children_views {
        let node = layout(child, limits, measurer);
        let s = node.size();
        if vertical {
            main_sum += s.height;
            cross_max = cross_max.max(s.width);
        } else {
            main_sum += s.width;
            cross_max = cross_max.max(s.height);
        }
        child_nodes.push(node);
    }

    let main_size = main_sum + total_spacing;
    let cross_size = cross_max;

    let (total_width, total_height) = if vertical {
        (
            cross_size.min(limits.max_width).max(limits.min_width),
            main_size.min(limits.max_height).max(limits.min_height),
        )
    } else {
        (
            main_size.min(limits.max_width).max(limits.min_width),
            cross_size.min(limits.max_height).max(limits.min_height),
        )
    };

    let align = alignment.factor();
    let mut main_cursor = 0.0f32;

    let positioned: Vec<Node> = child_nodes
        .into_iter()
        .map(|mut node| {
            let (mw, mh) = (node.bounds.width, node.bounds.height);
            let (x, y) = if vertical {
                let cross_offset = (total_width - mw) * align;
                let y = main_cursor;
                main_cursor += mh + spacing;
                (cross_offset, y)
            } else {
                let cross_offset = (total_height - mh) * align;
                let x = main_cursor;
                main_cursor += mw + spacing;
                (x, cross_offset)
            };
            node.bounds = Rectangle::new(x, y, mw, mh);
            node
        })
        .collect();

    Node::with_children(
        Rectangle::new(0.0, 0.0, total_width, total_height),
        positioned,
    )
}
