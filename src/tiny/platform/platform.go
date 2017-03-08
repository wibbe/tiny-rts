package platform

import (
	"image"
)

type option func(*config)

type config struct {
	width  int
	height int
	scale  int
	title  string
}

func Width(w int) option {
	return func(cfg *config) {
		cfg.width = w
	}
}

func Height(h int) option {
	return func(cfg *config) {
		cfg.height = h
	}
}

func Scale(s int) option {
	if s == 0 {
		s = 1
	}
	return func(cfg *config) {
		cfg.scale = s
	}
}

func Title(t string) option {
	return func(cfg *config) {
		cfg.title = t
	}
}

type PlatformWindow interface {
	Show()
	Step() bool
	GetWidth() int
	GetHeight() int

	Paint(img image.Image) error
}

func getConfig(options []option) config {
	cfg := config{
		width:  640,
		height: 480,
		scale:  1,
		title:  "",
	}

	// Apply options
	for _, opt := range options {
		opt(&cfg)
	}
	return cfg
}
