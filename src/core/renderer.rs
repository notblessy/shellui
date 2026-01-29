//! Write your own renderer.

use crate::core::background::Background;
use crate::core::border::Border;
use crate::core::shadow::Shadow;
use crate::layout::Rectangle;
use crate::core::transformation::Transformation;

/// A component that can be used by widgets to draw themselves on a screen.
pub trait Renderer {
    /// Starts recording a new layer.
    fn start_layer(&mut self, bounds: Rectangle);

    /// Ends recording a new layer.
    ///
    /// The new layer will clip its contents to the provided `bounds`.
    fn end_layer(&mut self);

    /// Draws the primitives recorded in the given closure in a new layer.
    ///
    /// The layer will clip its contents to the provided `bounds`.
    fn with_layer(&mut self, bounds: Rectangle, f: impl FnOnce(&mut Self)) {
        self.start_layer(bounds);
        f(self);
        self.end_layer();
    }

    /// Starts recording with a new [`Transformation`].
    fn start_transformation(&mut self, transformation: Transformation);

    /// Ends recording a new transformation.
    fn end_transformation(&mut self);

    /// Applies a [`Transformation`] to the primitives recorded in the given closure.
    fn with_transformation(&mut self, transformation: Transformation, f: impl FnOnce(&mut Self)) {
        self.start_transformation(transformation);
        f(self);
        self.end_transformation();
    }

    /// Applies a translation to the primitives recorded in the given closure.
    fn with_translation(&mut self, translation: crate::core::Vector, f: impl FnOnce(&mut Self)) {
        self.with_transformation(Transformation::translate(translation.x, translation.y), f);
    }

    /// Fills a [`Quad`] with the provided [`Background`].
    fn fill_quad(&mut self, quad: Quad, background: impl Into<Background>);

    /// Resets the [`Renderer`] to start drawing in the `new_bounds` from scratch.
    fn reset(&mut self, new_bounds: Rectangle);

    /// Provides hints to the [`Renderer`] about the rendering target.
    ///
    /// This may be used internally by the [`Renderer`] to perform optimizations
    /// and/or improve rendering quality.
    fn hint(&mut self, scale_factor: f32);

    /// Returns the last scale factor provided as a [`hint`](Self::hint).
    fn scale_factor(&self) -> Option<f32>;

    /// Polls any concurrent computations that may be pending in the [`Renderer`].
    ///
    /// By default, it does nothing.
    fn tick(&mut self) {}
}

/// A polygon with four sides.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct Quad {
    /// The bounds of the [`Quad`].
    pub bounds: Rectangle,

    /// The [`Border`] of the [`Quad`]. The border is drawn on the inside of the [`Quad`].
    pub border: Border,

    /// The [`Shadow`] of the [`Quad`].
    pub shadow: Shadow,

    /// Whether the [`Quad`] should be snapped to the pixel grid.
    pub snap: bool,
}

impl Quad {
    pub fn new(bounds: Rectangle) -> Self {
        Self {
            bounds,
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        }
    }
}

impl Default for Quad {
    fn default() -> Self {
        Self {
            bounds: Rectangle::new(0.0, 0.0, 0.0, 0.0),
            border: Border::default(),
            shadow: Shadow::default(),
            snap: false,
        }
    }
}
