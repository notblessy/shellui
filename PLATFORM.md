# Platform Implementation

## GLFW Backend

The framework now uses GLFW for cross-platform windowing support. GLFW works on:
- Linux (X11, Wayland)
- macOS (Cocoa)
- Windows (Win32)

## Setup

To use the GLFW backend, you need to download the dependencies:

```bash
go mod tidy
```

This will download the GLFW library and its dependencies.

## Architecture

The platform layer is structured as follows:

```
platform/
├── platform.go      # Platform interface and initialization
└── glfw/
    └── platform.go # GLFW implementation
```

### Platform Interface

The `platform.Platform` interface provides:
- `NewScene(userScene scene.Scene) Scene` - Creates a platform scene from a user scene

The `platform.Scene` interface provides:
- `Run()` - Starts the event loop

### GLFW Implementation

The GLFW implementation:
1. Initializes GLFW
2. Creates a window based on scene configuration (WindowGroup)
3. Sets up OpenGL context
4. Runs the event loop
5. Handles window close events

## Current Status

✅ Window creation
✅ Event loop
✅ Cross-platform support (Linux, macOS, Windows)
⏳ Rendering (OpenGL context is created, but rendering is not yet implemented)
⏳ Input handling (events are polled, but not processed)

## Next Steps

1. Implement OpenGL rendering
2. Process input events (mouse, keyboard)
3. Render the view tree
4. Handle window resize events

## Testing

Once dependencies are downloaded, you can test the examples:

```bash
cd examples/basic
go run main.go
```

This should open a window (though it will be blank until rendering is implemented).
