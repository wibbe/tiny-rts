package win32

import (
	"syscall"
	"unsafe"
)

type (
	BOOL int32
)

type RECT struct {
	Left, Top, Right, Bottom int32
}

type WNDCLASSEX struct {
	Size       uint32
	Style      uint32
	WndProc    uintptr
	ClsExtra   int32
	WndExtra   int32
	Instance   syscall.Handle
	Icon       syscall.Handle
	Cursor     syscall.Handle
	Background syscall.Handle
	MenuName   *uint16
	ClassName  *uint16
	IconSm     syscall.Handle
}

type POINT struct {
	X int32
	Y int32
}

type MSG struct {
	Hwnd    syscall.Handle
	Message uint32
	WParam  uint32
	LParam  uint32
	Time    uint32
	Point   POINT
}

const (
	SM_CXSCREEN int32 = 0
	SM_CYSCREEN int32 = 1

	TRUE  = 1
	FALSE = 0

	WS_CAPTION     uint32 = 0x00C00000
	WS_SYSMENU     uint32 = 0x00080000
	WS_MINIMIZEBOX uint32 = 0x00020000

	SW_HIDE int32 = 0
	SW_SHOW int32 = 5

	PM_REMOVE uint32 = 0x0001

	CS_VREDRAW uint32 = 0x0001
	CS_HREDRAW uint32 = 0x0002
	CS_OWNDC   uint32 = 0x0020

	IDI_APPLICATION = 32512

	BLACK_BRUSH = 4

	WM_DESTROY uint32 = 0x0002
	WM_CLOSE   uint32 = 0x0010
)

var (
	// user32.dll
	modUser32            = syscall.NewLazyDLL("user32.dll")
	procGetSystemMetrics = modUser32.NewProc("GetSystemMetrics")
	procAdjustWindowRect = modUser32.NewProc("AdjustWindowRect")
	procLoadIconW        = modUser32.NewProc("LoadIconW")
	procLoadCursorW      = modUser32.NewProc("LoadCursorW")
	procRegisterClassExW = modUser32.NewProc("RegisterClassExW")
	procDefWindowProcW   = modUser32.NewProc("DefWindowProcW")
	procCreateWindowExW  = modUser32.NewProc("CreateWindowExW")
	procDestroyWindow    = modUser32.NewProc("DestroyWindow")
	procPeekMessage      = modUser32.NewProc("PeekMessageW")
	procTranslateMessage = modUser32.NewProc("TranslateMessage")
	procDispatchMessage  = modUser32.NewProc("DispatchMessageW")
	procShowWindow       = modUser32.NewProc("ShowWindow")
	procPostQuitMessage  = modUser32.NewProc("PostQuitMessage")

	// gdi32.dll
	modGdi32           = syscall.NewLazyDLL("gdi32.dll")
	procGetStockObject = modGdi32.NewProc("GetStockObject")

	// kernel32.dll
	modKernel32          = syscall.NewLazyDLL("kernel32.dll")
	procGetModuleHandleW = modKernel32.NewProc("GetModuleHandleW")
)

// Library: user32.dll
func GetSystemMetrics(index int32) int {
	ret, _, _ := procGetSystemMetrics.Call(uintptr(index))
	return int(ret)
}

func AdjustWindowRect(rect *RECT, style uint32, menu bool) bool {
	ret, _, _ := procAdjustWindowRect.Call(uintptr(unsafe.Pointer(rect)), uintptr(style), uintptr(ToBool(menu)))
	return ret != 0
}

func LoadIcon(instance syscall.Handle, iconName *uint16) (syscall.Handle, error) {
	ret, _, err := procLoadIconW.Call(uintptr(instance), uintptr(unsafe.Pointer(iconName)))
	if ret == 0 {
		return 0, err
	}
	return syscall.Handle(ret), nil
}

func LoadCursor(instance syscall.Handle, cursorName *uint16) (syscall.Handle, error) {
	ret, _, err := procLoadCursorW.Call(uintptr(instance), uintptr(unsafe.Pointer(cursorName)))
	if ret == 0 {
		return 0, err
	}
	return syscall.Handle(ret), nil
}

func RegisterClass(class *WNDCLASSEX) (uint16, error) {
	ret, _, err := procRegisterClassExW.Call(uintptr(unsafe.Pointer(class)))
	if ret == 0 {
		return 0, err
	}
	return uint16(ret), nil
}

func DefWindowProc(hwnd syscall.Handle, msg uint32, wparam, lparam uintptr) uintptr {
	ret, _, _ := procDefWindowProcW.Call(uintptr(hwnd), uintptr(msg), wparam, lparam)
	return ret
}

func CreateWindow(className, windowName string, style uint32, x, y, width, height int32, parent, menu, instance syscall.Handle) (syscall.Handle, error) {
	ret, _, err := procCreateWindowExW.Call(
		uintptr(0),
		uintptr(unsafe.Pointer(syscall.StringToUTF16Ptr(className))),
		uintptr(unsafe.Pointer(syscall.StringToUTF16Ptr(windowName))),
		uintptr(style),
		uintptr(x),
		uintptr(y),
		uintptr(width),
		uintptr(height),
		uintptr(parent),
		uintptr(menu),
		uintptr(instance),
		uintptr(0))

	if ret == 0 {
		return 0, err
	}

	return syscall.Handle(ret), nil
}

func DestroyWindow(window syscall.Handle) error {
	ret, _, err := procDestroyWindow.Call(uintptr(window))
	if ret == 0 {
		return err
	}
	return nil
}

func PeekMessage(msg *MSG, hwnd syscall.Handle, msgFilterMin, msgFilterMax, removeMsg uint32) bool {
	ret, _, _ := procPeekMessage.Call(uintptr(unsafe.Pointer(msg)), uintptr(hwnd), uintptr(msgFilterMin), uintptr(msgFilterMax), uintptr(removeMsg))
	return ret != FALSE
}

func TranslateMessage(msg *MSG) {
	procTranslateMessage.Call(uintptr(unsafe.Pointer(msg)))
}

func DispatchMessage(msg *MSG) {
	procDispatchMessage.Call(uintptr(unsafe.Pointer(msg)))
}

func ShowWindow(hwnd syscall.Handle, cmdShow int32) {
	procShowWindow.Call(uintptr(hwnd), uintptr(cmdShow))
}

func PostQuitMessage(exitCode int32) {
	procPostQuitMessage.Call(uintptr(exitCode))
}

// Library: gdi32.dll
func GetStockObject(object int32) (syscall.Handle, error) {
	ret, _, err := procGetStockObject.Call(uintptr(object))
	if ret == 0 {
		return 0, err
	}
	return syscall.Handle(ret), nil
}

// Library: kernel32.dll
func GetModuleHandle() (syscall.Handle, error) {
	ret, _, err := procGetModuleHandleW.Call(uintptr(0))
	if ret == 0 {
		return 0, err
	}

	return syscall.Handle(ret), nil
}

// Helper functions
func ToBool(v bool) BOOL {
	if v {
		return TRUE
	}
	return FALSE
}

func ToIntResource(id uint16) *uint16 {
	return (*uint16)(unsafe.Pointer(uintptr(id)))
}

func ToLoWord(value uint32) uint16 {
	return uint16(value)
}

func ToHiWord(value uint32) uint16 {
	return uint16((value >> 16) & 0xffff)
}
