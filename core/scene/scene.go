package scene

import "github.com/notblessy/shellui/core/view"

// Scene represents a container that can hold views.
// In SwiftUI terms, this is like WindowGroup, DocumentGroup, etc.
type Scene interface {
	// Body returns the root view of this scene.
	Body() view.View
}

// WindowGroupType is a scene that represents a window for GUI applications.
type WindowGroupType struct {
	content view.View
	title   string
	width   int
	height  int
}

// NewWindowGroup creates a new WindowGroupType scene.
func NewWindowGroup(content view.View) *WindowGroupType {
	return &WindowGroupType{
		content: content,
		width:   800,
		height:  600,
	}
}

// Title sets the title of the window.
func (wg *WindowGroupType) Title(title string) *WindowGroupType {
	wg.title = title
	return wg
}

// Size sets the size of the window.
func (wg *WindowGroupType) Size(width, height int) *WindowGroupType {
	wg.width = width
	wg.height = height
	return wg
}

// Body returns the content view of this window group.
func (wg *WindowGroupType) Body() view.View {
	return wg.content
}

// GetTitle returns the window title.
func (wg *WindowGroupType) GetTitle() string {
	return wg.title
}

// GetSize returns the window size.
func (wg *WindowGroupType) GetSize() (width, height int) {
	return wg.width, wg.height
}

// DesktopSceneType is a scene for desktop environments.
// This is for future use when building desktop environments.
type DesktopSceneType struct {
	content view.View
}

// NewDesktopScene creates a new DesktopSceneType.
func NewDesktopScene(content view.View) *DesktopSceneType {
	return &DesktopSceneType{
		content: content,
	}
}

// Body returns the content view of this desktop scene.
func (ds *DesktopSceneType) Body() view.View {
	return ds.content
}
