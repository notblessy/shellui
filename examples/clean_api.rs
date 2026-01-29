use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct CleanApp;

impl App for CleanApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Clean API Demo") 
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    // Clean API - children in constructor, styling via builder pattern
    VStack::new([
        Text::new("hello").color(Color::new(0.2, 0.2, 0.8, 1.0)),
        HStack::new([
            Text::new("Left").color(Color::new(1.0, 0.2, 0.2, 1.0)),
            Text::new("Right").color(Color::new(0.2, 1.0, 0.2, 1.0)),
        ])
        .spacing(20.0)
        .leading(), // Justify left
        Text::new("Bottom text").color(Color::new(0.5, 0.5, 0.5, 1.0)),
    ])
    .spacing(15.0)
    .padding(30.0)
    .background(Color::new(0.95, 0.95, 0.95, 1.0))
    .leading() // Justify to top
    .into()
}

fn main() {
    CleanApp.run();
}