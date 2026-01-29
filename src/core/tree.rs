//! Store internal widget state in a state tree to ensure continuity.

use crate::core::widget::Widget;
use std::any::{self, Any};
use std::borrow::Borrow;

/// A persistent state widget tree.
///
/// A [`Tree`] is normally associated with a specific widget in the widget tree.
#[derive(Debug)]
pub struct Tree {
    /// The tag of the [`Tree`].
    pub tag: Tag,

    /// The [`State`] of the [`Tree`].
    pub state: State,

    /// The children of the root widget of the [`Tree`].
    pub children: Vec<Tree>,
}

impl Tree {
    /// Creates an empty, stateless [`Tree`] with no children.
    pub fn empty() -> Self {
        Self {
            tag: Tag::stateless(),
            state: State::None,
            children: Vec::new(),
        }
    }

    /// Creates a new [`Tree`] for the provided [`Widget`].
    pub fn new<'a, Message, Renderer>(
        widget: impl Borrow<dyn Widget<Message, Renderer> + 'a>,
    ) -> Self
    where
        Renderer: crate::core::renderer::Renderer,
    {
        let widget = widget.borrow();

        Self {
            tag: widget.tag(),
            state: widget.state(),
            children: widget.children(),
        }
    }

    /// Reconciles the current tree with the provided [`Widget`].
    ///
    /// If the tag of the [`Widget`] matches the tag of the [`Tree`], then the
    /// [`Widget`] proceeds with the reconciliation (i.e. [`Widget::diff`] is called).
    ///
    /// Otherwise, the whole [`Tree`] is recreated.
    pub fn diff<'a, Message, Renderer>(
        &mut self,
        new: impl Borrow<dyn Widget<Message, Renderer> + 'a>,
    ) where
        Renderer: crate::core::renderer::Renderer,
    {
        if self.tag == new.borrow().tag() {
            new.borrow().diff(self);
        } else {
            *self = Self::new(new);
        }
    }

    /// Reconciles the children of the tree with the provided list of widgets.
    pub fn diff_children<'a, Message, Renderer>(
        &mut self,
        new_children: &[impl Borrow<dyn Widget<Message, Renderer> + 'a>],
    ) where
        Renderer: crate::core::renderer::Renderer,
    {
        self.diff_children_custom(
            new_children,
            |tree, widget| tree.diff(widget.borrow()),
            |widget| Self::new(widget.borrow()),
        );
    }

    /// Reconciles the children of the tree with the provided list of widgets using custom
    /// logic both for diffing and creating new widget state.
    pub fn diff_children_custom<T>(
        &mut self,
        new_children: &[T],
        diff: impl Fn(&mut Tree, &T),
        new_state: impl Fn(&T) -> Self,
    ) {
        if self.children.len() > new_children.len() {
            self.children.truncate(new_children.len());
        }

        for (child_state, new) in self.children.iter_mut().zip(new_children.iter()) {
            diff(child_state, new);
        }

        if self.children.len() < new_children.len() {
            self.children
                .extend(new_children[self.children.len()..].iter().map(new_state));
        }
    }
}

/// The identifier of some widget state.
#[derive(Debug, Clone, Copy, PartialOrd, Ord, PartialEq, Eq, Hash)]
pub struct Tag(any::TypeId);

impl Tag {
    /// Creates a [`Tag`] for a state of type `T`.
    pub fn of<T>() -> Self
    where
        T: 'static,
    {
        Self(any::TypeId::of::<T>())
    }

    /// Creates a [`Tag`] for a stateless widget.
    pub fn stateless() -> Self {
        Self::of::<()>()
    }
}

/// The internal [`State`] of a widget.
#[derive(Debug)]
pub enum State {
    /// No meaningful internal state.
    None,

    /// Some meaningful internal state.
    Some(Box<dyn Any>),
}

impl State {
    /// Creates a new [`State`].
    pub fn new<T: 'static>(value: T) -> Self {
        State::Some(Box::new(value))
    }

    /// Downcasts the state to a reference of type `T`.
    pub fn downcast_ref<T: 'static>(&self) -> Option<&T> {
        match self {
            State::None => None,
            State::Some(value) => value.downcast_ref(),
        }
    }

    /// Downcasts the state to a mutable reference of type `T`.
    pub fn downcast_mut<T: 'static>(&mut self) -> Option<&mut T> {
        match self {
            State::None => None,
            State::Some(value) => value.downcast_mut(),
        }
    }
}
