package win32

import (
	"syscall"
	"unsafe"
)

var (
	modUser32 = syscall.NewLazyDLL("user32.dll")

	procGetSystemMetrics = modUser32.NewProc("GetSystemMetrics")
	procAdjustWindowRect = modUser32.NewProc("AdjustWindowRect")
)

func GetSystemMetrics(index int) int {
	ret, _, _ := procGetSystemMetrics.Call(uintptr(index))
	return int(ret)
}

func AdjustWindowRect(rect *RECT, style uint, menu bool) bool {
	ret, _, _ := procAdjustWindowRect.Call(uintptr(unsafe.Pointer(rect)), uintptr(style), uintptr(ToBool(menu)))
	return ret != 0
}
