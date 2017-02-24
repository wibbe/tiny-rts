package win32

type (
	BOOL   int32
	HANDLE uintptr
	HWND   HANDLE
)

type RECT struct {
	Left, Top, Right, Bottom int32
}
