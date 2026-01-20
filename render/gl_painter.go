package render

import (
	"fmt"
	"image"
	"image/color"
	_ "image/png"
	"sync"

	"github.com/go-gl/gl/v4.1-core/gl"
	gofont "golang.org/x/image/font"
	"golang.org/x/image/font/basicfont"
	"golang.org/x/image/math/fixed"

	"github.com/notblessy/shellui/core/view"
	"github.com/notblessy/shellui/widget/button"
	"github.com/notblessy/shellui/widget/text"
)

// glyphCacheEntry stores a cached glyph texture, image, and its metrics
type glyphCacheEntry struct {
	texture uint32
	image   *image.RGBA // Store image for direct drawing
	width   int
	height  int
}

// glyphCacheKey is used to uniquely identify a glyph in the cache
type glyphCacheKey struct {
	rune  rune
	scale float32
	bold  bool
}

// GLPainterType is an OpenGL-based painter implementation.
type GLPainterType struct {
	PainterType
	textureCache  map[string]uint32                 // Cache for full text strings (legacy)
	glyphCache    map[glyphCacheKey]glyphCacheEntry // Cache for individual glyphs (legacy, kept for compatibility)
	cacheMutex    sync.RWMutex
	shaderProgram uint32        // Shader program for texture rendering
	quadVAO       uint32        // VAO for quad rendering
	quadVBO       uint32        // VBO for quad vertices
	textRenderer  *textRenderer // Text renderer using go-text/render (Fyne-style)
	dpiScale      float32       // DPI scale factor
}

// NewGLPainter creates a new OpenGL-based painter.
func NewGLPainter(width, height int) Painter {
	p := &GLPainterType{
		PainterType: PainterType{
			width:  width,
			height: height,
		},
		textureCache: make(map[string]uint32),
		glyphCache:   make(map[glyphCacheKey]glyphCacheEntry),
		dpiScale:     1.0,
	}

	// Initialize text renderer (Fyne-style, uses system fonts)
	p.textRenderer = getTextRenderer()

	// Initialize shaders and quad rendering
	p.initShaders()
	p.initQuad()

	return p
}

// Note: Roboto font loading removed - now using system fonts via textRenderer

// initShaders initializes the shader program for texture rendering
func (p *GLPainterType) initShaders() {
	program, err := createShaderProgram(vertexShaderSource, fragmentShaderSource)
	if err != nil {
		panic(fmt.Sprintf("Failed to create shader program: %v", err))
	}
	p.shaderProgram = program
}

