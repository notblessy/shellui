//! Example: Fill window content sizing mode.

use shellui::{App, Alignment, Color, HStack, Text, VStack, View, window_group};

struct FullWindowView;

impl FullWindowView {
    fn new() -> View {
        VStack::new()
            .spacing(24.0)
            .padding(40.0)
            .background(Color::new(0.2, 0.2, 0.3, 1.0))
            .center_justify() // Center content vertically
            .add(
                Text::new("Full Window Layout")
                    .size(42.0)
                    .color(Color::new(1.0, 1.0, 1.0, 1.0))
            )
            .add(
                HStack::new()
                    .spacing(20.0)
                    .space_between() // Distribute horizontally
                    .add(
                        VStack::new()
                            .padding(16.0)
                            .background(Color::new(0.8, 0.3, 0.3, 1.0))
                            .add(Text::new("Left").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                            .add(Text::new("Panel").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    )
                    .add(
                        VStack::new()
                            .padding(16.0)
                            .background(Color::new(0.3, 0.8, 0.3, 1.0))
                            .add(Text::new("Center").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                            .add(Text::new("Panel").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    )
                    .add(
                        VStack::new()
                            .padding(16.0)
                            .background(Color::new(0.3, 0.3, 0.8, 1.0))
                            .add(Text::new("Right").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                            .add(Text::new("Panel").color(Color::new(1.0, 1.0, 1.0, 1.0)))
                    )
            )
            .add(
                Text::new("Content fills entire window and resizes with it")
                    .size(16.0)
                    .color(Color::new(0.8, 0.8, 0.8, 1.0))
            )
            .into()
    }
}

struct FillWindowApp;

impl App for FillWindowApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| FullWindowView::new())
            .title("Fill Window Demo")
            .size(900.0, 600.0)
            .resizable(true)
            .fill_window() // Content fills entire window
    }
}

fn main() {
    FillWindowApp.run();
}