package main

import (
	"github.com/notblessy/shellui/core/app"
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/core/view"
	"github.com/notblessy/shellui/widget/text"
)

// BasicApp demonstrates a simple shellui application.
type BasicApp struct{}

// Body returns the root scene of the application.
func (a *BasicApp) Body() scene.Scene {
	return scene.WindowGroup(
		view.VStack(
			text.Text("Hello, ShellUI!"),
		),
	).Title("Basic Example").Size(400, 300)
}

func main() {
	app.Run(&BasicApp{})
}
