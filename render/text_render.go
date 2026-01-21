package render

import (
	"image"
	"image/color"
	"math"
	"os"
	"sync"

	"github.com/go-text/render"
	"github.com/go-text/typesetting/di"
	"github.com/go-text/typesetting/font"
	"github.com/go-text/typesetting/fontscan"
	"github.com/go-text/typesetting/shaping"
	"golang.org/x/image/math/fixed"
)

// textRenderer handles text rendering using go-text/render
type textRenderer struct {
	fontMap *fontscan.FontMap
	mu      sync.RWMutex
}

var (
	globalTextRenderer *textRenderer
	textRendererOnce   sync.Once
)

// getTextRenderer returns the global text renderer instance
func getTextRenderer() *textRenderer {
	textRendererOnce.Do(func() {
		fm := fontscan.NewFontMap(noopLogger{})

		// Load system fonts
		cacheDir, err := os.UserCacheDir()
		if err == nil {
			if err := fm.UseSystemFonts(cacheDir); err != nil {
				// Log but continue - will use fallback
				// fmt.Printf("Warning: Failed to load system fonts: %v\n", err)
			}
		}

		globalTextRenderer = &textRenderer{fontMap: fm}
	})
	return globalTextRenderer
}

// getFontFace gets a font face for the given style
func (tr *textRenderer) getFontFace(family string, bold, italic bool) shaping.Fontmap {
	tr.mu.RLock()
	defer tr.mu.RUnlock()

	aspect := font.Aspect{Style: font.StyleNormal}
	if italic {
		aspect.Style = font.StyleItalic
	}
	if bold {
		aspect.Weight = font.WeightBold
	}

	tr.fontMap.SetQuery(fontscan.Query{
		Families: []string{family},
		Aspect:   aspect,
	})

	// Return a simple fontmap wrapper
	return &simpleFontMap{fontMap: tr.fontMap}
}

// renderTextToImage renders text to an image
func (tr *textRenderer) renderTextToImage(
	text string,
	fontSize float32,
	col color.Color,
	bold, italic bool,
	scale float32, // DPI scale
) *image.NRGBA {
	if text == "" {
		return nil
	}

	// Get font face
	fontFace := tr.getFontFace(fontscan.SansSerif, bold, italic)

	// Create renderer
	r := render.Renderer{
		FontSize: fontSize,
		PixScale: scale,
		Color:    col,
	}

	// Convert text to runes
	runes := []rune(text)
	if len(runes) == 0 {
		return nil
	}

	// Measure text first to get proper size
	textSize := fixed.Int26_6(fontSize * 64)
	input := shaping.Input{
		Text:      runes,
		RunStart:  0,
		RunEnd:    len(runes),
		Direction: di.DirectionLTR,
		Face:      fontFace.ResolveFace(' '),
		Size:      textSize,
	}

	shaper := &shaping.HarfbuzzShaper{}
	output := shaper.Shape(input)

	// Calculate image size
	width := int(math.Ceil(float64(fixed266ToFloat32(output.Advance) * scale)))
	lineThickness := fixed266ToFloat32(output.LineBounds.LineThickness())
	height := int(math.Ceil(float64(lineThickness * scale)))

	if width <= 0 {
		width = 1
	}
	if height <= 0 {
		height = 1
	}

	// Create image
	img := image.NewNRGBA(image.Rect(0, 0, width, height))

	// Calculate Y position (baseline)
	ascent := fixed266ToFloat32(output.LineBounds.Ascent)
	y := int(math.Ceil(float64(ascent * scale)))

	// Render text using walkString pattern
	advance := float32(0)
	walkString(fontFace, text, textSize, &advance, scale, func(run shaping.Output, x float32) {
		if len(run.Glyphs) == 1 {
			if run.Glyphs[0].GlyphID == 0 {
				// Missing glyph - draw replacement character
				r.DrawStringAt(string([]rune{0xfffd}), img, int(x), y, fontFace.ResolveFace(0xfffd))
				return
			}
		}

		r.DrawShapedRunAt(run, img, int(x), y)
	})

	return img
}

