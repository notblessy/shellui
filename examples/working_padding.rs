use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct WorkingPaddingApp;

impl App for WorkingPaddingApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("✅ Padding & Spacing Working!")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    VStack::new::<Vec<View>>(vec![
        Text::new("🎉 Padding & Spacing Now Work!")
            .size(24.0)
            .color(Color::new(0.2, 0.8, 0.2, 1.0))
            .into(),
        
        // Demonstrating VStack with padding and spacing
        VStack::new::<Vec<View>>(vec![
            Text::new("Padded VStack").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("Item 2").color(Color::new(0.9, 0.9, 0.9, 1.0)).into(),
            Text::new("Item 3").color(Color::new(0.8, 0.8, 0.8, 1.0)).into(),
        ])
        .spacing(12.0)  // 12px between items
        .padding(20.0)  // 20px around everything
        .background(Color::new(0.8, 0.3, 0.3, 1.0))
        .into(),
        
        // Demonstrating HStack with padding and spacing
        HStack::new::<Vec<View>>(vec![
            Text::new("A").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("B").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
            Text::new("C").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
        ])
        .spacing(25.0)  // 25px between items
        .padding(15.0)  // 15px around everything
        .background(Color::new(0.3, 0.8, 0.3, 1.0))
        .into(),
        
        Text::new("Both spacing and padding are now functional! 🚀")
            .color(Color::new(0.2, 0.2, 0.8, 1.0))
            .into(),
    ])
    .spacing(15.0)  // 15px between main sections
    .padding(30.0)  // 30px around the whole thing
    .background(Color::new(0.95, 0.95, 1.0, 1.0))
    .into()
}

fn main() {
    WorkingPaddingApp.run();
}