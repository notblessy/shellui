//! Window events.

/// A window event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Window was resized.
    Resized {
        /// New width in logical pixels.
        width: f32,
        /// New height in logical pixels.
        height: f32,
    },
    /// Redraw was requested.
    RedrawRequested,
    /// Window close was requested.
    CloseRequested,
}

/// A request for when to redraw.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum RedrawRequest {
    /// Wait for the next event.
    Wait,
    /// Redraw on the next frame.
    NextFrame,
}

impl RedrawRequest {
    /// Returns the minimum of two redraw requests.
    pub fn min(self, other: Self) -> Self {
        match (self, other) {
            (RedrawRequest::Wait, _) | (_, RedrawRequest::Wait) => RedrawRequest::Wait,
            (RedrawRequest::NextFrame, RedrawRequest::NextFrame) => RedrawRequest::NextFrame,
        }
    }
}
