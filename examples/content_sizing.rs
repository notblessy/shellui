//! Example: Different content sizing modes and justify alignments.

use shellui::{App, Alignment, Color, ContentSizing, HStack, Justify, Text, VStack, View, window_group};

struct ContentView;

impl ContentView {
    fn new() -> View {
        VStack::new()
            .spacing(20.0)
            .padding(32.0)
            .background(Color::new(0.9, 0.9, 0.9, 1.0))
            .trailing() // Justify to bottom for VStack
            .add(
                Text::new("Content Sizing Demo")
                    .size(36.0)
                    .color(Color::new(0.1, 0.1, 0.6, 1.0))
            )
            .add(
                HStack::new()
                    .spacing(16.0)
                    .padding(20.0)
                    .background(Color::new(0.7, 0.9, 0.7, 1.0))
                    .leading() // Justify to left for HStack
                    .add(Text::new("Leading").color(Color::new(0.2, 0.2, 0.2, 1.0)))
                    .add(Text::new("Justified").color(Color::new(0.2, 0.2, 0.2, 1.0)))
            )
            .add(
                HStack::new()
                    .spacing(16.0)
                    .padding(20.0)
                    .background(Color::new(0.9, 0.7, 0.7, 1.0))
                    .center_justify() // Justify to center for HStack
                    .add(Text::new("Center").color(Color::new(0.2, 0.2, 0.2, 1.0)))
                    .add(Text::new("Justified").color(Color::new(0.2, 0.2, 0.2, 1.0)))
            )
            .add(
                HStack::new()
                    .spacing(16.0)
                    .padding(20.0)
                    .background(Color::new(0.7, 0.7, 0.9, 1.0))
                    .trailing() // Justify to right for HStack
                    .add(Text::new("Trailing").color(Color::new(0.2, 0.2, 0.2, 1.0)))
                    .add(Text::new("Justified").color(Color::new(0.2, 0.2, 0.2, 1.0)))
            )
            .add(
                Text::new("Fixed 400x300 content size")
                    .size(14.0)
                    .color(Color::new(0.5, 0.5, 0.5, 1.0))
            )
            .into()
    }
}

struct SizingApp;

impl App for SizingApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| ContentView::new())
            .title("Content Sizing: Fixed Size Demo")
            .size(800.0, 600.0)
            .resizable(true)
            .fixed_size(400.0, 300.0) // Fixed content size, centered in window
    }
}

fn main() {
    SizingApp.run();
}