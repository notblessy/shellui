package view

// TextMeasurer is an interface for measuring text size.
// This allows views to measure text without importing the render package.
type TextMeasurer interface {
	// MeasureText returns the natural size of text in logical pixels.
	MeasureText(content string, fontSize float32, bold, italic bool) Size
}

// Global text measurer - set by the painter during initialization.
var globalTextMeasurer TextMeasurer

// SetTextMeasurer sets the global text measurer.
// This should be called by the painter during initialization.
func SetTextMeasurer(m TextMeasurer) {
	globalTextMeasurer = m
}

// GetTextMeasurer returns the global text measurer.
// Returns nil if not set.
func GetTextMeasurer() TextMeasurer {
	return globalTextMeasurer
}

// MeasureText measures text size using the global text measurer.
// Returns (0, 0) if no measurer is set.
func MeasureText(content string, fontSize float32, bold, italic bool) Size {
	if globalTextMeasurer == nil {
		return Size{Width: 0, Height: 0}
	}
	return globalTextMeasurer.MeasureText(content, fontSize, bold, italic)
}
