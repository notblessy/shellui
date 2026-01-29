use shellui::{App, Color, HStack, Text, VStack, View, window_group};

struct NewAPIApp;

impl App for NewAPIApp {
    fn body(&self) -> impl shellui::IntoScene {
        window_group(|| content_view())
            .title("New SwiftUI-Style API")
            .auto_size()
            .leading()
    }
}

fn content_view() -> View {
    // NEW API: View tree structure is defined in constructors
    // Builder pattern is ONLY for styling (spacing, padding, colors, etc.)
    VStack::new::<Vec<View>>(vec![
        Text::new("🎉 New API Structure")
            .size(32.0)
            .color(Color::new(0.2, 0.2, 0.8, 1.0))
            .into(),
            
        Text::new("View tree in constructors, styling via methods")
            .color(Color::new(0.4, 0.4, 0.4, 1.0))
            .into(),
            
        HStack::new::<Vec<View>>(vec![
            VStack::new::<Vec<View>>(vec![
                Text::new("Left Column").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
                Text::new("Content A").color(Color::new(0.9, 0.9, 0.9, 1.0)).into(),
            ])
            .padding(15.0)
            .background(Color::new(0.8, 0.3, 0.3, 1.0))
            .into(),
            
            VStack::new::<Vec<View>>(vec![
                Text::new("Right Column").color(Color::new(1.0, 1.0, 1.0, 1.0)).into(),
                Text::new("Content B").color(Color::new(0.9, 0.9, 0.9, 1.0)).into(),
            ])
            .padding(15.0)
            .background(Color::new(0.3, 0.8, 0.3, 1.0))
            .into(),
        ])
        .spacing(20.0)
        .space_between() // Justify between columns
        .into(),
        
        Text::new("Clean and intuitive! 🚀")
            .color(Color::new(0.2, 0.6, 0.2, 1.0))
            .into(),
    ])
    .spacing(20.0)
    .padding(30.0)
    .background(Color::new(0.95, 0.95, 1.0, 1.0))
    .leading() // Justify to top
    .into()
}

fn main() {
    NewAPIApp.run();
}