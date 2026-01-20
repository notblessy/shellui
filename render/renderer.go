package render

import (
	"github.com/go-gl/gl/v4.1-core/gl"
	"github.com/notblessy/shellui/core/view"
)

// RendererType handles rendering of views.
// This is a wrapper around the Painter interface for backward compatibility.
type RendererType struct {
	painter Painter
	width   int
	height  int
}

// NewRenderer creates a new renderer.
func NewRenderer(width, height int) *RendererType {
	return &RendererType{
		painter: NewGLPainter(width, height),
		width:   width,
		height:  height,
	}
}

// Render renders a view tree.
func (r *RendererType) Render(rootView view.View) {
	// Enable blending
	gl.Enable(gl.BLEND)
	gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)

	// Clear the screen
	r.painter.Clear()

	// Render the root view
	if rootView != nil {
		// Get view size (for now, use full window)
		r.painter.Paint(rootView, 0, 0, float32(r.width), float32(r.height))
	}
}

// SetSize updates the renderer size.
func (r *RendererType) SetSize(width, height int) {
	r.width = width
	r.height = height
	// Update painter size and DPI scale
	if glPainter, ok := r.painter.(*GLPainterType); ok {
		glPainter.SetSize(width, height)
	}
}

// GetPainter returns the underlying painter (for platform-specific operations)
func (r *RendererType) GetPainter() Painter {
	return r.painter
}