// initQuad initializes the quad VAO and VBO for rendering
func (p *GLPainterType) initQuad() {
	// Quad vertices: position (x, y) and texture coordinates (u, v)
	// Note: Go images have origin at top-left, but OpenGL textures have origin at bottom-left
	// So we flip the V coordinate to match
	vertices := []float32{
		// positions   // tex coords (V flipped: 1.0 at bottom, 0.0 at top)
		0.0, 0.0, 0.0, 1.0, // bottom-left (image top-left)
		1.0, 0.0, 1.0, 1.0, // bottom-right (image top-right)
		0.0, 1.0, 0.0, 0.0, // top-left (image bottom-left)
		1.0, 1.0, 1.0, 0.0, // top-right (image bottom-right)
	}

	var vao, vbo uint32
	gl.GenVertexArrays(1, &vao)
	gl.GenBuffers(1, &vbo)

	gl.BindVertexArray(vao)
	gl.BindBuffer(gl.ARRAY_BUFFER, vbo)
	gl.BufferData(gl.ARRAY_BUFFER, len(vertices)*4, gl.Ptr(vertices), gl.STATIC_DRAW)

	// Position attribute
	gl.VertexAttribPointer(0, 2, gl.FLOAT, false, 4*4, gl.PtrOffset(0))
	gl.EnableVertexAttribArray(0)

	// Texture coordinate attribute
	gl.VertexAttribPointer(1, 2, gl.FLOAT, false, 4*4, gl.PtrOffset(2*4))
	gl.EnableVertexAttribArray(1)

	gl.BindBuffer(gl.ARRAY_BUFFER, 0)
	gl.BindVertexArray(0)

	p.quadVAO = vao
	p.quadVBO = vbo
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

// drawText renders a text widget (following Fyne's pattern: text -> image -> texture)
func (p *GLPainterType) drawText(t *text.TextType, x, y, width, height float32) {
	content := t.GetContent()
	if content == "" {
		return
	}

	// Get font properties
	fontSize := t.GetSize()
	if fontSize == 0 {
		fontSize = 16 // Default font size
	}

	// Get color (default to black if not specified)
	textColor := &color.RGBA{R: 0, G: 0, B: 0, A: 255} // Default black

	// Determine bold/italic
	isBold := t.IsBold() || t.GetWeight() == text.FontWeightBold
	isItalic := false // Add italic support if needed

	// Render text to image using go-text/render (Fyne-style)
	if p.textRenderer == nil {
		// Fallback if renderer not initialized
		return
	}

	img := p.textRenderer.renderTextToImageRGBA(
		content,
		fontSize,
		textColor,
		isBold,
		isItalic,
		p.dpiScale,
	)

	if img == nil {
		return
	}

	// Get text size
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

	// Draw as texture (like Fyne)
	p.drawImage(img, textX, textY, textWidth, textHeight)
}

// measureTextWidth measures the total width of text without rendering it.
func (p *GLPainterType) measureTextWidth(content string, face gofont.Face, scale float32) float32 {
	var totalWidth float32
	for _, r := range content {
		glyphAdvance, ok := face.GlyphAdvance(r)
		if !ok {
			glyphAdvance = fixed.Int26_6(7 * 64) // Default width for basicfont
		}
		advance := float32(face.Kern(0, r).Ceil()) + float32(glyphAdvance.Ceil())
		totalWidth += advance * scale
	}
	return totalWidth
}

// getGlyphTexture gets or creates a texture for a single glyph.
func (p *GLPainterType) getGlyphTexture(r rune, face gofont.Face, scale float32, bold bool) glyphCacheEntry {
	// Create cache key (includes rune, scale, and bold for proper caching)
	key := glyphCacheKey{
		rune:  r,
		scale: scale,
		bold:  bold,
	}

	// Check cache first
	p.cacheMutex.RLock()
	if entry, ok := p.glyphCache[key]; ok {
		p.cacheMutex.RUnlock()
		return entry
	}
	p.cacheMutex.RUnlock()

	// Create glyph image
	glyphImg := p.renderGlyphToImage(r, face, scale, bold)
	if glyphImg == nil {
		return glyphCacheEntry{}
	}

	// Create texture from glyph image
	texture := p.createTextureFromImage(glyphImg)
	entry := glyphCacheEntry{
		texture: texture,
		image:   glyphImg,
		width:   glyphImg.Bounds().Dx(),
		height:  glyphImg.Bounds().Dy(),
	}

	// Cache it
	p.cacheMutex.Lock()
	p.glyphCache[key] = entry
	p.cacheMutex.Unlock()

	return entry
}

// renderGlyphToImage renders a single glyph to an image.
func (p *GLPainterType) renderGlyphToImage(r rune, face gofont.Face, scale float32, bold bool) *image.RGBA {
	// Use the same approach as textToImage but for a single character
	// Measure the single character
	charStr := string(r)
	advance := gofont.MeasureString(face, charStr)
	width := advance.Ceil()
	height := face.Metrics().Height.Ceil()

	// Add extra width for bold
	if bold {
		width += 1
	}

	// Create image at base size
	img := image.NewRGBA(image.Rect(0, 0, width, height))

	// Create drawer - use the exact same pattern as textToImage
	d := &gofont.Drawer{
		Dst:  img,
		Src:  image.NewUniform(color.Black),
		Face: face,
		Dot: fixed.Point26_6{
			X: 0,
			Y: fixed.Int26_6(face.Metrics().Ascent),
		},
	}

	// Draw the glyph
	d.DrawString(charStr)

	// Apply bold effect if needed
	if bold {
		d.Dot = fixed.Point26_6{
			X: fixed.Int26_6(1),
			Y: fixed.Int26_6(face.Metrics().Ascent),
		}
		d.DrawString(charStr)
	}

	// Scale if needed
	if scale != 1.0 {
		return p.scaleImage(img, scale)
	}

	return img
}

// createTextureFromImage creates an OpenGL texture from an image.
func (p *GLPainterType) createTextureFromImage(img *image.RGBA) uint32 {
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

	return texture
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
// VStack default: height 100%, width auto, no padding/margin
// Children are laid out vertically with natural sizing (not stretched).
func (p *GLPainterType) drawVStack(vs *view.VStackType, x, y, width, height float32) {
	children := vs.GetChildren()
	if len(children) == 0 {
		return
	}

	// Apply width/height styling
	stackWidth := width

	if vs.GetWidth() >= 0 {
		// Fixed width specified
		stackWidth = vs.GetWidth()
	}
	// If width < 0, use natural/auto width (use available width)

	// VStack uses natural height (sum of children) unless explicitly styled
	// No padding or margin by default (SwiftUI-like)
	// Children are laid out vertically starting from top
	// First, measure all children to calculate natural height
	type childInfo struct {
		view   view.View
		height float32
	}
	childInfos := make([]childInfo, 0, len(children))

	for _, child := range children {
		if child != nil {
			// Measure child to get natural height
			var childHeight float32
			if textChild, ok := child.(*text.TextType); ok {
				// Measure text to get natural height
				content := textChild.GetContent()
				if content != "" && p.textRenderer != nil {
					fontSize := textChild.GetSize()
					if fontSize == 0 {
						fontSize = 16
					}
					isBold := textChild.IsBold() || textChild.GetWeight() == text.FontWeightBold
					img := p.textRenderer.renderTextToImageRGBA(
						content,
						fontSize,
						&color.RGBA{R: 0, G: 0, B: 0, A: 255},
						isBold,
						false,
						p.dpiScale,
					)
					if img != nil {
						childHeight = float32(img.Bounds().Dy())
					} else {
						childHeight = 20 // Fallback
					}
				} else {
					childHeight = 20 // Fallback
				}
			} else {
				// For other view types, use a reasonable default
				// A proper layout engine would measure all children first
				childHeight = 20 // Fallback height
			}

			childInfos = append(childInfos, childInfo{view: child, height: childHeight})
		}
	}

	// Render children from top to bottom (first child at top)
	// Start at y (top of VStack), no centering or spacing
	currentY := y
	for i := 0; i < len(childInfos); i++ {
		info := childInfos[i]
		// Render child with natural height (not stretched)
		// Position from top, moving downward
		p.Paint(info.view, x, currentY, stackWidth, info.height)

		// Move to next position (downward)
		currentY += info.height
	}
}

// drawHStack renders a horizontal stack.
// HStack default: width 100%, height auto, no padding/margin
// Children are laid out horizontally with natural sizing (not stretched).
func (p *GLPainterType) drawHStack(hs *view.HStackType, x, y, width, height float32) {
	children := hs.GetChildren()
	if len(children) == 0 {
		return
	}

	// Apply width/height styling
	stackWidth := width
	stackHeight := height

	if hs.GetWidth() >= 0 {
		// Fixed width specified
		stackWidth = hs.GetWidth()
	}
	// If width < 0, use natural/auto width (defaults to 100% of available, which is already set)

	if hs.GetHeight() >= 0 {
		// Fixed height specified
		stackHeight = hs.GetHeight()
	}
	// If height < 0, use natural/auto height (use available height)

	// No padding or margin by default (SwiftUI-like)
	// Children are laid out horizontally starting from left
	currentX := x

	// Stacks don't draw backgrounds - just render children
	// Each child gets its natural width (height from parent)
	for _, child := range children {
		if child != nil {
			// Measure child to get natural width
			var childWidth float32
			if textChild, ok := child.(*text.TextType); ok {
				// Measure text to get natural width
				content := textChild.GetContent()
				if content != "" && p.textRenderer != nil {
					fontSize := textChild.GetSize()
					if fontSize == 0 {
						fontSize = 16
					}
					isBold := textChild.IsBold() || textChild.GetWeight() == text.FontWeightBold
					img := p.textRenderer.renderTextToImageRGBA(
						content,
						fontSize,
						&color.RGBA{R: 0, G: 0, B: 0, A: 255},
						isBold,
						false,
						p.dpiScale,
					)
					if img != nil {
						childWidth = float32(img.Bounds().Dx())
					} else {
						childWidth = 50 // Fallback
					}
				} else {
					childWidth = 50 // Fallback
				}
			} else if _, ok := child.(*view.SpacerType); ok {
				// Spacer takes remaining space
				remainingWidth := stackWidth - (currentX - x)
				childWidth = remainingWidth
			} else {
				// For other view types, use a reasonable default
				childWidth = 50 // Fallback width
			}

			// Render child with natural width (not stretched)
			p.Paint(child, currentX, y, childWidth, stackHeight)

			// Move to next position
			currentX += childWidth
		}
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

// textToImage renders text to an image using basicfont (backward compatibility).
// This is the original working implementation - kept for reference and fallback.
func (p *GLPainterType) textToImage(content string) *image.RGBA {
	if content == "" {
		return nil
	}

	face := basicfont.Face7x13
	d := &gofont.Drawer{
		Dst:  nil, // Will create below
		Src:  image.NewUniform(color.Black),
		Face: face,
		Dot:  fixed.Point26_6{},
	}

	// Measure text to determine image size
	advance := gofont.MeasureString(face, content)
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

// textToImageWithStyle renders text to an image using basicfont with styling.
// This follows the same pattern as the original textToImage but adds font styling support.
func (p *GLPainterType) textToImageWithStyle(content string, fontSize float32, bold bool, weight text.FontWeight) *image.RGBA {
	if content == "" {
		return nil
	}

	// If no styling is applied, use the original function for maximum compatibility
	if fontSize == 0 && !bold && weight == text.FontWeightRegular {
		return p.textToImage(content)
	}

	// Use basicfont.Face7x13 as base (this is a bitmap font, so we'll scale the image)
	face := basicfont.Face7x13

	// Calculate scale factor based on font size
	// Default size is 13 points (matching Face7x13)
	defaultSize := float32(13)
	scale := float32(1.0)
	if fontSize > 0 {
		scale = fontSize / defaultSize
	}

	// For bold, we'll render the text twice with slight offset to simulate boldness
	isBold := bold || weight == text.FontWeightBold

	// Create drawer - match the original working pattern exactly
	d := &gofont.Drawer{
		Dst:  nil, // Will create below
		Src:  image.NewUniform(color.Black),
		Face: face,
		Dot:  fixed.Point26_6{},
	}

	// Measure text to determine image size at base size
	advance := gofont.MeasureString(face, content)
	baseWidth := advance.Ceil()
	baseHeight := face.Metrics().Height.Ceil()

	// Add extra width for bold effect
	if isBold {
		baseWidth += 1
	}

	// Create image at base size first
	baseImg := image.NewRGBA(image.Rect(0, 0, baseWidth, baseHeight))
	// Initialize to fully transparent (already zero-initialized)

	d.Dst = baseImg
	// Set the dot position to account for font metrics
	// Y position needs to be at the baseline - use the original pattern
	d.Dot = fixed.Point26_6{
		X: 0,
		Y: fixed.Int26_6(face.Metrics().Ascent),
	}

	// Draw text at base size
	d.DrawString(content)

	// Apply bold effect by drawing text again with slight offset
	if isBold {
		d.Dot = fixed.Point26_6{
			X: fixed.Int26_6(1), // 1 pixel offset for bold
			Y: fixed.Int26_6(face.Metrics().Ascent),
		}
		d.DrawString(content)
	}

	// Scale the image if needed
	if scale != 1.0 {
		return p.scaleImage(baseImg, scale)
	}

	return baseImg
}

// scaleImage scales an image by the given factor.
func (p *GLPainterType) scaleImage(src *image.RGBA, scale float32) *image.RGBA {
	if scale == 1.0 {
		return src
	}

	bounds := src.Bounds()
	srcWidth := bounds.Dx()
	srcHeight := bounds.Dy()

	dstWidth := int(float32(srcWidth) * scale)
	dstHeight := int(float32(srcHeight) * scale)

	dst := image.NewRGBA(image.Rect(0, 0, dstWidth, dstHeight))

	// Simple nearest-neighbor scaling
	for y := 0; y < dstHeight; y++ {
		for x := 0; x < dstWidth; x++ {
			srcX := int(float32(x) / scale)
			srcY := int(float32(y) / scale)

			if srcX < srcWidth && srcY < srcHeight {
				srcIdx := (srcY*srcWidth + srcX) * 4
				dstIdx := (y*dstWidth + x) * 4

				if srcIdx+3 < len(src.Pix) && dstIdx+3 < len(dst.Pix) {
					dst.Pix[dstIdx] = src.Pix[srcIdx]
					dst.Pix[dstIdx+1] = src.Pix[srcIdx+1]
					dst.Pix[dstIdx+2] = src.Pix[srcIdx+2]
					dst.Pix[dstIdx+3] = src.Pix[srcIdx+3]
				}
			}
		}
	}

	return dst
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
// width and height parameters should match the actual image size - image is never scaled.
// drawImageWithShader renders an image using shader-based quad rendering
func (p *GLPainterType) drawImageWithShader(texture uint32, x, y, width, height float32) {
	// Enable blending for transparency
	gl.Enable(gl.BLEND)
	gl.BlendFunc(gl.SRC_ALPHA, gl.ONE_MINUS_SRC_ALPHA)
	defer gl.Disable(gl.BLEND)

	// Use shader program
	gl.UseProgram(p.shaderProgram)

	// Bind texture
	gl.ActiveTexture(gl.TEXTURE0)
	gl.BindTexture(gl.TEXTURE_2D, texture)

	// Set texture uniform
	texUniform := gl.GetUniformLocation(p.shaderProgram, gl.Str("uTexture\x00"))
	gl.Uniform1i(texUniform, 0)

	// Set color uniform (white, full opacity)
	colorUniform := gl.GetUniformLocation(p.shaderProgram, gl.Str("uColor\x00"))
	gl.Uniform4f(colorUniform, 1.0, 1.0, 1.0, 1.0)

	// Calculate transform matrix
	// Convert from screen coordinates to normalized device coordinates
	// OpenGL uses bottom-left origin, so we need to flip Y
	flippedY := float32(p.height) - y - height

	// Transform: scale and translate
	// Scale: width/height to screen size, then to NDC
	scaleX := (width / float32(p.width)) * 2.0
	scaleY := (height / float32(p.height)) * 2.0
	translateX := (x/float32(p.width))*2.0 - 1.0
	translateY := (flippedY/float32(p.height))*2.0 - 1.0

	// Create 3x3 transform matrix (mat3)
	transform := [9]float32{
		scaleX, 0, 0,
		0, scaleY, 0,
		translateX, translateY, 1.0,
	}

	transformUniform := gl.GetUniformLocation(p.shaderProgram, gl.Str("transform\x00"))
	gl.UniformMatrix3fv(transformUniform, 1, false, &transform[0])

	// Bind and draw quad
	gl.BindVertexArray(p.quadVAO)
	gl.DrawArrays(gl.TRIANGLE_STRIP, 0, 4)
	gl.BindVertexArray(0)
}

// drawImage renders an image using shader-based rendering (replaces old pixel-by-pixel method)
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

	// Ensure we use the actual image size, not the layout-provided size
	// This prevents text from being stretched when window is resized
	actualWidth := float32(imgWidth)
	actualHeight := float32(imgHeight)

	// Get or create texture for this image
	texture := p.getOrCreateTexture(img)
	if texture == 0 {
		return
	}

	// Use shader-based rendering
	p.drawImageWithShader(texture, x, y, actualWidth, actualHeight)
}

// getOrCreateTexture gets or creates a texture for an image
func (p *GLPainterType) getOrCreateTexture(img *image.RGBA) uint32 {
	// For now, create a new texture each time (can be optimized with caching)
	// In a production system, you'd cache textures by image content hash
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

	return texture
}

// NOTE: renderTextWithRoboto and renderBitmapGlyph removed
// These functions are no longer used - text rendering now uses textRenderer
// which follows Fyne's pattern with go-text/render

func drawButtonBackground(x, y, width, height float32) {
	gl.Enable(gl.SCISSOR_TEST)
	gl.Scissor(int32(x), int32(y), int32(width), int32(height))
	gl.ClearColor(0.7, 0.7, 0.7, 1.0) // Light gray
	gl.Clear(gl.COLOR_BUFFER_BIT)
	gl.Disable(gl.SCISSOR_TEST)
}
