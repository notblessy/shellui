package render

import (
	"github.com/go-gl/gl/v4.1-core/gl"
	"github.com/notblessy/shellui/core/canvas"
	"github.com/notblessy/shellui/core/view"
)

// RendererType handles rendering of views.
// It owns a Canvas and coordinates between the canvas and painter.
type RendererType struct {
	canvas  *canvas.Canvas
	painter Painter
}

// NewRenderer creates a new renderer with the given canvas.
func NewRenderer(cnv *canvas.Canvas) *RendererType {
	return &RendererType{
		canvas:  cnv,
		painter: NewGLPainter(cnv),
	}
}

// Render renders the canvas content.
// Uses canvas size (logical coordinates) for stable rendering.
func (r *RendererType) Render() {
	// Enable blending
	gl.Enable(gl.BLEND)
	gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)

	// Clear the screen
	r.painter.Clear()

	// Render the canvas content
	content := r.canvas.Content()
	if content != nil {
		// Get canvas size and content position (accounting for padding)
		canvasSize := r.canvas.Size()
		contentPos := r.canvas.ContentPos()
		
		// Paint the content at the content position with canvas size
		r.painter.Paint(content, contentPos.X, contentPos.Y, canvasSize.Width, canvasSize.Height)
	}
}

// ResizeCanvas updates the canvas size.
// This is called when the window is resized.
func (r *RendererType) ResizeCanvas(size view.Size) {
	r.canvas.Resize(size)
}

// GetCanvas returns the canvas.
func (r *RendererType) GetCanvas() *canvas.Canvas {
	return r.canvas
}

// GetPainter returns the underlying painter (for platform-specific operations)
func (r *RendererType) GetPainter() Painter {
	return r.painter
}
