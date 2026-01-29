# ShellUI

_Pronounced "Shelly"_

A SwiftUI-inspired GUI framework prototype for Rust, focusing on declarative view composition and layout.

## Overview

ShellUI implements a declarative UI approach inspired by SwiftUI's view tree architecture. Build interfaces using composable views with stack-based layouts.

## Features

- **Declarative Views**: Compose interfaces with reusable view components
- **Stack Layouts**: VStack and HStack for vertical and horizontal arrangements
- **Text Rendering**: High-quality text with cosmic-text and SwashCache

## Quick Start

```rust
use shellui::*;

fn main() {
    window::run(|| {
        VStack::new(vec![
            Text::new("Hello, World!").into(),
            Text::new("SwiftUI-style declarative UI").into(),
        ]).into()
    });
}
```

## Architecture

The framework follows SwiftUI's view tree pattern where views are composable, stateless descriptions of UI that get rendered through a unified rendering pipeline.
