//! 2D transformation matrix.

/// A 2D transformation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Transformation {
    pub a: f32,
    pub b: f32,
    pub c: f32,
    pub d: f32,
    pub e: f32,
    pub f: f32,
}

impl Transformation {
    /// Identity transformation.
    pub const fn identity() -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Translation transformation.
    pub fn translate(x: f32, y: f32) -> Self {
        Self {
            a: 1.0,
            b: 0.0,
            c: 0.0,
            d: 1.0,
            e: x,
            f: y,
        }
    }

    /// Scale transformation.
    pub fn scale(x: f32, y: f32) -> Self {
        Self {
            a: x,
            b: 0.0,
            c: 0.0,
            d: y,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Rotation transformation (in radians).
    pub fn rotate(angle: f32) -> Self {
        let cos = angle.cos();
        let sin = angle.sin();
        Self {
            a: cos,
            b: sin,
            c: -sin,
            d: cos,
            e: 0.0,
            f: 0.0,
        }
    }

    /// Scale factor for this transformation.
    pub fn scale_factor(&self) -> f32 {
        (self.a * self.a + self.b * self.b).sqrt()
    }
}

impl Default for Transformation {
    fn default() -> Self {
        Self::identity()
    }
}
