package canvas

import (
	"math"

	"github.com/notblessy/shellui/core/view"
)

// Canvas represents a drawing surface with logical coordinates and scaling.
// This is the core abstraction between the window (physical pixels) and content (logical coordinates).
type Canvas struct {
	content  view.View   // The root view being displayed
	size     view.Size   // Logical size in canvas coordinates
	scale    float32     // User/system scale factor (1.0, 1.25, 1.5, 2.0, etc.)
	texScale float32     // Texture/DPI scale (framebuffer/window ratio)
	padded   bool        // Whether content should have padding
}

// NewCanvas creates a new canvas with default scale (1.0).
func NewCanvas() *Canvas {
	return &Canvas{
		scale:    1.0,
		texScale: 1.0,
		padded:   true,
	}
}

// Content returns the root view displayed on this canvas.
func (c *Canvas) Content() view.View {
	return c.content
}

// SetContent sets the root view and calculates its minimum size.
func (c *Canvas) SetContent(content view.View) {
	c.content = content
	if content != nil {
		// Give content its natural size
		minSize := content.MinSize()
		if minSize.Width > 0 && minSize.Height > 0 {
			// Use at least the minimum size
			c.size = c.size.Max(minSize)
		}
	}
}

// Size returns the current canvas size in logical coordinates.
func (c *Canvas) Size() view.Size {
	return c.size
}

// Resize updates the canvas size and resizes the content to fit.
// The size is in logical coordinates (device-independent).
func (c *Canvas) Resize(size view.Size) {
	// Round to pixel boundaries to prevent sub-pixel jitter
	c.size = view.Size{
		Width:  c.roundToPixel(size.Width),
		Height: c.roundToPixel(size.Height),
	}

	// Resize content to fit the canvas
	if c.content != nil {
		contentSize := c.contentSize(c.size)
		// Content resize will trigger layout of children
		// (In a full implementation, content would have a Resize method)
		_ = contentSize // Content resizing handled by painter during rendering
	}
}

// roundToPixel rounds a value to the nearest pixel boundary at the current scale.
// This prevents blurry rendering by aligning to pixel boundaries.
func (c *Canvas) roundToPixel(v float32) float32 {
	pixScale := c.PixScale()
	if pixScale == 0 || pixScale == 1.0 {
		return float32(math.Round(float64(v)))
	}
	return float32(math.Round(float64(v*pixScale))) / pixScale
}

// Scale returns the canvas scale factor (user/system scale).
// This is used to convert between logical coordinates and screen pixels.
func (c *Canvas) Scale() float32 {
	return c.scale
}

// SetScale updates the canvas scale factor.
func (c *Canvas) SetScale(scale float32) {
	if scale <= 0 {
		scale = 1.0
	}
	c.scale = scale
}

// TexScale returns the texture scale factor (framebuffer/window ratio).
// This is typically > 1.0 on HiDPI/Retina displays.
func (c *Canvas) TexScale() float32 {
	return c.texScale
}

// SetTexScale updates the texture scale factor.
func (c *Canvas) SetTexScale(texScale float32) {
	if texScale <= 0 {
		texScale = 1.0
	}
	c.texScale = texScale
}

// PixScale returns the combined pixel scale (scale * texScale).
// This is the total scaling factor for rendering.
func (c *Canvas) PixScale() float32 {
	return c.scale * c.texScale
}

// Padded returns whether the canvas content should have padding.
func (c *Canvas) Padded() bool {
	return c.padded
}

// SetPadded sets whether the canvas content should have padding.
func (c *Canvas) SetPadded(padded bool) {
	c.padded = padded
}

// MinSize returns the minimum size needed for the canvas content.
func (c *Canvas) MinSize() view.Size {
	if c.content == nil {
		return view.Size{Width: 0, Height: 0}
	}
	return c.canvasSize(c.content.MinSize())
}

// canvasSize computes the canvas size needed for the given content size.
// This adds padding if enabled.
func (c *Canvas) canvasSize(contentSize view.Size) view.Size {
	if c.padded {
		// Add padding around content (theme.Padding() would be 4.0 typically)
		padding := float32(4.0) * 2 // padding on both sides
		return view.Size{
			Width:  contentSize.Width + padding,
			Height: contentSize.Height + padding,
		}
	}
	return contentSize
}

// contentSize computes the content size available within the canvas size.
// This subtracts padding if enabled.
func (c *Canvas) contentSize(canvasSize view.Size) view.Size {
	if c.padded {
		padding := float32(4.0) * 2
		return view.Size{
			Width:  canvasSize.Width - padding,
			Height: canvasSize.Height - padding,
		}
	}
	return canvasSize
}

// ContentPos returns the position where content should be drawn.
// This accounts for padding if enabled and rounds to pixel boundaries.
func (c *Canvas) ContentPos() view.Position {
	if c.padded {
		padding := c.roundToPixel(4.0)
		return view.Position{X: padding, Y: padding}
	}
	return view.Position{X: 0, Y: 0}
}

// PixelCoordinateForPosition converts a logical position to physical pixels.
func (c *Canvas) PixelCoordinateForPosition(pos view.Position) (int, int) {
	multiple := c.PixScale()
	scaleInt := func(x float32) int {
		return int(math.Round(float64(x * multiple)))
	}
	return scaleInt(pos.X), scaleInt(pos.Y)
}
