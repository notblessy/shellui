// SPDX-License-Identifier: Unlicense OR MIT

// Package opentype implements text layout and shaping for OpenType
// files.
//
// NOTE: the OpenType specification allows for fonts to include bitmap images
// in a variety of formats. In the interest of small binary sizes, the opentype
// package only automatically imports the PNG image decoder. If you have a font
// with glyphs in JPEG or TIFF formats, register those decoders with the image
// package in order to ensure those glyphs are visible in text.
package opentype

import (
	"bytes"
	"fmt"
	_ "image/png"

	fontapi "github.com/go-text/typesetting/font"
	"github.com/go-text/typesetting/font/opentype"
)

// Face is a thread-safe representation of a loaded font. For efficiency, applications
// should construct a face for any given font file once, reusing it across different
// text shapers.
type Face struct {
	Face *fontapi.Face
	Font *fontapi.Font
}

// Parse constructs a Face from source bytes.
func Parse(src []byte) (Face, error) {
	ld, err := opentype.NewLoader(bytes.NewReader(src))
	if err != nil {
		return Face{}, err
	}
	fontObj, err := fontapi.NewFont(ld)
	if err != nil {
		return Face{}, fmt.Errorf("failed parsing truetype font: %w", err)
	}
	return Face{
		Face: &fontapi.Face{Font: fontObj},
		Font: fontObj,
	}, nil
}

// ParseCollection parse an Opentype font file, with support for collections.
// Single font files are supported, returning a slice with length 1.
func ParseCollection(src []byte) ([]Face, error) {
	lds, err := opentype.NewLoaders(bytes.NewReader(src))
	if err != nil {
		return nil, err
	}
	out := make([]Face, len(lds))
	for i, ld := range lds {
		fontObj, err := fontapi.NewFont(ld)
		if err != nil {
			return nil, fmt.Errorf("reading font %d of collection: %s", i, err)
		}
		out[i] = Face{
			Face: &fontapi.Face{Font: fontObj},
			Font: fontObj,
		}
	}

	return out, nil
}
