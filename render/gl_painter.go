package render

import (
	"image"
	"image/color"
	"sync"

	"github.com/go-gl/gl/v4.1-core/gl"
	"golang.org/x/image/font"
	"golang.org/x/image/font/basicfont"
	"golang.org/x/image/math/fixed"

	"github.com/notblessy/shellui/core/view"
	"github.com/notblessy/shellui/widget/button"
	"github.com/notblessy/shellui/widget/text"
)

// GLPainterType is an OpenGL-based painter implementation.
type GLPainterType struct {
	PainterType
	textureCache map[string]uint32 // Cache for text textures
	cacheMutex   sync.RWMutex
}

// NewGLPainter creates a new OpenGL-based painter.
func NewGLPainter(width, height int) Painter {
	return &GLPainterType{
		PainterType: PainterType{
			width:  width,
			height: height,
		},
		textureCache: make(map[string]uint32),
	}
}

// Clear clears the screen with background color.
func (p *GLPainterType) Clear() {
	// Light gray background
	gl.ClearColor(0.9, 0.9, 0.9, 1.0)
	gl.Clear(gl.COLOR_BUFFER_BIT)
}

// Paint paints a view at the specified position and size.
func (p *GLPainterType) Paint(v view.View, x, y, width, height float32) {
	if v == nil {
		return
	}

	// Dispatch to specific draw methods based on view type
	p.drawObject(v, x, y, width, height)
}

// drawObject dispatches to specific draw methods based on view type.
func (p *GLPainterType) drawObject(v view.View, x, y, width, height float32) {
	switch obj := v.(type) {
	case *text.TextType:
		p.drawText(obj, x, y, width, height)
	case *button.ButtonType:
		p.drawButton(obj, x, y, width, height)
	case *view.VStackType:
		p.drawVStack(obj, x, y, width, height)
	case *view.HStackType:
		p.drawHStack(obj, x, y, width, height)
	case *view.ZStackType:
		p.drawZStack(obj, x, y, width, height)
	case *view.SpacerType:
		// Spacers are transparent - they don't draw anything
		return
	default:
		// Try to get Body and render that
		body := v.Body()
		if body != nil && body != v {
			p.drawObject(body, x, y, width, height)
		}
	}
}

// drawText renders a text widget:
// 1. Render text to an image
// 2. Draw image directly (can be upgraded to texture later)
func (p *GLPainterType) drawText(t *text.TextType, x, y, width, height float32) {
	content := t.GetContent()
	if content == "" {
		return
	}

	// Render text to image
	img := p.textToImage(content)
	if img == nil {
		return
	}

	// Calculate text metrics for alignment
	textWidth := float32(img.Bounds().Dx())
	textHeight := float32(img.Bounds().Dy())

	// Calculate X position based on alignment
	var textX float32
	switch t.GetAlignment() {
	case text.TextAlignLeading:
		textX = x
	case text.TextAlignCenter:
		textX = x + (width-textWidth)/2
	case text.TextAlignTrailing:
		textX = x + width - textWidth
	default:
		textX = x
	}

	// Center vertically
	textY := y + (height-textHeight)/2

	// Convert to OpenGL coordinates (bottom-left origin)
	glX := textX
	glY := float32(p.height) - (textY + textHeight)
	glWidth := textWidth
	glHeight := textHeight

	// Draw image directly (simple approach, can upgrade to texture later)
	p.drawImage(img, glX, glY, glWidth, glHeight)
}

// drawButton renders a button widget.
func (p *GLPainterType) drawButton(b *button.ButtonType, x, y, width, height float32) {
	label := b.GetLabel()
	if label == "" {
		return
	}

	// Draw button background
	flippedY := float32(p.height) - y - height
	drawButtonBackground(x, flippedY, width, height)

	// Draw button text
	textX := x + 10
	textY := y + 7
	p.drawText(text.NewText(label), textX, textY, width-20, height-14)
}

// drawVStack renders a vertical stack.
// Stacks are transparent containers - they don't draw backgrounds.
func (p *GLPainterType) drawVStack(vs *view.VStackType, x, y, width, height float32) {
	children := vs.GetChildren()
	if len(children) == 0 {
		return
	}

	// Simple layout: divide height equally among children
	// Add some padding
	padding := float32(10)
	availableHeight := height - float32(len(children)-1)*padding
	childHeight := availableHeight / float32(len(children))
	currentY := y + padding // Start with some padding from top

	// Stacks don't draw backgrounds - just render children
	for _, child := range children {
		if child != nil {
			p.Paint(child, x+padding, currentY, width-padding*2, childHeight)
		}
		currentY += childHeight + padding
	}
}

// drawHStack renders a horizontal stack.
// Stacks are transparent containers - they don't draw backgrounds.
func (p *GLPainterType) drawHStack(hs *view.HStackType, x, y, width, height float32) {
	children := hs.GetChildren()
	if len(children) == 0 {
		return
	}

	// Simple layout: divide width equally among children
	childWidth := width / float32(len(children))
	currentX := x

	// Stacks don't draw backgrounds - just render children
	for _, child := range children {
		if child != nil {
			p.Paint(child, currentX, y, childWidth, height)
		}
		currentX += childWidth
	}
}

// drawZStack renders a z-ordered stack (layers).
func (p *GLPainterType) drawZStack(zs *view.ZStackType, x, y, width, height float32) {
	children := zs.GetChildren()
	// Render all children at the same position (layered)
	for _, child := range children {
		p.Paint(child, x, y, width, height)
	}
}

