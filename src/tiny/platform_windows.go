package main

import "tiny/win32"

type Window struct {
	hwnd win32.HWND
}

func NewWindow(width, height, scale int, title string) *Window {

	screenWidth := win32.GetSystemMetrics(win32.SM_CXSCREEN)
	screenHeight := win32.GetSystemMetrics(win32.SM_CYSCREEN)

	windowWidth := width * scale
	windowHeight := height * scale
	windowLeft := (screenWidth - windowWidth) / 2
	windowTop := (screenHeight - windowHeight) / 2

	rect := win32.RECT{
		Left:   int32(windowLeft),
		Right:  int32(windowLeft + windowWidth),
		Top:    int32(windowTop),
		Bottom: int32(windowTop + windowHeight),
	}

	style := win32.WS_CAPTION | win32.WS_SYSMENU | win32.WS_MINIMIZEBOX
	win32.AdjustWindowRect(&rect, style, false)

	return &Window{}
}

func (w *Window) IsRunning() bool {
	return false
}

func (w *Window) Step() bool {
	return true
}
