use shellui::*;
use std::sync::atomic::{AtomicI32, Ordering};

// Global counter state using atomic integer for thread safety
static COUNTER: AtomicI32 = AtomicI32::new(0);

// Function to increment counter
fn increment() {
    let new_value = COUNTER.fetch_add(1, Ordering::Relaxed) + 1;
    println!("Counter incremented to: {}", new_value);
}

// Function to decrement counter  
fn decrement() {
    let new_value = COUNTER.fetch_sub(1, Ordering::Relaxed) - 1;
    println!("Counter decremented to: {}", new_value);
}

// Function to reset counter
fn reset() {
    COUNTER.store(0, Ordering::Relaxed);
    println!("Counter reset to: 0");
}

// Create an app struct
struct AdvancedCounterApp;

impl App for AdvancedCounterApp {
    fn body(&self) -> impl IntoScene {
        window_group(|| {
            let current_count = COUNTER.load(Ordering::Relaxed);
            
            VStack::new::<Vec<View>>(vec![
                Text::new("Advanced Counter Demo")
                    .size(28.0)
                    .color(Color::new(0.3, 0.3, 0.7, 1.0))
                    .into(),
                    
                Text::new(format!("Count: {}", current_count))
                    .size(48.0)
                    .color(Color::new(0.1, 0.5, 0.1, 1.0))
                    .into(),
                    
                // Button row
                HStack::new::<Vec<View>>(vec![
                    Button::new("-")
                        .on_click(decrement)
                        .padding(20.0)
                        .background(Color::new(0.8, 0.2, 0.2, 1.0))
                        .text_color(Color::new(1.0, 1.0, 1.0, 1.0))
                        .into(),
                        
                    Button::new("+")
                        .on_click(increment)
                        .padding(20.0)
                        .background(Color::new(0.2, 0.7, 0.2, 1.0))
                        .text_color(Color::new(1.0, 1.0, 1.0, 1.0))
                        .into(),
                ])
                .spacing(15.0)
                .center_justify()
                .into(),
                
                Button::new("Reset")
                    .on_click(reset)
                    .padding(18.0)
                    .background(Color::new(0.5, 0.5, 0.5, 1.0))
                    .text_color(Color::new(1.0, 1.0, 1.0, 1.0))
                    .into(),
            ])
            .spacing(12.0)
            .padding(15.0)
            .alignment(Alignment::Center)
            .center_justify()
            .into()
        })
        .title("Advanced Counter")
        .size(400.0, 300.0)
        .resizable(false)
        .auto_size()
        .center()
    }
}

fn main() {
    let app = AdvancedCounterApp;
    app.run();
}