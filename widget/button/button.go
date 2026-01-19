package button

import "github.com/notblessy/shellui/core/view"

// ButtonType is a view that represents a button.
type ButtonType struct {
	view.ViewBaseType
	label  string
	action func()
}

// NewButton creates a new ButtonType view.
func NewButton(label string, action func()) *ButtonType {
	return &ButtonType{
		label:  label,
		action: action,
	}
}

// Body returns nil for ButtonType (it's a leaf view).
func (b *ButtonType) Body() view.View {
	return nil
}

// GetLabel returns the button label.
func (b *ButtonType) GetLabel() string {
	return b.label
}

// GetAction returns the button action.
func (b *ButtonType) GetAction() func() {
	return b.action
}

// Button is a convenience function for creating button views.
func Button(label string, action func()) view.View {
	return NewButton(label, action)
}
