//! Border styling for widgets.

use crate::core::background::Color;

/// Border styling.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Border {
    pub color: Color,
    pub width: f32,
    pub radius: f32,
}

impl Border {
    pub const fn new(color: Color, width: f32, radius: f32) -> Self {
        Self { color, width, radius }
    }
}

impl Default for Border {
    fn default() -> Self {
        Self {
            color: Color::BLACK,
            width: 0.0,
            radius: 0.0,
        }
    }
}
