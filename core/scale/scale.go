package scale

import (
	"math"
)

// CanvasScaler is an interface for objects that provide scale information.
// This allows scale functions to work with Canvas without creating a circular dependency.
type CanvasScaler interface {
	Scale() float32
}

// ToScreenPixels converts a logical (device-independent) coordinate to physical screen pixels.
// logicalValue: size in logical coordinates (e.g., 200px stays 200px)
// scale: the combined scale factor (canvasScale * texScale)
func ToScreenPixels(logicalValue float32, scale float32) int {
	return int(math.Ceil(float64(logicalValue * scale)))
}

// ToLogicalPixels converts physical screen pixels to logical (device-independent) coordinates.
// physicalValue: size in physical screen pixels
// scale: the combined scale factor (canvasScale * texScale)
func ToLogicalPixels(physicalValue int, scale float32) float32 {
	if scale == 0 {
		panic("scale cannot be zero")
	}
	if scale == 1.0 {
		return float32(physicalValue)
	}
	return float32(physicalValue) / scale
}

// ToLogicalSize converts physical screen dimensions to logical coordinates.
func ToLogicalSize(physicalWidth, physicalHeight int, scale float32) (float32, float32) {
	return ToLogicalPixels(physicalWidth, scale), ToLogicalPixels(physicalHeight, scale)
}

// ToScreenSize converts logical dimensions to physical screen pixels.
func ToScreenSize(logicalWidth, logicalHeight float32, scale float32) (int, int) {
	return ToScreenPixels(logicalWidth, scale), ToScreenPixels(logicalHeight, scale)
}

// RoundToPixel rounds a value to the nearest pixel boundary at the given scale.
// This prevents blurry rendering by aligning to pixel boundaries.
func RoundToPixel(v float32, pixScale float32) float32 {
	if pixScale == 1.0 {
		return float32(math.Round(float64(v)))
	}
	return float32(math.Round(float64(v*pixScale))) / pixScale
}

// ScaleType holds the scale factors for a canvas/renderer.
type ScaleType struct {
	// CanvasScale is the user/system scale factor (e.g., 1.0, 1.25, 1.5, 2.0)
	CanvasScale float32
	// TexScale is the texture scale (framebuffer pixels / window pixels) for HiDPI
	TexScale float32
}

// NewScale creates a new ScaleType with default values.
func NewScale() *ScaleType {
	return &ScaleType{
		CanvasScale: 1.0,
		TexScale:    1.0,
	}
}

// PixScale returns the combined pixel scale (canvasScale * texScale).
// This is the factor to multiply logical coordinates by to get physical pixels.
func (s *ScaleType) PixScale() float32 {
	return s.CanvasScale * s.TexScale
}

// SetTexScale updates the texture scale from framebuffer and window sizes.
// This is called when the framebuffer size changes (e.g., moving to a HiDPI display).
func (s *ScaleType) SetTexScale(framebufferWidth, windowWidth int) {
	if windowWidth > 0 {
		s.TexScale = float32(framebufferWidth) / float32(windowWidth)
	} else {
		s.TexScale = 1.0
	}
	if s.TexScale < 1.0 {
		s.TexScale = 1.0
	}
}

// ToScreenCoordinate converts a logical canvas coordinate to a screen coordinate.
// c: the canvas providing scale information
// v: the logical coordinate value
func ToScreenCoordinate(c CanvasScaler, v float32) int {
	return int(math.Ceil(float64(v * c.Scale())))
}

// ToCanvasCoordinate converts a screen coordinate to a logical canvas coordinate.
// c: the canvas providing scale information
// v: the screen coordinate value (in pixels)
func ToCanvasCoordinate(c CanvasScaler, v int) float32 {
	scale := c.Scale()
	if scale == 0.0 {
		panic("Incorrect scale most likely not set.")
	}
	if scale == 1.0 {
		return float32(v)
	}
	return float32(v) / scale
}
