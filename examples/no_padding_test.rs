use shellui::{App, Color, Text, VStack, View, window_group};

struct NoPaddingTestApp;

impl App for NoPaddingTestApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("No Padding Test")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    // No padding at all - should be tight against edges
    VStack::new::<Vec<View>>(vec![
        Text::new("Should be tight").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
        Text::new("No padding here").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
    ])
    .background(Color::new(1.0, 0.0, 0.0, 1.0)) // Red background to see exact bounds
    .into()
}

fn main() {
    NoPaddingTestApp.run();
}