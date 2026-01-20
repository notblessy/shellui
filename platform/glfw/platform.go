package glfw

import (
	"runtime"

	"github.com/go-gl/gl/v4.1-core/gl"
	"github.com/go-gl/glfw/v3.3/glfw"
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/render"
)

// PlatformType is the GLFW-based platform implementation.
type PlatformType struct{}

// NewPlatform creates a new GLFW platform.
func NewPlatform() *PlatformType {
	return &PlatformType{}
}

// NewScene creates a platform scene from a user scene.
func (p *PlatformType) NewScene(userScene scene.Scene) *SceneType {
	return &SceneType{
		userScene: userScene,
	}
}

// SceneType represents a GLFW-based scene.
type SceneType struct {
	userScene scene.Scene
	window    *glfw.Window
}

// Run starts the GLFW event loop.
func (s *SceneType) Run() {
	// GLFW requires main thread
	runtime.LockOSThread()
	defer runtime.UnlockOSThread()

	// Initialize GLFW
	if err := glfw.Init(); err != nil {
		panic("failed to initialize GLFW: " + err.Error())
	}
	defer glfw.Terminate()

	// Get window configuration from scene
	var width, height int = 800, 600
	var title string = "ShellUI App"

	if wg, ok := s.userScene.(*scene.WindowGroupType); ok {
		width, height = wg.GetSize()
		title = wg.GetTitle()
		if title == "" {
			title = "ShellUI App"
		}
	}

	// Configure GLFW
	glfw.WindowHint(glfw.ContextVersionMajor, 3)
	glfw.WindowHint(glfw.ContextVersionMinor, 3)
	glfw.WindowHint(glfw.OpenGLProfile, glfw.OpenGLCoreProfile)
	glfw.WindowHint(glfw.OpenGLForwardCompatible, glfw.True)

	// Create window
	window, err := glfw.CreateWindow(width, height, title, nil, nil)
	if err != nil {
		panic("failed to create GLFW window: " + err.Error())
	}
	s.window = window
	defer window.Destroy()

	// Make context current
	window.MakeContextCurrent()

	// Initialize OpenGL
	if err := gl.Init(); err != nil {
		panic("failed to initialize OpenGL: " + err.Error())
	}

	// Set viewport
	fbWidth, fbHeight := window.GetFramebufferSize()
	winWidth, winHeight := window.GetSize()
	gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))

	// Create renderer with logical window size (not framebuffer size)
	// This ensures text maintains constant size regardless of DPI
	renderer := render.NewRenderer(winWidth, winHeight)

	// Set up window size callback (for logical coordinate updates during resize)
	// This is called during window resize drag, updating the coordinate system in real-time
	window.SetSizeCallback(func(w *glfw.Window, winWidth, winHeight int) {
		// Update renderer with new logical window size immediately
		// This ensures text maintains constant size during resize drag
		renderer.SetSize(winWidth, winHeight)
		// Also update framebuffer size to recalculate DPI scale
		fbWidth, fbHeight := w.GetFramebufferSize()
		if glPainter, ok := renderer.GetPainter().(*render.GLPainterType); ok {
			glPainter.SetFramebufferSize(fbWidth, fbHeight)
		}
	})

	// Set up framebuffer size callback (for viewport updates)
	// This is called when the framebuffer size changes (e.g., on retina displays)
	window.SetFramebufferSizeCallback(func(w *glfw.Window, fbWidth, fbHeight int) {
		// Update viewport to match framebuffer size (physical pixels)
		gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))
		// Update framebuffer size in painter to recalculate DPI scale
		if glPainter, ok := renderer.GetPainter().(*render.GLPainterType); ok {
			glPainter.SetFramebufferSize(fbWidth, fbHeight)
		}
	})

	// Set up callbacks
	window.SetCloseCallback(func(w *glfw.Window) {
		// Window close requested
	})

	// Main loop
	for !window.ShouldClose() {
		// Poll events
		glfw.PollEvents()

		// Render the view tree
		// Get the view from the scene (this is the root view, e.g., VStack)
		rootView := s.userScene.Body()
		if rootView != nil {
			// Render the view tree using the new renderer
			// The renderer will clear the screen
			renderer.Render(rootView)
		}

		// Swap buffers
		window.SwapBuffers()
	}
}

// GetWindow returns the GLFW window (for future use).
func (s *SceneType) GetWindow() *glfw.Window {
	return s.window
}
