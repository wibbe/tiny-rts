package main

import (
	"fmt"
	"tiny/platform"
)

func main() {

	win, err := platform.NewWindow(320, 200, 3, "Tiny RTS")
	if err != nil {
		fmt.Println(err)
		return
	}

	win.Show()

	for win.Step() {

	}
}
