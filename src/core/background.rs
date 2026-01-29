//! Background fill for widgets.

/// A background fill.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Background {
    /// Solid color background.
    Color(Color),
}

impl From<Color> for Background {
    fn from(color: Color) -> Self {
        Background::Color(color)
    }
}

/// A color in RGBA format.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Color {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Color {
    pub const fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub const BLACK: Self = Self::new(0.0, 0.0, 0.0, 1.0);
    pub const WHITE: Self = Self::new(1.0, 1.0, 1.0, 1.0);
    pub const TRANSPARENT: Self = Self::new(0.0, 0.0, 0.0, 0.0);

    /// Convert to u32 in 0x00RRGGBB format.
    pub fn to_u32(self) -> u32 {
        let r = (self.r * 255.0).clamp(0.0, 255.0) as u32;
        let g = (self.g * 255.0).clamp(0.0, 255.0) as u32;
        let b = (self.b * 255.0).clamp(0.0, 255.0) as u32;
        (r << 16) | (g << 8) | b
    }
}
