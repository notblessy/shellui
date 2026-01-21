package view

// Position represents a position in logical pixels.
type Position struct {
	X float32
	Y float32
}

// Size represents a size in logical pixels.
type Size struct {
	Width  float32
	Height float32
}

// Max returns a new Size that is the maximum of this size and the other size.
func (s Size) Max(other Size) Size {
	width := s.Width
	if other.Width > width {
		width = other.Width
	}
	height := s.Height
	if other.Height > height {
		height = other.Height
	}
	return Size{Width: width, Height: height}
}

// View is the fundamental protocol that all UI elements conform to.
// Everything visible in shellui is a View.
type View interface {
	// Body returns the view's content. For primitive views, this may return nil.
	// For container views, this returns the composed child views.
	Body() View
	
	// MinSize returns the minimum natural size of this view.
	// For text, this is the size needed to display the text without stretching.
	// For containers, this is calculated from children's MinSize.
	// Returns (0, 0) if size cannot be determined.
	MinSize() Size
}

// ViewBaseType provides a base implementation for views.
// Custom views can embed this to get default behavior.
type ViewBaseType struct {
	children []View
}

// Body returns nil by default. Views should override this.
func (vb *ViewBaseType) Body() View {
	return nil
}

// SetChildren sets the children of this view.
func (vb *ViewBaseType) SetChildren(children []View) {
	vb.children = children
}

// GetChildren returns the children of this view.
func (vb *ViewBaseType) GetChildren() []View {
	return vb.children
}

// MinSize returns (0, 0) by default. Views should override this.
func (vb *ViewBaseType) MinSize() Size {
	return Size{Width: 0, Height: 0}
}
