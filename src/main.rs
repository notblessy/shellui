//! Example: SwiftUI-style UI with VStack, HStack, padding, and backgrounds.

use shellui::{Alignment, Color, HStack, Text, VStack};

fn main() {
    let view = || {
        VStack::new()
            .spacing(16.0)
            .padding(24.0)
            .background(Color::new(0.95, 0.95, 0.95, 1.0))
            .alignment(Alignment::Center)
            .add(
                Text::new("ShellUI")
                    .size(48.0)
            )
            .add(
                Text::new("SwiftUI-inspired API")
                    .size(24.0)
            )
            .add(
                HStack::new()
                    .spacing(12.0)
                    .padding(16.0)
                    .background(Color::new(0.8, 0.8, 1.0, 1.0))
                    .add(Text::new("HStack"))
                    .add(Text::new("•"))
                    .add(Text::new("with"))
                    .add(Text::new("•"))
                    .add(Text::new("spacing"))
            )
            .add(
                VStack::new()
                    .spacing(8.0)
                    .padding(16.0)
                    .background(Color::new(1.0, 0.9, 0.8, 1.0))
                    .alignment(Alignment::Center)
                    .add(Text::new("Nested VStack").size(20.0))
                    .add(Text::new("Method chaining like SwiftUI"))
                    .add(Text::new("With padding & background colors"))
            )
            .into()
    };
    shellui::run(view);
}
