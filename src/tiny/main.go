package main

import (
	"tiny/platform"
)

func main() {

	win := platform.NewWindow(320, 200, 3, "Tiny RTS")
	win.Show()

	for win.IsRunning() {

	}
}
