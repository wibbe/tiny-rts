pub use self::platform::*;

#[cfg(target_os = "windows")]
#[path="win32_platform.rs"]
mod platform;

#[cfg(any(target_os = "linux", target_os = "macos"))]
#[path="glutin_platform.rs"]
mod platform;
