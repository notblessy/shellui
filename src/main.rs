//! Example: SwiftUI-style App with @main equivalent.

use shellui::{App, Alignment, Color, HStack, Text, VStack, View, window_group};

struct ContentView;

impl ContentView {
    fn new() -> View {
        VStack::new()
            .spacing(16.0)
            .padding(24.0)
            .background(Color::new(0.95, 0.95, 0.95, 1.0))
            .alignment(Alignment::Center)
            .add(
                Text::new("ShellUI")
                    .size(48.0)
                    .color(Color::new(0.2, 0.2, 0.8, 1.0))
            )
            .add(
                Text::new("SwiftUI-inspired API")
                    .size(24.0)
                    .color(Color::new(0.4, 0.4, 0.4, 1.0))
            )
            .add(
                HStack::new()
                    .spacing(12.0)
                    .padding(16.0)
                    .background(Color::new(0.8, 0.8, 1.0, 1.0))
                    .add(Text::new("HStack").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    .add(Text::new("•").color(Color::new(0.5, 0.5, 0.5, 1.0)))
                    .add(Text::new("with").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    .add(Text::new("•").color(Color::new(0.5, 0.5, 0.5, 1.0)))
                    .add(Text::new("spacing").color(Color::new(1.0, 1.0, 1.0, 1.0)))
            )
            .add(
                VStack::new()
                    .spacing(8.0)
                    .padding(16.0)
                    .background(Color::new(1.0, 0.9, 0.8, 1.0))
                    .alignment(Alignment::Center)
                    .add(Text::new("Nested VStack").size(20.0).color(Color::new(0.8, 0.2, 0.2, 1.0)))
                    .add(Text::new("Method chaining like SwiftUI").color(Color::new(0.3, 0.3, 0.3, 1.0)))
                    .add(Text::new("With padding & background colors").color(Color::new(0.2, 0.6, 0.2, 1.0)))
            )
            .into()
    }
}

struct MyApp;

impl App for MyApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| ContentView::new())
            .title("My ShellUI App")
            .size(400.0, 400.0)
            .min_size(400.0, 300.0)
            .resizable(true)
    }
}

fn main() {
    MyApp.run();
}

// Alternative: using macro equivalent to SwiftUI's @main
// main_app!(MyApp);
