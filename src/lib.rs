//! shellui: SwiftUI-like GUI framework prototype.
//!
//! Window, text rendering, and stacks (VStack, HStack) only.

pub mod app;
pub mod core;
pub mod layout;
pub mod render;
pub mod runtime;
pub mod view;
pub mod widget;
pub mod window;

// Core exports
pub use core::{
    Background, Border, Color, Cursor, Element, Event, Interaction, Layout, Length, Limits, Node,
    Point, RedrawRequest, Renderer as RendererTrait, Shell, Shadow, Size as CoreSize, Status, Tag,
    Tree, Transformation, Vector, WindowEvent,
};

// Layout exports
pub use layout::{layout, PlaceholderMeasurer, Rectangle, Size, TextMeasurer};

// Render exports
pub use render::Renderer;

// Runtime exports
pub use runtime::{Cache, UserInterface};

// View exports (main API)
pub use view::{Alignment, Justify, Button, HStack, Text, VStack, View};

// Widget exports (alternative widget-based API)
pub use widget::{HStack as HStackWidget, VStack as VStackWidget};

// App exports
pub use app::{App, ContentPosition, ContentSizing, IntoScene, Scene, WindowConfiguration, WindowGroup, window_group};

// Window exports
pub use window::{run, run_scene};
