package text

import "github.com/notblessy/shellui/core/view"

// TextAlign represents the horizontal alignment of text.
type TextAlign int

const (
	// TextAlignLeading aligns text to the leading edge (left in LTR languages).
	TextAlignLeading TextAlign = iota
	// TextAlignCenter centers text horizontally.
	TextAlignCenter
	// TextAlignTrailing aligns text to the trailing edge (right in LTR languages).
	TextAlignTrailing
)

// FontWeight represents the weight/thickness of the font.
type FontWeight int

const (
	// FontWeightRegular is the default/normal font weight.
	FontWeightRegular FontWeight = iota
	// FontWeightBold is bold font weight.
	FontWeightBold
	// FontWeightLight is light font weight.
	FontWeightLight
	// FontWeightMedium is medium font weight.
	FontWeightMedium
	// FontWeightSemibold is semibold font weight.
	FontWeightSemibold
)

// TextType is a view that displays text.
type TextType struct {
	view.ViewBaseType
	content   string
	alignment TextAlign
	size      float32 // Font size in points (default 0 means use system default)
	weight    FontWeight
	bold      bool // Quick way to set bold
}

// NewText creates a new TextType view.
func NewText(content string) *TextType {
	return &TextType{
		content:   content,
		alignment: TextAlignLeading, // Default to leading (left-aligned)
		size:      0,                // 0 means use system default
		weight:    FontWeightRegular,
		bold:      false,
	}
}

// Body returns nil for TextType (it's a leaf view).
func (t *TextType) Body() view.View {
	return nil
}

// GetContent returns the text content.
func (t *TextType) GetContent() string {
	return t.content
}

// GetAlignment returns the text alignment.
func (t *TextType) GetAlignment() TextAlign {
	return t.alignment
}

// Alignment sets the text alignment (SwiftUI-style modifier).
func (t *TextType) Alignment(align TextAlign) *TextType {
	t.alignment = align
	return t
}

// Size sets the font size in points (SwiftUI-style modifier).
func (t *TextType) Size(size float32) *TextType {
	t.size = size
	return t
}

// Weight sets the font weight (SwiftUI-style modifier).
func (t *TextType) Weight(weight FontWeight) *TextType {
	t.weight = weight
	if weight == FontWeightBold {
		t.bold = true
	} else {
		t.bold = false
	}
	return t
}

// Bold sets the text to bold (SwiftUI-style modifier).
func (t *TextType) Bold() *TextType {
	t.bold = true
	t.weight = FontWeightBold
	return t
}

// GetSize returns the font size.
func (t *TextType) GetSize() float32 {
	return t.size
}

// GetWeight returns the font weight.
func (t *TextType) GetWeight() FontWeight {
	return t.weight
}

// IsBold returns whether the text is bold.
func (t *TextType) IsBold() bool {
	return t.bold
}

// MinSize returns the minimum size needed to display this text without stretching.
// This is the natural size of the text in logical pixels.
func (t *TextType) MinSize() view.Size {
	fontSize := t.size
	if fontSize <= 0 {
		fontSize = 16 // Default font size
	}
	
	// Use the global text measurer (registered by the painter)
	return view.MeasureText(t.content, fontSize, t.bold, false)
}

// Text is a convenience function for creating text views.
// Returns *TextType to allow method chaining.
func Text(content string) *TextType {
	return NewText(content)
}