// measureTextSize measures text size without rendering
// Returns size in logical pixels (not physical pixels)
func (tr *textRenderer) measureTextSize(
	text string,
	fontSize float32,
	bold, italic bool,
) (width, height float32) {
	if text == "" {
		return 0, 0
	}

	// Get font face
	fontFace := tr.getFontFace(fontscan.SansSerif, bold, italic)

	// Convert text to runes
	runes := []rune(text)
	if len(runes) == 0 {
		return 0, 0
	}

	// Measure text size
	textSize := fixed.Int26_6(fontSize * 64)

	// Resolve font face - try multiple runes if needed
	resolvedFace := fontFace.ResolveFace(' ')
	if resolvedFace == nil {
		for _, testRune := range []rune{'a', 'A', '0', 'e', 'E'} {
			resolvedFace = fontFace.ResolveFace(testRune)
			if resolvedFace != nil {
				break
			}
		}
	}
	if resolvedFace == nil {
		return 0, 0
	}

	input := shaping.Input{
		Text:      runes,
		RunStart:  0,
		RunEnd:    len(runes),
		Direction: di.DirectionLTR,
		Face:      resolvedFace,
		Size:      textSize,
	}

	shaper := &shaping.HarfbuzzShaper{}
	output := shaper.Shape(input)

	// Check if shaping produced any glyphs
	if len(output.Glyphs) == 0 {
		return 0, 0
	}

	// Calculate size in logical pixels (not scaled by DPI)
	// The advance gives us the width
	width = fixed266ToFloat32(output.Advance)
	
	// Line thickness gives us the height
	lineThickness := fixed266ToFloat32(output.LineBounds.LineThickness())
	height = lineThickness

	return width, height
}

// renderTextToImageRGBA renders text and returns RGBA (for compatibility with drawImage)
func (tr *textRenderer) renderTextToImageRGBA(
	text string,
	fontSize float32,
	col color.Color,
	bold, italic bool,
	scale float32,
) *image.RGBA {
	nrgba := tr.renderTextToImage(text, fontSize, col, bold, italic, scale)
	if nrgba == nil {
		return nil
	}

	// Convert NRGBA to RGBA (they're compatible, just copy the pixels)
	rgba := image.NewRGBA(nrgba.Bounds())
	copy(rgba.Pix, nrgba.Pix)
	return rgba
}

// walkString walks through text and calls callback for each shaped run
func walkString(
	faces shaping.Fontmap,
	s string,
	textSize fixed.Int26_6,
	advance *float32,
	scale float32,
	cb func(run shaping.Output, x float32),
) {
	runes := []rune(s)
	if len(runes) == 0 {
		return
	}

	in := shaping.Input{
		Text:      runes,
		RunStart:  0,
		RunEnd:    len(runes),
		Direction: di.DirectionLTR,
		Face:      faces.ResolveFace(' '),
		Size:      textSize,
	}

	shaper := &shaping.HarfbuzzShaper{}
	output := shaper.Shape(in)

	// Simple approach: render the entire shaped output as one run
	x := float32(0)
	cb(output, x)

	// Calculate total advance
	totalAdvance := fixed.Int26_6(0)
	for _, g := range output.Glyphs {
		totalAdvance += g.XAdvance
	}
	*advance = fixed266ToFloat32(totalAdvance) * scale
}

// Helper functions
func fixed266ToFloat32(i fixed.Int26_6) float32 {
	return float32(float64(i) / (1 << 6))
}

type noopLogger struct{}

func (n noopLogger) Printf(string, ...interface{}) {}

// simpleFontMap wraps fontscan.FontMap to implement shaping.Fontmap
type simpleFontMap struct {
	fontMap *fontscan.FontMap
}

func (sfm *simpleFontMap) ResolveFace(r rune) *font.Face {
	return sfm.fontMap.ResolveFace(r)
}
