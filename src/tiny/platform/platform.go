package platform

type PlatformWindow interface {
	IsRunning() bool
	Step() bool
}
