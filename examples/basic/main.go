package main

import (
	"github.com/notblessy/shellui/core/app"
	"github.com/notblessy/shellui/core/scene"
	"github.com/notblessy/shellui/core/view"
	"github.com/notblessy/shellui/widget/text"
)

// BasicApp demonstrates a simple shellui application with font styling.
type BasicApp struct{}

// Body returns the root scene of the application.
func (a *BasicApp) Body() scene.Scene {
	return scene.WindowGroup(
		view.VStack(
			// Large title text
			text.Text("Hello, ShellUI!").Size(48).Bold(),

			// Different font sizes
			text.Text("Font Sizes:").Size(20).Bold(),
			text.Text("Size 12").Size(12),
			text.Text("Size 16 (default)"),
			text.Text("Size 24").Size(24),
			text.Text("Size 32").Size(32),

			// Different font weights
			text.Text("Font Weights:").Size(20).Bold(),
			text.Text("Regular weight"),
			text.Text("Bold weight").Bold(),
			text.Text("Light weight").Weight(text.FontWeightLight),
			text.Text("Medium weight").Weight(text.FontWeightMedium),
			text.Text("Semibold weight").Weight(text.FontWeightSemibold),

			// Text alignment examples
			text.Text("Text Alignment:").Size(20).Bold(),
			text.Text("Left aligned (leading)").Alignment(text.TextAlignLeading),
			text.Text("Center aligned").Alignment(text.TextAlignCenter),
			text.Text("Right aligned (trailing)").Alignment(text.TextAlignTrailing),

			// Combined styling
			text.Text("Combined Styling:").Size(20).Bold(),
			text.Text("Large Bold Text").Size(28).Bold(),
			text.Text("Small Medium Text").Size(14).Weight(text.FontWeightMedium),
		),
	).Title("Basic Example - Font Styling").Size(600, 800)
}

func main() {
	app.Run(&BasicApp{})
}
