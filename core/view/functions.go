package view

// Convenience functions for creating views in a SwiftUI-like style.
// These provide a more ergonomic API similar to SwiftUI's ViewBuilder.

// VStack creates a vertical stack of views.
// Returns *VStackType to allow method chaining.
func VStack(children ...View) *VStackType {
	return NewVStack(children...)
}

// HStack creates a horizontal stack of views.
// Returns *HStackType to allow method chaining.
func HStack(children ...View) *HStackType {
	return NewHStack(children...)
}

// ZStack creates a z-ordered stack of views.
func ZStack(children ...View) View {
	return NewZStack(children...)
}

// Spacer creates a spacer view.
func Spacer() View {
	return NewSpacer()
}
