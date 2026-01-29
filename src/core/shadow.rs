//! Shadow styling for widgets.

use crate::core::background::Color;
use crate::core::vector::Vector;

/// Shadow styling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Shadow {
    pub color: Color,
    pub offset: Vector,
    pub blur: f32,
}

impl Shadow {
    pub const fn new(color: Color, offset: Vector, blur: f32) -> Self {
        Self { color, offset, blur }
    }
}

impl Default for Shadow {
    fn default() -> Self {
        Self {
            color: Color::new(0.0, 0.0, 0.0, 0.2),
            offset: Vector::ZERO,
            blur: 0.0,
        }
    }
}
