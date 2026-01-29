//! Core types and traits for shellui.

pub mod widget;
pub mod element;
pub mod tree;
pub mod renderer;
pub mod layout;
pub mod event;
pub mod mouse;
pub mod window;
pub mod shell;
pub mod overlay;
pub mod length;
pub mod point;
pub mod vector;
pub mod transformation;
pub mod background;
pub mod border;
pub mod shadow;

pub use widget::Widget;
pub use element::Element;
pub use tree::{Tag, State, Tree};
pub use renderer::Renderer;
pub use renderer::Renderer as RendererTrait;
pub use layout::Layout;
pub use crate::layout::{Limits, Node};
pub use event::{Event, Status};
pub use mouse::{Cursor, Interaction};
pub use window::{Event as WindowEvent, RedrawRequest};
pub use shell::Shell;
pub use overlay::Element as OverlayElement;
pub use length::Length;
pub use point::Point;
pub use vector::Vector;
pub use transformation::Transformation;
pub use background::{Background, Color};
pub use border::Border;
pub use shadow::Shadow;

// Re-export common types from layout
pub use crate::layout::{Rectangle, Size};
