use shellui::*;

// Create an app struct
struct CounterApp;

impl App for CounterApp {
    fn body(&self) -> impl IntoScene {
        window_group(|| {
            VStack::new::<Vec<View>>(vec![
                Text::new("Count: 0")
                    .size(24.0)
                    .into(),
                Button::new("Increment")
                    .on_click(increment)
                    .padding(16.0)
                    .into(),
            ])
            .spacing(20.0)
            .padding(20.0)
            .into()
        })
        .title("Counter App")
        .size(300.0, 200.0)
    }
}

// Static function for button click
fn increment() {
    println!("Button clicked!");
}

fn main() {
    let app = CounterApp;
    app.run();
}