//! Example: Advanced window configuration with fullscreen and size constraints.

use shellui::{App, Alignment, Color, HStack, Text, VStack, View, window_group};

struct ContentView;

impl ContentView {
    fn new() -> View {
        VStack::new()
            .spacing(20.0)
            .padding(32.0)
            .background(Color::new(0.1, 0.1, 0.1, 1.0))
            .alignment(Alignment::Center)
            .add(
                Text::new("Fullscreen Example")
                    .size(56.0)
            )
            .add(
                Text::new("Press Escape or Cmd+Q to exit fullscreen")
                    .size(18.0)
            )
            .add(
                HStack::new()
                    .spacing(16.0)
                    .padding(20.0)
                    .background(Color::new(0.2, 0.3, 0.5, 1.0))
                    .add(Text::new("Window Size:"))
                    .add(Text::new("1200x800"))
            )
            .add(
                VStack::new()
                    .spacing(10.0)
                    .padding(20.0)
                    .background(Color::new(0.5, 0.2, 0.2, 1.0))
                    .alignment(Alignment::Center)
                    .add(Text::new("Configuration").size(24.0))
                    .add(Text::new("• Min Size: 600x400"))
                    .add(Text::new("• Max Size: 1920x1080"))
                    .add(Text::new("• Resizable: Yes"))
                    .add(Text::new("• Fullscreen: No"))
            )
            .into()
    }
}

// This is equivalent to SwiftUI's @main attribute
struct MyAdvancedApp;

impl App for MyAdvancedApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| ContentView::new())
            .title("Advanced ShellUI App - Custom Sizing")
            .size(1200.0, 800.0)
            .min_size(600.0, 400.0)
            .max_size(1920.0, 1080.0)
            .resizable(true)
            .fullscreen(false)  // Set to true for fullscreen
    }
}

fn main() {
    // In SwiftUI this would be handled by @main attribute
    MyAdvancedApp.run();
}