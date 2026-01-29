//! Example: SwiftUI-style App with justify alignment and content sizing.

use shellui::{App, Alignment, Color, ContentSizing, HStack, Justify, Text, VStack, View, window_group};

struct ContentView;

impl ContentView {
    fn new() -> View {
        VStack::new()
            .spacing(16.0)
            .padding(24.0)
            .background(Color::new(0.95, 0.95, 0.95, 1.0))
            .alignment(Alignment::Center)
            .leading() // Justify to top (leading edge for VStack)
            .add(
                Text::new("ShellUI")
                    .size(48.0)
                    .color(Color::new(0.2, 0.2, 0.8, 1.0))
            )
            .add(
                Text::new("SwiftUI-inspired API with Justification")
                    .size(24.0)
                    .color(Color::new(0.4, 0.4, 0.4, 1.0))
            )
            .add(
                HStack::new()
                    .spacing(12.0)
                    .padding(16.0)
                    .background(Color::new(0.8, 0.8, 1.0, 1.0))
                    .space_between() // Justify space-between for HStack
                    .add(Text::new("Leading").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    .add(Text::new("•").color(Color::new(0.5, 0.5, 0.5, 1.0)))
                    .add(Text::new("Center").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    .add(Text::new("•").color(Color::new(0.5, 0.5, 0.5, 1.0)))
                    .add(Text::new("Trailing").color(Color::new(1.0, 1.0, 1.0, 1.0)))
            )
            .add(
                VStack::new()
                    .spacing(8.0)
                    .padding(16.0)
                    .background(Color::new(1.0, 0.9, 0.8, 1.0))
                    .alignment(Alignment::Center)
                    .center_justify() // Center justify for nested VStack
                    .add(Text::new("Nested VStack").size(20.0).color(Color::new(0.8, 0.2, 0.2, 1.0)))
                    .add(Text::new("With center justification").color(Color::new(0.3, 0.3, 0.3, 1.0)))
                    .add(Text::new("Auto-sized content").color(Color::new(0.2, 0.6, 0.2, 1.0)))
            )
            .into()
    }
}

struct MyApp;

impl App for MyApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| ContentView::new())
            .title("ShellUI - Justify & Leading Position Demo")
            .size(600.0, 500.0)
            .min_size(400.0, 300.0)
            .resizable(true)
            .auto_size() // Content auto-sizes
            .leading()   // Position at top-left instead of center
    }
}

fn main() {
    MyApp.run();
}
