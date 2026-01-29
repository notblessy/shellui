//! SwiftUI-like view types: Text, VStack, HStack.

use crate::core::Background;

/// Alignment along the cross axis for stacks.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub enum Alignment {
    Start,
    Center,
    End,
    #[default]
    Default,
}

impl Alignment {
    pub(crate) fn factor(self) -> f32 {
        match self {
            Alignment::Start | Alignment::Default => 0.0,
            Alignment::Center => 0.5,
            Alignment::End => 1.0,
        }
    }
}

/// A text view displaying a string.
#[derive(Debug, Clone)]
pub struct Text {
    pub(crate) string: String,
    pub(crate) size: Option<f32>,
}

impl Text {
    pub fn new(s: impl Into<String>) -> Self {
        Self {
            string: s.into(),
            size: None,
        }
    }

    /// Set the font size in pixels. If not set, uses the default size.
    pub fn size(mut self, size: f32) -> Self {
        self.size = Some(size);
        self
    }
}

/// Vertical stack of views.
#[derive(Debug, Clone)]
pub struct VStack {
    pub(crate) spacing: f32,
    pub(crate) alignment: Alignment,
    pub(crate) padding: f32,
    pub(crate) background: Option<Background>,
    pub(crate) children: Vec<View>,
}

impl VStack {
    pub fn new() -> Self {
        Self {
            spacing: 0.0,
            alignment: Alignment::Default,
            padding: 0.0,
            background: None,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn background<B: Into<Background>>(mut self, background: B) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn push(mut self, child: View) -> Self {
        self.children.push(child);
        self
    }

    pub fn add<V: Into<View>>(mut self, child: V) -> Self {
        self.children.push(child.into());
        self
    }
}

impl Default for VStack {
    fn default() -> Self {
        Self::new()
    }
}

/// Horizontal stack of views.
#[derive(Debug, Clone)]
pub struct HStack {
    pub(crate) spacing: f32,
    pub(crate) alignment: Alignment,
    pub(crate) padding: f32,
    pub(crate) background: Option<Background>,
    pub(crate) children: Vec<View>,
}

impl HStack {
    pub fn new() -> Self {
        Self {
            spacing: 0.0,
            alignment: Alignment::Default,
            padding: 0.0,
            background: None,
            children: Vec::new(),
        }
    }

    pub fn spacing(mut self, spacing: f32) -> Self {
        self.spacing = spacing;
        self
    }

    pub fn alignment(mut self, alignment: Alignment) -> Self {
        self.alignment = alignment;
        self
    }

    pub fn padding(mut self, padding: f32) -> Self {
        self.padding = padding;
        self
    }

    pub fn background<B: Into<Background>>(mut self, background: B) -> Self {
        self.background = Some(background.into());
        self
    }

    pub fn push(mut self, child: View) -> Self {
        self.children.push(child);
        self
    }

    pub fn add<V: Into<View>>(mut self, child: V) -> Self {
        self.children.push(child.into());
        self
    }
}

impl Default for HStack {
    fn default() -> Self {
        Self::new()
    }
}

/// A view in the tree: text or a stack of child views.
#[derive(Debug, Clone)]
pub enum View {
    Text(Text),
    VStack(VStack),
    HStack(HStack),
}

impl View {
    pub fn text(s: impl Into<String>) -> Self {
        Self::Text(Text::new(s))
    }

    pub fn vstack(children: Vec<View>) -> Self {
        Self::VStack(VStack {
            spacing: 0.0,
            alignment: Alignment::Default,
            padding: 0.0,
            background: None,
            children,
        })
    }

    pub fn hstack(children: Vec<View>) -> Self {
        Self::HStack(HStack {
            spacing: 0.0,
            alignment: Alignment::Default,
            padding: 0.0,
            background: None,
            children,
        })
    }
}

impl From<Text> for View {
    fn from(t: Text) -> Self {
        Self::Text(t)
    }
}

impl From<VStack> for View {
    fn from(v: VStack) -> Self {
        Self::VStack(v)
    }
}

impl From<HStack> for View {
    fn from(h: HStack) -> Self {
        Self::HStack(h)
    }
}

// Re-export widget types for convenience
pub use crate::widget::{Text as TextWidget, VStack as VStackWidget, HStack as HStackWidget};

// Adapter module for View to Element conversion
pub mod adapter;

/// Builds a vertical stack of views. Example: `vstack![Text::new("A"), Text::new("B")]`
#[macro_export]
macro_rules! vstack {
    ($($child:expr),+ $(,)?) => {
        $crate::View::VStack($crate::VStack {
            spacing: 0.0,
            alignment: $crate::Alignment::Default,
            padding: 0.0,
            background: None,
            children: vec![$(($child).into()),+],
        })
    };
}

/// Builds a horizontal stack of views. Example: `hstack![Text::new("A"), Text::new("B")]`
#[macro_export]
macro_rules! hstack {
    ($($child:expr),+ $(,)?) => {
        $crate::View::HStack($crate::HStack {
            spacing: 0.0,
            alignment: $crate::Alignment::Default,
            padding: 0.0,
            background: None,
            children: vec![$(($child).into()),+],
        })
    };
}
