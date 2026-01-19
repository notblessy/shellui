package app

import (
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/platform"
)

// App is the protocol that user applications must implement.
// It represents the root of the application.
type App interface {
	// Body returns the root scene of the application.
	Body() scene.Scene
}

// Run starts the framework and runs the user's application.
// This is the main entry point for shellui applications.
func Run(app App) {
	// Initialize platform
	platformInstance := platform.Initialize()
	
	// Get user's scene
	userScene := app.Body()
	
	// Create platform scene
	platformScene := platformInstance.NewScene(userScene)
	
	// Run the platform scene (handles rendering, input, event loop)
	platformScene.Run()
}
