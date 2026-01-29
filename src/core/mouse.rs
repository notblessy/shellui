//! Mouse input handling.

use crate::core::Point;

/// The current state of the mouse cursor.
#[derive(Debug, Clone, Copy, PartialEq, Default)]
pub struct Cursor {
    /// The position of the cursor.
    pub position: Point,
}

impl Cursor {
    pub fn new(position: Point) -> Self {
        Self { position }
    }

    /// Returns whether the cursor is over the given bounds.
    pub fn is_over(&self, bounds: crate::layout::Rectangle) -> bool {
        self.position.x >= bounds.x
            && self.position.x < bounds.x + bounds.width
            && self.position.y >= bounds.y
            && self.position.y < bounds.y + bounds.height
    }
}

/// The visual interaction of the mouse cursor.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Interaction {
    /// Default cursor.
    Idle,
    /// Pointer cursor (hand).
    Pointer,
    /// Grab cursor.
    Grab,
    /// Grabbing cursor.
    Grabbing,
    /// Text selection cursor.
    Text,
}

impl Default for Interaction {
    fn default() -> Self {
        Interaction::Idle
    }
}

/// A mouse event.
#[derive(Debug, Clone, PartialEq)]
pub enum Event {
    /// Mouse button was pressed.
    ButtonPressed {
        /// The button that was pressed.
        button: Button,
        /// The position where the button was pressed.
        position: Point,
    },
    /// Mouse button was released.
    ButtonReleased {
        /// The button that was released.
        button: Button,
        /// The position where the button was released.
        position: Point,
    },
    /// Mouse cursor moved.
    CursorMoved {
        /// The new position of the cursor.
        position: Point,
    },
}

/// A mouse button.
#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Button {
    /// Left mouse button.
    Left,
    /// Right mouse button.
    Right,
    /// Middle mouse button.
    Middle,
    /// Other mouse button.
    Other(u8),
}
