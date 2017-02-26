package main

import "tiny/win32"
import "unsafe"
import "syscall"

type Window struct {
	hwnd win32.HWND
}

func NewWindow(width, height, scale int, title string) *Window {
	registerClass(title + "Class")

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

func registerClass(className string) {
	var class win32.WNDCLASSEX

	class.Size = uint(unsafe.Sizeof(class))
	class.Style = win32.CS_HREDRAW | win32.CS_VREDRAW | win32.CS_OWNDC
	class.WndProc = syscall.NewCallback(wndProc)
	class.ClsExtra = 0
	class.WndExtra = 0
	class.Instance = win32.GetModuleHandle("")
	class.Icon = win32.LoadIcon(0, win32.ToIntResource(win32.IDI_APPLICATION))
	class.Cursor = win32.LoadCursor(0, win32.ToIntResource(win32.IDI_APPLICATION))
	class.Background = win32.HBRUSH(win32.GetStockObject(win32.BLACK_BRUSH))
	class.MenuName = nil
	class.ClassName = syscall.StringToUTF16Ptr(className)
	class.IconSm = 0

	if !win32.RegisterClassEx(&class) {
		panic(syscall.GetLastError())
	}
}

func wndProc(hwnd win32.HWND, msg uint, wparam, lparam uintptr) uintptr {
	return win32.DefWindowProc(hwnd, msg, wparam, lparam)
}
