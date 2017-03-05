package main

import (
	"fmt"
	"image"
	"image/color"
	"image/draw"
	"time"
	"tiny/platform"
)

func main() {

	win, err := platform.NewWindow(320, 200, 3, "Tiny RTS")
	if err != nil {
		fmt.Println(err)
		return
	}

	win.Show()

	img := image.NewRGBA(image.Rect(0, 0, 320, 200))
	draw.Draw(img, img.Bounds(), &image.Uniform{color.RGBA{0, 255, 0, 0}}, image.ZP, draw.Src)

	for win.Step() {
		err := win.Paint(img)
		if err != nil {
			fmt.Println(err)
			break
		}

		time.Sleep(10 * time.Millisecond)
	}
}
