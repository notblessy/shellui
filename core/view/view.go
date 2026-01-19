package view

// View is the fundamental protocol that all UI elements conform to.
// Everything visible in shellui is a View.
type View interface {
	// Body returns the view's content. For primitive views, this may return nil.
	// For container views, this returns the composed child views.
	Body() View
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
