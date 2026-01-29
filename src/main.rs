//! Example: window with VStack of Text views.

use shellui::{Alignment, Text, VStack, View};

fn main() {
    let view = || {
        View::from(
            VStack::new()
                .spacing(8.0)
                .alignment(Alignment::Center)
                .push(Text::new("Hello").size(64.0).into())
                .push(Text::new("ShellUI").size(64.0).into()),
        )
    };
    shellui::run(view);
}
