//! A connection to the state of a shell.

use crate::core::event;
use crate::core::window;

/// A connection to the state of a shell.
///
/// A [`Widget`] can leverage a [`Shell`] to trigger changes in an application,
/// like publishing messages or invalidating the current layout.
#[derive(Debug)]
pub struct Shell<'a, Message> {
    messages: &'a mut Vec<Message>,
    event_status: event::Status,
    redraw_request: window::RedrawRequest,
    is_layout_invalid: bool,
}

impl<'a, Message> Shell<'a, Message> {
    /// Creates a new [`Shell`] with the provided buffer of messages.
    pub fn new(messages: &'a mut Vec<Message>) -> Self {
        Self {
            messages,
            event_status: event::Status::Ignored,
            redraw_request: window::RedrawRequest::Wait,
            is_layout_invalid: false,
        }
    }

    /// Returns true if the [`Shell`] contains no published messages
    #[must_use]
    pub fn is_empty(&self) -> bool {
        self.messages.is_empty()
    }

    /// Publish the given `Message` for an application to process it.
    pub fn publish(&mut self, message: Message) {
        self.messages.push(message);
    }

    /// Marks the current event as captured. Prevents "event bubbling".
    ///
    /// A widget should capture an event when no ancestor should
    /// handle it.
    pub fn capture_event(&mut self) {
        self.event_status = event::Status::Captured;
    }

    /// Returns the current [`event::Status`] of the [`Shell`].
    #[must_use]
    pub fn event_status(&self) -> event::Status {
        self.event_status
    }

    /// Returns whether the current event has been captured.
    #[must_use]
    pub fn is_event_captured(&self) -> bool {
        self.event_status == event::Status::Captured
    }

    /// Requests a new frame to be drawn as soon as possible.
    pub fn request_redraw(&mut self) {
        self.redraw_request = window::RedrawRequest::NextFrame;
    }

    /// Returns the request a redraw should happen, if any.
    #[must_use]
    pub fn redraw_request(&self) -> window::RedrawRequest {
        self.redraw_request
    }

    /// Returns whether the current layout is invalid or not.
    #[must_use]
    pub fn is_layout_invalid(&self) -> bool {
        self.is_layout_invalid
    }

    /// Invalidates the current application layout.
    ///
    /// The shell will relayout the application widgets.
    pub fn invalidate_layout(&mut self) {
        self.is_layout_invalid = true;
    }

    /// Triggers the given function if the layout is invalid, cleaning it in the
    /// process.
    pub fn revalidate_layout(&mut self, f: impl FnOnce()) {
        if self.is_layout_invalid {
            self.is_layout_invalid = false;
            f();
        }
    }

    /// Merges the current [`Shell`] with another one by applying the given
    /// function to the messages of the latter.
    ///
    /// This method is useful for composition.
    pub fn merge<B>(&mut self, other: Shell<'_, B>, f: impl Fn(B) -> Message) {
        self.messages.extend(other.messages.drain(..).map(f));

        self.is_layout_invalid = self.is_layout_invalid || other.is_layout_invalid;

        self.redraw_request = self.redraw_request.min(other.redraw_request);
        self.event_status = self.event_status.merge(other.event_status);
    }
}
