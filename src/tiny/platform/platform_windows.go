package platform

import "tiny/win32"
import "unsafe"
import "syscall"

type Window struct {
	handle  syscall.Handle
	running bool
}

func NewWindow(width, height, scale int, title string) (PlatformWindow, error) {
	instance, err := win32.GetModuleHandle()
	if err != nil {
		return nil, err
	}

	window := &Window{
		handle:  0,
		running: true,
	}

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

	handle, err := win32.CreateWindow(
		"TinyRTSClass",
		title,
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

func (w *Window) Step() bool {
	msg := win32.MSG{}

	for win32.PeekMessage(&msg, 0, 0, 0, win32.PM_REMOVE) {
		win32.TranslateMessage(&msg)
		win32.DispatchMessage(&msg)
	}

	return w.running
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