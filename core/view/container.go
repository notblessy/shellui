package view

// VStackType arranges views vertically.
// Default: height 100%, width auto (natural)
type VStackType struct {
	ViewBaseType
	width  float32 // -1 means auto (natural), >= 0 means fixed width
	height float32 // -1 means auto (natural), >= 0 means fixed height
}

// NewVStack creates a new VStackType with the given children.
func NewVStack(children ...View) *VStackType {
	vs := &VStackType{
		width:  -1, // auto (natural width)
		height: -1, // auto, but default behavior is 100% of available
	}
	vs.SetChildren(children)
	return vs
}

// Width sets the width of the VStack (SwiftUI-style modifier).
// If width < 0, uses natural/auto width.
func (vs *VStackType) Width(width float32) *VStackType {
	vs.width = width
	return vs
}

// Height sets the height of the VStack (SwiftUI-style modifier).
// If height < 0, uses natural/auto height (defaults to 100% of available).
func (vs *VStackType) Height(height float32) *VStackType {
	vs.height = height
	return vs
}

// GetWidth returns the width setting (-1 for auto).
func (vs *VStackType) GetWidth() float32 {
	return vs.width
}

// GetHeight returns the height setting (-1 for auto).
func (vs *VStackType) GetHeight() float32 {
	return vs.height
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

// MinSize returns the minimum size of the VStack.
// If width is fixed (>= 0), uses that. Otherwise calculates from children.
// Height is sum of children's heights.
func (vs *VStackType) MinSize() Size {
	children := vs.GetChildren()
	if len(children) == 0 {
		return Size{Width: 0, Height: 0}
	}

	var maxWidth float32 = 0
	var totalHeight float32 = 0

	for _, child := range children {
		if child != nil {
			childSize := child.MinSize()
			if childSize.Width > maxWidth {
				maxWidth = childSize.Width
			}
			totalHeight += childSize.Height
		}
	}

	// Use fixed width if specified, otherwise use calculated maxWidth
	width := maxWidth
	if vs.width >= 0 {
		width = vs.width
	}

	// Use fixed height if specified, otherwise use calculated totalHeight
	height := totalHeight
	if vs.height >= 0 {
		height = vs.height
	}

	return Size{Width: width, Height: height}
}

// HStackType arranges views horizontally.
// Default: width 100%, height auto (natural)
type HStackType struct {
	ViewBaseType
	width  float32 // -1 means auto (natural), >= 0 means fixed width
	height float32 // -1 means auto (natural), >= 0 means fixed height
}

// NewHStack creates a new HStackType with the given children.
func NewHStack(children ...View) *HStackType {
	hs := &HStackType{
		width:  -1, // auto, but default behavior is 100% of available
		height: -1, // auto (natural height)
	}
	hs.SetChildren(children)
	return hs
}

// Width sets the width of the HStack (SwiftUI-style modifier).
// If width < 0, uses natural/auto width (defaults to 100% of available).
func (hs *HStackType) Width(width float32) *HStackType {
	hs.width = width
	return hs
}

// Height sets the height of the HStack (SwiftUI-style modifier).
// If height < 0, uses natural/auto height.
func (hs *HStackType) Height(height float32) *HStackType {
	hs.height = height
	return hs
}

// GetWidth returns the width setting (-1 for auto).
func (hs *HStackType) GetWidth() float32 {
	return hs.width
}

// GetHeight returns the height setting (-1 for auto).
func (hs *HStackType) GetHeight() float32 {
	return hs.height
}

// Body returns the children of this HStackType.
func (hs *HStackType) Body() View {
	children := hs.GetChildren()
	if len(children) == 0 {
		return nil
	}
	return children[0]
}

// MinSize returns the minimum size of the HStack.
// If width/height are fixed (>= 0), uses those. Otherwise calculates from children.
func (hs *HStackType) MinSize() Size {
	children := hs.GetChildren()
	if len(children) == 0 {
		return Size{Width: 0, Height: 0}
	}

	var totalWidth float32 = 0
	var maxHeight float32 = 0

	for _, child := range children {
		if child != nil {
			childSize := child.MinSize()
			totalWidth += childSize.Width
			if childSize.Height > maxHeight {
				maxHeight = childSize.Height
			}
		}
	}

	// Use fixed width if specified, otherwise use calculated totalWidth
	width := totalWidth
	if hs.width >= 0 {
		width = hs.width
	}

	// Use fixed height if specified, otherwise use calculated maxHeight
	height := maxHeight
	if hs.height >= 0 {
		height = hs.height
	}

	return Size{Width: width, Height: height}
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

// MinSize returns the minimum size of the ZStack.
// For ZStack, this is the size of the largest child (since they overlap).
func (zs *ZStackType) MinSize() Size {
	children := zs.GetChildren()
	if len(children) == 0 {
		return Size{Width: 0, Height: 0}
	}

	var maxWidth float32 = 0
	var maxHeight float32 = 0

	for _, child := range children {
		if child != nil {
			childSize := child.MinSize()
			if childSize.Width > maxWidth {
				maxWidth = childSize.Width
			}
			if childSize.Height > maxHeight {
				maxHeight = childSize.Height
			}
		}
	}

	return Size{Width: maxWidth, Height: maxHeight}
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

// MinSize returns (0, 0) for SpacerType since it's flexible and fills available space.
func (s *SpacerType) MinSize() Size {
	return Size{Width: 0, Height: 0}
}
