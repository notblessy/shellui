use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct TestPaddingApp;

impl App for TestPaddingApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("Padding & Spacing Test")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    VStack::new::<Vec<View>>(vec![
        Text::new("No Padding").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
        
        VStack::new::<Vec<View>>(vec![
            Text::new("With 20px Padding").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("Inside padded area").color(Color::new(0.8, 0.8, 0.8, 1.0)).into(),
        ])
        .padding(20.0)
        .background(Color::new(0.8, 0.3, 0.3, 1.0))
        .into(),
        
        HStack::new::<Vec<View>>(vec![
            Text::new("Item1").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("Item2").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("Item3").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
        ])
        .spacing(30.0)  // Should have 30px between items
        .padding(15.0)  // Should have 15px around the whole thing
        .background(Color::new(0.3, 0.8, 0.3, 1.0))
        .into(),
    ])
    .spacing(10.0)  // 10px between main items
    .padding(25.0)  // 25px around everything
    .background(Color::new(0.9, 0.9, 1.0, 1.0))
    .into()
}

fn main() {
    TestPaddingApp.run();
}