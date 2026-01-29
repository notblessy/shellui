use shellui::{App, Color, Text, VStack, View, window_group};

struct CenterPositioningApp;

impl App for CenterPositioningApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Center Position Demo")
            .size(800.0, 600.0)
            .resizable(true)
            .auto_size() // Content is smaller than window
            .center()    // Position content at center (default)
    }
}

fn content_view() -> View {
    View::VStack(
        VStack::new([
            View::Text(
                Text::new("🎯 Center Position")
                    .size(32.0)
                    .color(Color::new(0.8, 0.2, 0.2, 1.0))
            ),
            View::Text(
                Text::new("This content is centered in the window")
                    .size(18.0)
                    .color(Color::new(0.4, 0.4, 0.4, 1.0))
            ),
            View::Text(
                Text::new("Compare with the leading position example!")
                    .size(16.0)
                    .color(Color::new(0.5, 0.5, 0.5, 1.0))
            ),
        ])
        .spacing(12.0)
        .padding(32.0)
        .background(Color::new(1.0, 0.95, 0.95, 1.0))
    )
}

fn main() {
    CenterPositioningApp.run();
}