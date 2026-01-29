//! Example: Advanced window configuration with fullscreen and size constraints.

use shellui::{App, Alignment, Color, HStack, Text, VStack, View, window_group};

struct ContentView;

impl ContentView {
    fn new() -> View {
        VStack::new([
            Text::new("Fullscreen Example")
                .size(56.0),
            Text::new("Press Escape or Cmd+Q to exit fullscreen")
                .size(18.0),
            HStack::new([
                Text::new("Window Size:"),
                Text::new("1200x800"),
            ])
            .spacing(16.0)
            .padding(20.0)
            .background(Color::new(0.2, 0.3, 0.5, 1.0)),
            VStack::new([
                Text::new("Configuration").size(24.0),
                Text::new("• Min Size: 600x400"),
                Text::new("• Max Size: 1920x1080"),
                Text::new("• Resizable: Yes"),
                Text::new("• Fullscreen: No"),
            ])
            .spacing(10.0)
            .padding(20.0)
            .background(Color::new(0.5, 0.2, 0.2, 1.0))
            .alignment(Alignment::Center),
        ])
        .spacing(20.0)
        .padding(32.0)
        .background(Color::new(0.1, 0.1, 0.1, 1.0))
        .alignment(Alignment::Center)
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