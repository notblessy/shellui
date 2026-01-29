use shellui::{App, Color, Text, VStack, View, window_group};

struct DebugSpacingApp;

impl App for DebugSpacingApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Debug Spacing - Should be TIGHT")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    // This should be completely tight - no padding, no spacing
    VStack::new::<Vec<View>>(vec![
        Text::new("TIGHT1").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
        Text::new("TIGHT2").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
    ])
    // NO .spacing(), NO .padding() - should be completely tight
    .background(Color::new(1.0, 0.0, 0.0, 1.0)) // Red background to see exact bounds
    .into()
}

fn main() {
    DebugSpacingApp.run();
}