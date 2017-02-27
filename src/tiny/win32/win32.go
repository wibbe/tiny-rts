package win32

import (
	"syscall"
	"unsafe"
)

type (
	BOOL      int32
	HANDLE    uintptr
	HWND      HANDLE
	HINSTANCE HANDLE
	HICON     HANDLE
	HCURSOR   HANDLE
	HBRUSH    HANDLE
	HGDIOBJ   HANDLE
)

type RECT struct {
	Left, Top, Right, Bottom int32
}

type WNDCLASSEX struct {
	Size       uint
	Style      uint
	WndProc    uintptr
	ClsExtra   int32
	WndExtra   int32
	Instance   HINSTANCE
	Icon       HICON
	Cursor     HCURSOR
	Background HBRUSH
	MenuName   *uint16
	ClassName  *uint16
	IconSm     HICON
}

const (
	SM_CXSCREEN = 0
	SM_CYSCREEN = 1

	TRUE  = 1
	FALSE = 0

	WS_CAPTION     uint = 0x00C00000
	WS_SYSMENU     uint = 0x00080000
	WS_MINIMIZEBOX uint = 0x00020000

	CS_VREDRAW uint = 0x0001
	CS_HREDRAW uint = 0x0002
	CS_OWNDC   uint = 0x0020

	IDI_APPLICATION = 32512

	BLACK_BRUSH = 4
)

var (
	// user32.dll
	modUser32            = syscall.NewLazyDLL("user32.dll")
	procGetSystemMetrics = modUser32.NewProc("GetSystemMetrics")
	procAdjustWindowRect = modUser32.NewProc("AdjustWindowRect")
	procLoadIcon         = modUser32.NewProc("LoadIconW")
	procLoadCursor       = modUser32.NewProc("LoadCursorW")
	procRegisterClassEx  = modUser32.NewProc("RegisterClassExW")
	procDefWindowProc    = modUser32.NewProc("DefWindowProcW")

	// gdi32.dll
	modGdi32           = syscall.NewLazyDLL("gdi32.dll")
	procGetStockObject = modGdi32.NewProc("GetStockObject")

	// kernel32.dll
	modKernel32         = syscall.NewLazyDLL("kernel32.dll")
	procGetModuleHandle = modKernel32.NewProc("GetModuleHandleW")
)

// Library: user32.dll
func GetSystemMetrics(index int) int {
	ret, _, _ := procGetSystemMetrics.Call(uintptr(index))
	return int(ret)
}

func AdjustWindowRect(rect *RECT, style uint, menu bool) bool {
	ret, _, _ := procAdjustWindowRect.Call(uintptr(unsafe.Pointer(rect)), uintptr(style), uintptr(ToBool(menu)))
	return ret != 0
}

func LoadIcon(instance HINSTANCE, iconName *uint16) HICON {
	ret, _, _ := procLoadIcon.Call(uintptr(instance), uintptr(unsafe.Pointer(iconName)))
	return HICON(ret)
}

func LoadCursor(instance HINSTANCE, cursorName *uint16) HCURSOR {
	ret, _, _ := procLoadCursor.Call(uintptr(instance), uintptr(unsafe.Pointer(cursorName)))
	return HCURSOR(ret)
}

func RegisterClassEx(class *WNDCLASSEX) bool {
	ret, _, _ := procRegisterClassEx.Call(uintptr(unsafe.Pointer(class)))
	return ret == 0
}

func DefWindowProc(hwnd HWND, msg uint, wparam, lparam uintptr) uintptr {
	ret, _, _ := procDefWindowProc.Call(uintptr(hwnd), uintptr(msg), wparam, lparam)
	return ret
}

// Library: gdi32.dll
func GetStockObject(object int) HGDIOBJ {
	ret, _, _ := procGetStockObject.Call(uintptr(object))
	return HGDIOBJ(ret)
}

// Library: kernel32.dll
func GetModuleHandle(moduleName string) HINSTANCE {
	var name uintptr
	if moduleName == "" {
		name = 0
	} else {
		name = uintptr(unsafe.Pointer(syscall.StringToUTF16Ptr(moduleName)))
	}
	ret, _, _ := procGetModuleHandle.Call(name)
	return HINSTANCE(ret)
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
