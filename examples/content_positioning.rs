use shellui::{App, Color, Text, VStack, View, window_group};

struct ContentPositioningApp;

impl App for ContentPositioningApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Content Positioning Demo")
            .size(800.0, 600.0)
            .resizable(true)
            .auto_size() // Content is smaller than window
            .leading()   // Position content at top-left
    }
}

fn content_view() -> View {
    View::VStack(
        VStack::new([
            View::Text(
                Text::new("📍 Leading Position")
                    .size(32.0)
                    .color(Color::new(0.2, 0.2, 0.8, 1.0))
            ),
            View::Text(
                Text::new("This content is positioned at the top-left corner")
                    .size(18.0)
                    .color(Color::new(0.4, 0.4, 0.4, 1.0))
            ),
            View::Text(
                Text::new("instead of being centered in the window.")
                    .size(16.0)
                    .color(Color::new(0.5, 0.5, 0.5, 1.0))
            ),
            View::Text(
                Text::new("Try resizing the window to see the positioning behavior!")
                    .size(14.0)
                    .color(Color::new(0.2, 0.6, 0.2, 1.0))
            ),
        ])
        .spacing(12.0)
        .padding(32.0)
        .background(Color::new(0.95, 0.95, 1.0, 1.0))
    )
}

fn main() {
    ContentPositioningApp.run();
}