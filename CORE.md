# ShellUI Core Implementation

## Overview

This document describes the core architecture of ShellUI framework. The core provides the fundamental building blocks for building declarative GUI applications in Go, following SwiftUI's paradigm.

## Core Components

### 1. App Protocol (`core/app/`)

The `App` protocol is the root of every ShellUI application. Users implement this to define their application.

```go
type App interface {
    Body() scene.Scene
}
```

**Entry Point:**
```go
app.Run(&MyApp{})
```

### 2. Scene Protocol (`core/scene/`)

A `Scene` represents a container that can hold views. Currently implemented:

- **WindowGroup**: For GUI applications (single window)
- **DesktopScene**: For desktop environments (future)

```go
type Scene interface {
    Body() view.View
}
```

**Usage:**
```go
scene.WindowGroup(contentView).Title("My App").Size(800, 600)
```

### 3. View Protocol (`core/view/`)

Everything visible in ShellUI is a `View`. Views compose other views to build the UI hierarchy.

```go
type View interface {
    Body() View
}
```

**Container Views:**
- `VStack` - Vertical stack
- `HStack` - Horizontal stack  
- `ZStack` - Z-ordered stack (layers)
- `Spacer` - Flexible spacer

**Usage:**
```go
view.VStack(
    view.Text("Hello"),
    view.HStack(
        view.Button("OK", func() {}),
        view.Spacer(),
    ),
)
```

### 4. State Management (`core/state/`)

Reactive state management similar to SwiftUI's `@State` and `@Binding`.

**State:**
```go
count := state.New(0)
count.Set(5)
value := count.Value()
count.OnChange(func(newVal int) {
    // React to changes
})
```

**Binding:**
```go
binding := state.BindingFromState(count)
binding.Set(10)
value := binding.Get()
```

### 5. Reconciliation Engine (`core/reconcile/`)

The reconciliation engine handles diffing between old and new view trees, generating minimal update operations. This is the core of the declarative UI system.

**Status:** Framework in place, implementation pending.

### 6. Layout Engine (`core/layout/`)

Calculates layout for views using constraints. Will support flexbox, grid, and other layout algorithms.

**Status:** Framework in place, implementation pending.

## Platform Abstraction (`platform/`)

The platform layer abstracts platform-specific details (X11, Wayland, etc.). Currently provides a placeholder implementation.

**Status:** Framework in place, X11 implementation pending.

## Widget Library (`widget/`)

Basic widgets are provided as separate packages:

- `widget/text` - Text display
- `widget/button` - Button widget

More widgets will be added as the framework evolves.

## Architecture Flow

```
User App (implements App)
    ↓
Scene (WindowGroup, DesktopScene)
    ↓
View Tree (composed Views)
    ↓
Reconciliation Engine (diffs view trees)
    ↓
Layout Engine (calculates positions)
    ↓
Rendering Engine (draws to screen)
    ↓
Platform Layer (X11, etc.)
```

## Design Principles

1. **Declarative**: Describe what you want, not how to do it
2. **Composable**: Views compose other views
3. **Reactive**: UI updates automatically when state changes
4. **Modular**: Each component is a separate package
5. **Extensible**: Users can create custom views and widgets

## Next Steps

To complete the core framework:

1. **Rendering Engine**: Implement OpenGL renderer
2. **Platform Implementation**: X11 backend
3. **Reconciliation**: Implement diffing algorithm
4. **Layout**: Implement flexbox/grid layout
5. **Style System**: LVGL-inspired styling
6. **More Widgets**: Expand widget library

## Example Usage

```go
package main

import (
    "github.com/notblessy/shellui/core/app"
    "github.com/notblessy/shellui/core/scene"
    "github.com/notblessy/shellui/core/view"
    "github.com/notblessy/shellui/widget/text"
)

type MyApp struct{}

func (a *MyApp) Body() scene.Scene {
    return scene.WindowGroup(
        view.VStack(
            text.Text("Hello, ShellUI!"),
        ),
    ).Title("My App")
}

func main() {
    app.Run(&MyApp{})
}
```
