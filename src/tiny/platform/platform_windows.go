// +build windows

package platform

import (
	"image"
	"runtime"
	"syscall"
	"tiny/win32"
	"unsafe"
)

type Window struct {
	handle   syscall.Handle
	running  bool
	width    int
	height   int
	bmi      win32.BITMAPINFO
	canvas   []uint8
	keyState []bool
	keyDelta []bool
}

func init() {
	runtime.LockOSThread()
}

func NewWindow(options ...option) (PlatformWindow, error) {
	cfg := getConfig(options)

	instance, err := win32.GetModuleHandle()
	if err != nil {
		return nil, err
	}

	window := &Window{
		handle:  0,
		running: true,
		width:   cfg.width * cfg.scale,
		height:  cfg.height * cfg.scale,
		bmi: win32.BITMAPINFO{
			Header: win32.BITMAPINFOHEADER{
				Width:         int32(cfg.width),
				Height:        int32(cfg.height),
				Planes:        1,
				BitCount:      32,
				Compression:   win32.BI_RGB,
				SizeImage:     0,
				XPelsPerMeter: 0,
				YPelsPerMeter: 0,
				ClrUsed:       0,
				ClrImportant:  0,
			},
			Colors: nil,
		},
		canvas:   make([]uint8, cfg.width*cfg.height*4),
		keyState: make([]bool, 256),
		keyDelta: make([]bool, 256),
	}

	window.bmi.Header.Size = uint32(unsafe.Sizeof(window.bmi.Header))

	wndProc := func(hwnd syscall.Handle, msg uint32, wparam, lparam uintptr) uintptr {
		switch msg {
		case win32.WM_CLOSE:
			win32.DestroyWindow(hwnd)

		case win32.WM_DESTROY:
			win32.PostQuitMessage(0)
			window.running = false

		default:
			return win32.DefWindowProc(hwnd, msg, wparam, lparam)
		}
		return 0
	}

	err = registerClass("TinyRTSClass", instance, syscall.NewCallback(wndProc))
	if err != nil {
		return nil, err
	}

	screenWidth := win32.GetSystemMetrics(win32.SM_CXSCREEN)
	screenHeight := win32.GetSystemMetrics(win32.SM_CYSCREEN)

	windowLeft := (screenWidth - window.width) / 2
	windowTop := (screenHeight - window.height) / 2

	rect := win32.RECT{
		Left:   int32(windowLeft),
		Right:  int32(windowLeft + window.width),
		Top:    int32(windowTop),
		Bottom: int32(windowTop + window.height),
	}

	style := win32.WS_CAPTION | win32.WS_SYSMENU | win32.WS_MINIMIZEBOX
	win32.AdjustWindowRect(&rect, style, false)

	handle, err := win32.CreateWindow(
		"TinyRTSClass",
		cfg.title,
		style,
		rect.Left,
		rect.Top,
		rect.Right-rect.Left,
		rect.Bottom-rect.Top,
		0,
		0,
		instance)

	if err != nil {
		return nil, err
	}

	window.handle = handle
	return window, nil
}

func (w *Window) Show() {
	win32.ShowWindow(w.handle, win32.SW_SHOW)
}

func (w *Window) GetWidth() int {
	return w.width
}

func (w *Window) GetHeight() int {
	return w.height
}

func (w *Window) Step() bool {
	msg := win32.MSG{}

	for i := 0; i < len(w.keyDelta); i++ {
		w.keyDelta[i] = false
	}

	for win32.PeekMessage(&msg, 0, 0, 0, win32.PM_REMOVE) {
		win32.TranslateMessage(&msg)
		win32.DispatchMessage(&msg)
	}

	return w.running
}

func (w *Window) Paint(img image.Image) error {
	dc, err := win32.GetDC(w.handle)
	if err != nil {
		return err
	}

	defer win32.ReleaseDC(w.handle, dc)

	bounds := img.Bounds()
	size := bounds.Size()

	if len(w.canvas) != size.X*size.Y {
		w.canvas = make([]uint8, size.X*size.Y*4)
	}

	// Copy image data to the canvas
	idx := 0
	for y := bounds.Min.Y; y < bounds.Max.Y; y++ {
		for x := bounds.Min.X; x < bounds.Max.X; x++ {
			r, g, b, _ := img.At(x, y).RGBA()
			w.canvas[idx+3] = 255
			w.canvas[idx+2] = uint8(r >> 8)
			w.canvas[idx+1] = uint8(g >> 8)
			w.canvas[idx+0] = uint8(b >> 8)

			idx += 4
		}
	}

	// Blit the canvas to the window
	err = win32.StretchDIBits(
		dc,
		0, 0,
		int32(w.width), int32(w.height),
		0, 0,
		int32(size.X), int32(size.Y),
		w.canvas,
		&w.bmi,
		win32.DIB_RGB_COLORS,
		win32.SRCCOPY)

	return err
}

func registerClass(className string, instance syscall.Handle, callback uintptr) error {
	class := win32.WNDCLASSEX{
		Style:     win32.CS_HREDRAW | win32.CS_VREDRAW | win32.CS_OWNDC,
		WndProc:   callback,
		Instance:  instance,
		ClassName: syscall.StringToUTF16Ptr(className),
	}

	if icon, err := win32.LoadIcon(0, win32.ToIntResource(win32.IDI_APPLICATION)); err == nil {
		class.Icon = icon
	} else {
		return err
	}

	if cursor, err := win32.LoadCursor(0, win32.ToIntResource(win32.IDI_APPLICATION)); err == nil {
		class.Cursor = cursor
	} else {
		return err
	}

	if brush, err := win32.GetStockObject(win32.BLACK_BRUSH); err == nil {
		class.Background = brush
	} else {
		return err
	}

	class.Size = uint32(unsafe.Sizeof(class))

	if _, err := win32.RegisterClass(&class); err != nil {
		return err
	}

	return nil
}
