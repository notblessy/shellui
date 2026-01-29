//! SwiftUI-style App and Scene system.

use crate::View;

/// Window configuration for a WindowGroup scene.
#[derive(Debug, Clone)]
pub struct WindowConfiguration {
    /// Window title
    pub title: String,
    /// Initial window size (width, height)
    pub size: (f32, f32),
    /// Minimum window size (width, height)
    pub min_size: Option<(f32, f32)>,
    /// Maximum window size (width, height)  
    pub max_size: Option<(f32, f32)>,
    /// Whether window should start in fullscreen
    pub fullscreen: bool,
    /// Whether window is resizable
    pub resizable: bool,
}

impl Default for WindowConfiguration {
    fn default() -> Self {
        Self {
            title: "ShellUI".to_string(),
            size: (800.0, 600.0),
            min_size: None,
            max_size: None,
            fullscreen: false,
            resizable: true,
        }
    }
}

impl WindowConfiguration {
    /// Set the window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.title = title.into();
        self
    }

    /// Set the initial window size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.size = (width, height);
        self
    }

    /// Set the minimum window size
    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.min_size = Some((width, height));
        self
    }

    /// Set the maximum window size
    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        self.max_size = Some((width, height));
        self
    }

    /// Enable or disable fullscreen mode
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.fullscreen = fullscreen;
        self
    }

    /// Enable or disable window resizing
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.resizable = resizable;
        self
    }
}

/// A scene in the app, similar to SwiftUI's Scene protocol.
pub enum Scene {
    /// A window group containing a root view
    WindowGroup {
        /// The root view for this window
        content: Box<dyn Fn() -> View>,
        /// Window configuration
        config: WindowConfiguration,
    },
}

impl Scene {
    /// Create a new window group with a content view
    pub fn window_group<F>(content: F) -> Self
    where
        F: Fn() -> View + 'static,
    {
        Self::WindowGroup {
            content: Box::new(content),
            config: WindowConfiguration::default(),
        }
    }

    /// Create a new window group with configuration
    pub fn window_group_with_config<F>(content: F, config: WindowConfiguration) -> Self
    where
        F: Fn() -> View + 'static,
    {
        Self::WindowGroup {
            content: Box::new(content),
            config,
        }
    }
}

/// The main App trait, similar to SwiftUI's App protocol.
pub trait App {
    /// The content and behavior of the app
    fn body(&self) -> impl IntoScene;

    /// Run the application
    fn run(self) 
    where 
        Self: Sized + 'static
    {
        let scene = self.body().into_scene();
        crate::window::run_scene(scene);
    }
}

/// Trait for converting types into Scene
pub trait IntoScene {
    fn into_scene(self) -> Scene;
}

impl IntoScene for Scene {
    fn into_scene(self) -> Scene {
        self
    }
}

/// A WindowGroup builder that implements IntoScene
pub struct WindowGroup<F> {
    content: F,
    config: WindowConfiguration,
}

impl<F> WindowGroup<F>
where
    F: Fn() -> View + 'static,
{
    /// Create a new WindowGroup
    pub fn new(content: F) -> Self {
        Self {
            content,
            config: WindowConfiguration::default(),
        }
    }

    /// Configure the window
    pub fn configure(mut self, config: WindowConfiguration) -> Self {
        self.config = config;
        self
    }

    /// Set window title
    pub fn title(mut self, title: impl Into<String>) -> Self {
        self.config = self.config.title(title);
        self
    }

    /// Set window size
    pub fn size(mut self, width: f32, height: f32) -> Self {
        self.config = self.config.size(width, height);
        self
    }

    /// Set minimum window size
    pub fn min_size(mut self, width: f32, height: f32) -> Self {
        self.config = self.config.min_size(width, height);
        self
    }

    /// Set maximum window size
    pub fn max_size(mut self, width: f32, height: f32) -> Self {
        self.config = self.config.max_size(width, height);
        self
    }

    /// Enable fullscreen
    pub fn fullscreen(mut self, fullscreen: bool) -> Self {
        self.config = self.config.fullscreen(fullscreen);
        self
    }

    /// Enable or disable resizing
    pub fn resizable(mut self, resizable: bool) -> Self {
        self.config = self.config.resizable(resizable);
        self
    }
}

impl<F> IntoScene for WindowGroup<F>
where
    F: Fn() -> View + 'static,
{
    fn into_scene(self) -> Scene {
        Scene::WindowGroup {
            content: Box::new(self.content),
            config: self.config,
        }
    }
}

/// Convenience function to create a WindowGroup
pub fn window_group<F>(content: F) -> WindowGroup<F>
where
    F: Fn() -> View + 'static,
{
    WindowGroup::new(content)
}

/// Macro to simulate SwiftUI's @main attribute
#[macro_export]
macro_rules! main_app {
    ($app_struct:ident) => {
        fn main() {
            $app_struct.run();
        }
    };
}