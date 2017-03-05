package platform

type PlatformWindow interface {
	Show()
	Step() bool
	GetWidth() int
	GetHeight() int
}
