package scene

import "github.com/notblessy/shellui/core/view"

// Convenience functions for creating scenes in a SwiftUI-like style.

// WindowGroup creates a new WindowGroupType scene.
func WindowGroup(content view.View) *WindowGroupType {
	return NewWindowGroup(content)
}

// DesktopScene creates a new DesktopSceneType.
func DesktopScene(content view.View) *DesktopSceneType {
	return NewDesktopScene(content)
}
