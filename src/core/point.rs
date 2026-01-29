//! 2D point in logical pixels.

use crate::core::vector::Vector;

/// A 2D point in logical pixels.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Point {
    pub x: f32,
    pub y: f32,
}

impl Point {
    pub const fn new(x: f32, y: f32) -> Self {
        Self { x, y }
    }

    pub const ZERO: Self = Self { x: 0.0, y: 0.0 };
}

impl std::ops::Add<Vector> for Point {
    type Output = Point;

    fn add(self, vector: Vector) -> Self::Output {
        Point::new(self.x + vector.x, self.y + vector.y)
    }
}

impl std::ops::Sub<Vector> for Point {
    type Output = Point;

    fn sub(self, vector: Vector) -> Self::Output {
        Point::new(self.x - vector.x, self.y - vector.y)
    }
}

impl std::ops::Sub<Point> for Point {
    type Output = Vector;

    fn sub(self, other: Point) -> Self::Output {
        Vector::new(self.x - other.x, self.y - other.y)
    }
}
