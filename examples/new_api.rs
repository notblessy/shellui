use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct SimpleApp;

impl App for SimpleApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("New API Demo")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    // Example of the new API structure:
    // VStack::new([children]).styling()
    VStack::new::<Vec<View>>(vec![
        Text::new("hello").into(),
        HStack::new::<Vec<View>>(vec![
            Text::new("Left").into(),
            Text::new("Right").into(),
        ])
        .spacing(10.0)
        .into(),
    ])
    .padding(20.0)
    .background(Color::new(0.9, 0.9, 0.9, 1.0))
    .leading()
    .into()
}

fn main() {
    SimpleApp.run();
}