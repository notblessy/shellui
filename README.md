# ShellUI

A declarative GUI framework for Go, inspired by SwiftUI's paradigm and LVGL's aesthetics.

## Overview

ShellUI is a framework/engine that provides:
- **SwiftUI-like declarative API** - Write UI code in a declarative, composable style
- **LVGL-inspired aesthetics** - Beautiful, modern UI components
- **Reactive state management** - Automatic UI updates when state changes
- **Cross-platform** - Runs on Linux (X11), with more platforms coming

## Core Concepts

### App Protocol

Your application implements the `App` protocol:

```go
type MyApp struct{}

func (a *MyApp) Body() scene.Scene {
    return scene.WindowGroup(
        ContentView(),
    ).Title("My App")
}
```

### Scene

A `Scene` represents a container. Currently supported:
- `WindowGroup` - For GUI applications
- `DesktopScene` - For desktop environments (future)

### View

Everything is a `View`. Views compose other views:

```go
func ContentView() view.View {
    return view.VStack(
        view.Text("Hello"),
        view.Button("Click", func() { /* ... */ }),
    )
}
```

### State Management

Reactive state management similar to SwiftUI's `@State`:

```go
type CounterView struct {
    Count state.State[int]
}

func (cv *CounterView) Body() view.View {
    return view.VStack(
        view.Text(fmt.Sprintf("Count: %d", cv.Count.Value())),
        view.Button("Increment", func() {
            cv.Count.Set(cv.Count.Value() + 1)
        }),
    )
}
```

## Project Structure

```
shellui/
├── core/              # Core framework APIs
│   ├── app/           # App protocol, app.Run()
│   ├── scene/         # Scene protocol
│   ├── view/          # View protocol, containers
│   ├── state/         # State management
│   ├── reconcile/     # Diffing engine
│   └── layout/        # Layout algorithms
│
├── platform/          # Platform abstraction
│
├── render/            # Rendering engine (TODO)
│
├── style/             # Style system (TODO)
│
└── widget/            # Widget library (TODO)
```

## Getting Started

### Basic Example

```go
package main

import (
    "github.com/notblessy/shellui/app"
    "github.com/notblessy/shellui/scene"
    "github.com/notblessy/shellui/view"
)

type MyApp struct{}

func (a *MyApp) Body() scene.Scene {
    return scene.WindowGroup(
        view.VStack(
            view.Text("Hello, ShellUI!"),
        ),
    ).Title("My App")
}

func main() {
    app.Run(&MyApp{})
}
```

## Status

🚧 **Work in Progress**

Currently implemented:
- ✅ Core App, Scene, View protocols
- ✅ State management system
- ✅ Basic container views (VStack, HStack, ZStack)
- ✅ Platform abstraction layer

Coming soon:
- ⏳ Rendering engine
- ⏳ Widget library
- ⏳ Style system
- ⏳ Layout engine
- ⏳ Reconciliation engine

## License

[To be determined]
