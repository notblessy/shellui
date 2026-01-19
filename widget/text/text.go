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

// TextType is a view that displays text.
type TextType struct {
	view.ViewBaseType
	content   string
	alignment TextAlign
}

// NewText creates a new TextType view.
func NewText(content string) *TextType {
	return &TextType{
		content:   content,
		alignment: TextAlignLeading, // Default to leading (left-aligned)
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

// Text is a convenience function for creating text views.
func Text(content string) view.View {
	return NewText(content)
}
