package font

import (
	otfontapi "github.com/go-text/typesetting/font"
	otopentype "github.com/notblessy/shellui/core/font/opentype"
)

// ParseRobotoFont parses a Roboto font from bytes
func ParseRobotoFont(data []byte) (*otfontapi.Font, error) {
	face, err := otopentype.Parse(data)
	if err != nil {
		return nil, err
	}
	return face.Font, nil
}

// GetFontFace returns the font face from a robotoFace
func GetFontFace(face Face) (*otfontapi.Font, bool) {
	otFace := face.Face()
	if otFace == nil {
		return nil, false
	}
	return otFace.Font, true
}
