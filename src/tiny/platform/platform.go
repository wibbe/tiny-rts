package platform

type PlatformWindow interface {
	Show()
	Step() bool
}
