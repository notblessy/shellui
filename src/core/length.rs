//! Length units for widget sizing.

/// A length unit for widget dimensions.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Length {
    /// Shrink to fit content.
    Shrink,
    /// Fill available space.
    Fill,
    /// Fixed size in pixels.
    Fixed(f32),
}

impl Length {
    /// Resolves the length to a concrete pixel value given constraints.
    pub fn resolve(self, available: f32, intrinsic: f32) -> f32 {
        match self {
            Length::Shrink => intrinsic,
            Length::Fill => available,
            Length::Fixed(value) => value.min(available),
        }
    }
}

impl Default for Length {
    fn default() -> Self {
        Length::Shrink
    }
}

impl From<f32> for Length {
    fn from(value: f32) -> Self {
        Length::Fixed(value)
    }
}
