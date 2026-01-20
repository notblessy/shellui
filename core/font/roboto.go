package font

import (
	"fmt"
	"os"
	"path/filepath"
	"sync"

	otfontapi "github.com/go-text/typesetting/font"
	otopentype "github.com/notblessy/shellui/core/font/opentype"
)

// Note: embed doesn't support wildcards in subdirectories
// We'll load fonts from filesystem instead

var (
	robotoOnce    sync.Once
	robotoFaces   []FontFace
	robotoFaceMap map[string]FontFace // key: "Roboto-Regular", "Roboto-Bold", etc.
)

// LoadRoboto loads all Roboto fonts from the assets directory
func LoadRoboto() ([]FontFace, error) {
	var err error
	robotoOnce.Do(func() {
		robotoFaces, robotoFaceMap, err = loadRobotoFonts()
	})
	return robotoFaces, err
}

// GetRobotoFace returns a specific Roboto font face by name
// Examples: "Roboto-Regular", "Roboto-Bold", "Roboto-Italic", "Roboto-BoldItalic"
func GetRobotoFace(name string) (FontFace, bool) {
	LoadRoboto()
	face, ok := robotoFaceMap[name]
	return face, ok
}

// GetRobotoFaceByStyle returns a Roboto font face matching the style and weight
func GetRobotoFaceByStyle(style Style, weight Weight) (FontFace, bool) {
	LoadRoboto()

	// Build font name
	var name string
	switch weight {
	case Thin:
		name = "Roboto-Thin"
	case ExtraLight:
		name = "Roboto-ExtraLight"
	case Light:
		name = "Roboto-Light"
	case Normal:
		name = "Roboto-Regular"
	case Medium:
		name = "Roboto-Medium"
	case SemiBold:
		name = "Roboto-SemiBold"
	case Bold:
		name = "Roboto-Bold"
	case ExtraBold:
		name = "Roboto-ExtraBold"
	case Black:
		name = "Roboto-Black"
	default:
		name = "Roboto-Regular"
	}

	if style == Italic {
		name += "Italic"
	}

	return GetRobotoFace(name)
}

func loadRobotoFonts() ([]FontFace, map[string]FontFace, error) {
	faces := make([]FontFace, 0)
	faceMap := make(map[string]FontFace)

	// Load from filesystem
	assetsPath := "assets/Roboto/static"
	entries, err := os.ReadDir(assetsPath)
	if err == nil {
		for _, entry := range entries {
			if !entry.IsDir() && filepath.Ext(entry.Name()) == ".ttf" {
				data, err := os.ReadFile(filepath.Join(assetsPath, entry.Name()))
				if err == nil {
					face, fontName := parseRobotoFont(data, entry.Name())
					if face.Face != nil {
						faces = append(faces, face)
						faceMap[fontName] = face
					}
				}
			}
		}
	}

	if len(faces) == 0 {
		return nil, nil, fmt.Errorf("no Roboto fonts found")
	}

	return faces, faceMap, nil
}

func parseRobotoFont(data []byte, filename string) (FontFace, string) {
	// Extract font name from filename
	// e.g., "Roboto-Regular.ttf" -> "Roboto-Regular"
	fontName := filename[:len(filename)-4] // Remove .ttf

	// Parse font name to extract style and weight
	var gioFont Font
	var style Style = Regular
	var weight Weight = Normal

	// Parse weight from filename
	if contains(filename, "Black") {
		weight = Black
	} else if contains(filename, "ExtraBold") {
		weight = ExtraBold
	} else if contains(filename, "Bold") {
		weight = Bold
	} else if contains(filename, "SemiBold") {
		weight = SemiBold
	} else if contains(filename, "Medium") {
		weight = Medium
	} else if contains(filename, "Light") {
		weight = Light
	} else if contains(filename, "ExtraLight") {
		weight = ExtraLight
	} else if contains(filename, "Thin") {
		weight = Thin
	}

	// Parse style
	if contains(filename, "Italic") {
		style = Italic
	}

	gioFont = Font{
		Typeface: "Roboto",
		Style:    style,
		Weight:   weight,
	}

	// Parse the font using opentype package
	otFace, err := otopentype.Parse(data)
	if err != nil {
		// If parsing fails, return a face that stores the data
		face := robotoFace{
			fontData: data,
			fontMeta: gioFont,
			fontName: fontName,
		}
		return FontFace{
			Font: gioFont,
			Face: face,
		}, fontName
	}
	
	// Convert opentype.Face to our Face interface
	// Create a wrapper that implements our Face interface
	wrapper := &opentypeFaceWrapper{face: otFace}
	return FontFace{
		Font: gioFont,
		Face: wrapper,
	}, fontName
}

// opentypeFaceWrapper wraps opentype.Face to implement font.Face interface
type opentypeFaceWrapper struct {
	face otopentype.Face
}

func (w *opentypeFaceWrapper) Face() *otfontapi.Face {
	return w.face.Face
}

type robotoFace struct {
	fontData []byte
	fontMeta Font
	fontName string
}

func (f robotoFace) Face() *otfontapi.Face {
	// Parse on demand
	otFace, err := otopentype.Parse(f.fontData)
	if err != nil {
		return nil
	}
	return otFace.Face
}

func contains(s, substr string) bool {
	return len(s) >= len(substr) && (s == substr ||
		(len(s) > len(substr) && (s[:len(substr)] == substr ||
			s[len(s)-len(substr):] == substr ||
			containsMiddle(s, substr))))
}

func containsMiddle(s, substr string) bool {
	for i := 0; i <= len(s)-len(substr); i++ {
		if s[i:i+len(substr)] == substr {
			return true
		}
	}
	return false
}
