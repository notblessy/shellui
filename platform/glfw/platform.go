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
	gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))

	// Create renderer with framebuffer size (important for retina displays)
	renderer := render.NewRenderer(fbWidth, fbHeight)

	// Set up window resize callback
	window.SetFramebufferSizeCallback(func(w *glfw.Window, fbWidth, fbHeight int) {
		gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))
		renderer.SetSize(fbWidth, fbHeight)
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
