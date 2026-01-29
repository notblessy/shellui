//! 2D vector in logical pixels.

use crate::core::point::Point;

/// A 2D vector in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Vector {
    pub x: f32,
    pub y: f32,
}

impl Vector {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl std::ops::Add<Vector> for Vector {
    type Output = Vector;

    fn add(self, other: Vector) -> Self::Output {
        Vector::new(self.x + other.x, self.y + other.y)
    }
}

impl std::ops::Sub<Vector> for Vector {
    type Output = Vector;

    fn sub(self, other: Vector) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}

impl std::ops::Mul<f32> for Vector {
    type Output = Vector;

    fn mul(self, scalar: f32) -> Self::Output {
        Vector::new(self.x * scalar, self.y * scalar)
    }
}

impl From<Point> for Vector {
    fn from(point: Point) -> Self {
        Vector::new(point.x, point.y)
    }
}
