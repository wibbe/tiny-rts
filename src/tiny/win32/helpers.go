package win32

func ToBool(v bool) BOOL {
	if v {
		return TRUE
	}
	return FALSE
}
