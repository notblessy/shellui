package view

// Convenience functions for creating views in a SwiftUI-like style.
// These provide a more ergonomic API similar to SwiftUI's ViewBuilder.

// VStack creates a vertical stack of views.
func VStack(children ...View) View {
	return NewVStack(children...)
}

// HStack creates a horizontal stack of views.
func HStack(children ...View) View {
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
