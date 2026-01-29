use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct PositioningApp;

impl App for PositioningApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Positioning Example")
            .auto_size() // Content is auto-sized (not full window)
            .leading()   // Position content at top-left instead of centered
    }
}

fn content_view() -> View {
    View::VStack(
        VStack::new()
            .add(View::Text(
                Text::new("Hello World!")
                    .color(Color::new(1.0, 1.0, 1.0, 1.0))
            ))
            .add(View::HStack(
                HStack::new()
                    .add(View::Text(Text::new("Left").color(Color::new(0.8, 0.8, 1.0, 1.0))))
                    .add(View::Text(Text::new("Right").color(Color::new(1.0, 0.8, 0.8, 1.0))))
                    .spacing(20.0)
                    .leading() // Justify left
            ))
            .add(View::Text(
                Text::new("This content should be positioned at the top-left of the window")
                    .color(Color::new(0.9, 0.9, 0.9, 1.0))
            ))
            .spacing(10.0)
            .padding(20.0)
            .background(Color::new(0.2, 0.2, 0.3, 1.0))
    )
}

fn main() {
    PositioningApp.run();
}