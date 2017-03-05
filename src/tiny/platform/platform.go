package platform

import (
	"image"
)

type PlatformWindow interface {
	Show()
	Step() bool
	GetWidth() int
	GetHeight() int

	Paint(img *image.RGBA) error
}
