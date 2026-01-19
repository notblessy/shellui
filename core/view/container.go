package view

// VStackType arranges views vertically.
type VStackType struct {
	ViewBaseType
}

// NewVStack creates a new VStackType with the given children.
func NewVStack(children ...View) *VStackType {
	vs := &VStackType{}
	vs.SetChildren(children)
	return vs
}

// Body returns the children of this VStackType.
func (vs *VStackType) Body() View {
	children := vs.GetChildren()
	if len(children) == 0 {
		return nil
	}
	// Return first child as representative, or a container view
	// In practice, the layout engine will handle all children
	return children[0]
}

// HStackType arranges views horizontally.
type HStackType struct {
	ViewBaseType
}

// NewHStack creates a new HStackType with the given children.
func NewHStack(children ...View) *HStackType {
	hs := &HStackType{}
	hs.SetChildren(children)
	return hs
}

// Body returns the children of this HStackType.
func (hs *HStackType) Body() View {
	children := hs.GetChildren()
	if len(children) == 0 {
		return nil
	}
	return children[0]
}

// ZStackType arranges views in layers (stacked on top of each other).
type ZStackType struct {
	ViewBaseType
}

// NewZStack creates a new ZStackType with the given children.
func NewZStack(children ...View) *ZStackType {
	zs := &ZStackType{}
	zs.SetChildren(children)
	return zs
}

// Body returns the children of this ZStackType.
func (zs *ZStackType) Body() View {
	children := zs.GetChildren()
	if len(children) == 0 {
		return nil
	}
	return children[0]
}

// SpacerType is a view that takes up available space.
type SpacerType struct {
	ViewBaseType
}

// NewSpacer creates a new SpacerType.
func NewSpacer() *SpacerType {
	return &SpacerType{}
}

// Body returns nil for SpacerType.
func (s *SpacerType) Body() View {
	return nil
}
