package platform

import (
	"runtime"

	"github.com/notblessy/shellui/core/scene"
	glfwPlatform "github.com/notblessy/shellui/platform/glfw"
)

// Platform represents the platform abstraction layer.
type Platform interface {
	// NewScene creates a platform scene from a user scene.
	NewScene(userScene scene.Scene) Scene
}

// Scene represents a platform scene that can be run.
type Scene interface {
	// Run starts the scene's event loop.
	Run()
}

var globalPlatform Platform

// Initialize initializes the platform layer.
// This will detect the current platform and initialize the appropriate backend.
func Initialize() Platform {
	if globalPlatform == nil {
		globalPlatform = newPlatform()
	}
	return globalPlatform
}

// SetPlatform sets a custom platform implementation.
// This is useful for testing or custom backends.
func SetPlatform(p Platform) {
	globalPlatform = p
}

// glfwPlatformAdapter adapts glfw.PlatformType to platform.Platform interface.
type glfwPlatformAdapter struct {
	*glfwPlatform.PlatformType
}

func (a *glfwPlatformAdapter) NewScene(userScene scene.Scene) Scene {
	return &glfwSceneAdapter{
		SceneType: a.PlatformType.NewScene(userScene),
	}
}

// glfwSceneAdapter adapts glfw.SceneType to platform.Scene interface.
type glfwSceneAdapter struct {
	*glfwPlatform.SceneType
}

// newPlatform creates a platform implementation based on the current OS.
func newPlatform() Platform {
	// For now, use GLFW on all platforms (cross-platform)
	// In the future, we can add platform-specific implementations
	return &glfwPlatformAdapter{
		PlatformType: glfwPlatform.NewPlatform(),
	}
}

// defaultPlatformType is a fallback implementation.
// This should not be used in normal operation.
type defaultPlatformType struct{}

func newDefaultPlatform() Platform {
	return &defaultPlatformType{}
}

func (p *defaultPlatformType) NewScene(userScene scene.Scene) Scene {
	return &defaultSceneType{
		userScene: userScene,
	}
}

// defaultSceneType is a placeholder scene implementation.
type defaultSceneType struct {
	userScene scene.Scene
}

func (ds *defaultSceneType) Run() {
	// This should not be called in normal operation
	// GLFW platform should be used instead
	runtime.LockOSThread()
	// TODO: Implement fallback if GLFW is not available
}
