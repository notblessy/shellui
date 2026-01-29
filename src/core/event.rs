//! Handle events of a user interface.

use crate::core::mouse;
use crate::core::window;

/// A user interface event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// A mouse event
    Mouse(mouse::Event),

    /// A window event
    Window(window::Event),
}

/// The status of an [`Event`] after being processed.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Status {
    /// The [`Event`] was **NOT** handled by any widget.
    Ignored,

    /// The [`Event`] was handled and processed by a widget.
    Captured,
}

impl Status {
    /// Merges two [`Status`] into one.
    ///
    /// `Captured` takes precedence over `Ignored`.
    pub fn merge(self, b: Self) -> Self {
        match self {
            Status::Ignored => b,
            Status::Captured => Status::Captured,
        }
    }
}