// StartClipping starts clipping to the specified area.
func (p *GLPainterType) StartClipping(x, y, width, height float32) {
	flippedY := float32(p.height) - y - height
	gl.Enable(gl.SCISSOR_TEST)
	gl.Scissor(int32(x), int32(flippedY), int32(width), int32(height))
}

// StopClipping stops clipping.
func (p *GLPainterType) StopClipping() {
	gl.Disable(gl.SCISSOR_TEST)
}

// textToImage renders text to an image using basicfont.
func (p *GLPainterType) textToImage(content string) *image.RGBA {
	if content == "" {
		return nil
	}

	face := basicfont.Face7x13
	d := &font.Drawer{
		Dst:  nil, // Will create below
		Src:  image.NewUniform(color.Black),
		Face: face,
		Dot:  fixed.Point26_6{},
	}

	// Measure text to determine image size
	advance := font.MeasureString(face, content)
	width := advance.Ceil()
	height := face.Metrics().Height.Ceil()

	// Create image with transparent background
	img := image.NewRGBA(image.Rect(0, 0, width, height))
	// Initialize to fully transparent
	// The image is already zero-initialized which gives us transparent pixels

	d.Dst = img
	// Set the dot position to account for font metrics
	// Y position needs to be at the baseline
	d.Dot = fixed.Point26_6{
		X: 0,
		Y: fixed.Int26_6(face.Metrics().Ascent),
	}

	// Draw text (this will draw black text on transparent background)
	d.DrawString(content)

	return img
}

// getTextTexture gets or creates a texture for the given text.
func (p *GLPainterType) getTextTexture(content string, img *image.RGBA) uint32 {
	p.cacheMutex.RLock()
	if tex, ok := p.textureCache[content]; ok {
		p.cacheMutex.RUnlock()
		return tex
	}
	p.cacheMutex.RUnlock()

	// Create new texture
	var texture uint32
	gl.GenTextures(1, &texture)
	gl.BindTexture(gl.TEXTURE_2D, texture)

	// Set texture parameters
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MIN_FILTER, gl.LINEAR)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_MAG_FILTER, gl.LINEAR)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_S, gl.CLAMP_TO_EDGE)
	gl.TexParameteri(gl.TEXTURE_2D, gl.TEXTURE_WRAP_T, gl.CLAMP_TO_EDGE)

	// Upload image data
	width := img.Bounds().Dx()
	height := img.Bounds().Dy()
	gl.TexImage2D(
		gl.TEXTURE_2D,
		0,
		gl.RGBA,
		int32(width),
		int32(height),
		0,
		gl.RGBA,
		gl.UNSIGNED_BYTE,
		gl.Ptr(img.Pix),
	)

	// Cache texture
	p.cacheMutex.Lock()
	p.textureCache[content] = texture
	p.cacheMutex.Unlock()

	return texture
}

// drawImage draws an image directly to the screen (simple approach).
// Renders to image first, then draws. Can be upgraded to use textures and shaders later for better performance.
func (p *GLPainterType) drawImage(img *image.RGBA, x, y, width, height float32) {
	if img == nil {
		return
	}

	bounds := img.Bounds()
	imgWidth := bounds.Dx()
	imgHeight := bounds.Dy()

	if imgWidth == 0 || imgHeight == 0 {
		return
	}

	// Enable blending for transparency
	gl.Enable(gl.BLEND)
	gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
	defer gl.Disable(gl.BLEND)

	// Use scissor test to clip to the drawing area
	gl.Enable(gl.SCISSOR_TEST)
	defer gl.Disable(gl.SCISSOR_TEST)

	// Set scissor to the entire image area
	gl.Scissor(int32(x), int32(y), int32(width), int32(height))

	// Draw image pixels - use actual image size
	// Draw each pixel as a 1x1 rectangle
	// Note: y is already in OpenGL coordinates (bottom-left origin)
	for py := 0; py < imgHeight; py++ {
		for px := 0; px < imgWidth; px++ {
			idx := (py*imgWidth + px) * 4
			if idx+3 < len(img.Pix) {
				r, g, b, a := img.Pix[idx], img.Pix[idx+1], img.Pix[idx+2], img.Pix[idx+3]
				if a > 0 { // Only draw non-transparent pixels
					// Calculate pixel position
					// x increases to the right, y increases upward in OpenGL
					drawX := int32(x) + int32(px)
					// Image is drawn top-to-bottom, but we're at bottom y, so draw upward
					drawY := int32(y) + int32(imgHeight-1-py)

					// Draw single pixel
					gl.Scissor(drawX, drawY, 1, 1)
					gl.ClearColor(float32(r)/255.0, float32(g)/255.0, float32(b)/255.0, float32(a)/255.0)
					gl.Clear(gl.COLOR_BUFFER_BIT)
				}
			}
		}
	}
}

func drawButtonBackground(x, y, width, height float32) {
	gl.Enable(gl.SCISSOR_TEST)
	gl.Scissor(int32(x), int32(y), int32(width), int32(height))
	gl.ClearColor(0.7, 0.7, 0.7, 1.0) // Light gray
	gl.Clear(gl.COLOR_BUFFER_BIT)
	gl.Disable(gl.SCISSOR_TEST)
}
