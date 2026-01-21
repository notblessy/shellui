package glfw

import (
	"runtime"

	"github.com/go-gl/gl/v4.1-core/gl"
	"github.com/go-gl/glfw/v3.3/glfw"
	"github.com/notblessy/shellui/core/canvas"
	"github.com/notblessy/shellui/core/scale"
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/core/view"
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

	// Create canvas with initial size in logical coordinates
	cnv := canvas.NewCanvas()
	initialCanvasSize := computeCanvasSize(cnv, winWidth, winHeight)
	cnv.Resize(initialCanvasSize)

	// Set the root view as canvas content
	rootView := s.userScene.Body()
	cnv.SetContent(rootView)

	// Create renderer with canvas
	renderer := render.NewRenderer(cnv)

	// Calculate and set texture scale (HiDPI support)
	texScale := detectTextureScale(winWidth, fbWidth)
	cnv.SetTexScale(texScale)

	// Flag to prevent double rendering (resize callback renders, main loop skips)
	resizeRendered := false

	// Set up window size callback
	// Converts window size to canvas size and resizes canvas
	window.SetSizeCallback(func(w *glfw.Window, winWidth, winHeight int) {
		// Update viewport
		fbWidth, fbHeight := w.GetFramebufferSize()
		gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))

		// Update canvas
		canvasSize := computeCanvasSize(cnv, winWidth, winHeight)
		renderer.ResizeCanvas(canvasSize)

		// Render immediately to prevent OS from stretching old frame
		renderer.Render()
		w.SwapBuffers()

		// Mark that we already rendered this frame
		resizeRendered = true
	})

	// Set up framebuffer size callback (for viewport and texture scale updates)
	window.SetFramebufferSizeCallback(func(w *glfw.Window, fbWidth, fbHeight int) {
		// Update OpenGL viewport to match framebuffer size (physical pixels)
		gl.Viewport(0, 0, int32(fbWidth), int32(fbHeight))

		// Update texture scale for HiDPI rendering
		winWidth, _ := w.GetSize()
		texScale := detectTextureScale(winWidth, fbWidth)
		cnv.SetTexScale(texScale)
	})

	// Set up callbacks
	window.SetCloseCallback(func(w *glfw.Window) {
		// Window close requested
	})

	// Main loop
	for !window.ShouldClose() {
		// Poll events (this may trigger resize callback which renders)
		glfw.PollEvents()

		// Skip render if resize callback already rendered this frame
		if resizeRendered {
			resizeRendered = false
			continue
		}

		// Render the canvas
		renderer.Render()

		// Swap buffers
		window.SwapBuffers()
	}
}

// computeCanvasSize converts window size (screen pixels) to canvas size (logical coordinates).
func computeCanvasSize(c *canvas.Canvas, width, height int) view.Size {
	return view.Size{
		Width:  scale.ToCanvasCoordinate(c, width),
		Height: scale.ToCanvasCoordinate(c, height),
	}
}

// detectTextureScale calculates the texture scale from window and framebuffer sizes.
// This is the HiDPI scale factor (e.g., 2.0 on Retina displays).
func detectTextureScale(winWidth, fbWidth int) float32 {
	if winWidth == 0 {
		return 1.0
	}
	return float32(fbWidth) / float32(winWidth)
}

// GetWindow returns the GLFW window (for future use).
func (s *SceneType) GetWindow() *glfw.Window {
	return s.window
}
