package render

import (
	"github.com/notblessy/shellui/core/view"
)

// Painter defines the functionality of our renderer.
type Painter interface {
	// Clear tells the painter to prepare a fresh paint
	Clear()
	// Paint a single view but not its children
	Paint(v view.View, x, y float32, width, height float32)
	// SetOutputSize is used to change the resolution of our output viewport
	SetOutputSize(width, height int)
	// StartClipping tells us that the following paint actions should be clipped to the specified area
	StartClipping(x, y, width, height float32)
	// StopClipping stops clipping paint actions
	StopClipping()
}

// PainterType is the base implementation of the Painter interface.
// This is a base struct that can be embedded by specific implementations.
type PainterType struct {
	width  int
	height int
}

// SetOutputSize updates the painter's output size.
func (p *PainterType) SetOutputSize(width, height int) {
	p.width = width
	p.height = height
}

// GetSize returns the current size.
func (p *PainterType) GetSize() (width, height int) {
	return p.width, p.height
}

// Clear is a placeholder - should be implemented by specific painters.
func (p *PainterType) Clear() {
	// Base implementation does nothing
}

// Paint is a placeholder - should be implemented by specific painters.
func (p *PainterType) Paint(v view.View, x, y, width, height float32) {
	// Base implementation does nothing
}

// StartClipping is a placeholder - should be implemented by specific painters.
func (p *PainterType) StartClipping(x, y, width, height float32) {
	// Base implementation does nothing
}

// StopClipping is a placeholder - should be implemented by specific painters.
func (p *PainterType) StopClipping() {
	// Base implementation does nothing
}
