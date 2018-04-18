
#[windows_subsystem(windows)]

extern crate winapi;

use std::ptr;
use std::mem;
use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;

//use super::winapi;
use self::winapi::gdi32;
use self::user32;
use self::kernel32;

use super::super::*;

#[derive(Copy, Clone)]
enum Event {
    KeyDown(u8),
    KeyUp(u8),
}

static mut WIN_EVENT: Option<Event> = None;


pub struct Window {
   handle: winapi::HWND,
   window_bmi: winapi::BITMAPINFO,
   window_buffer: Vec<u32>,
   pub key_state: [bool; 256],
   pub key_delta: [bool; 256],
   canvas_width: u32,
   canvas_height: u32,
   window_width: u32,
   window_height: u32,
}

fn to_wstring(str: &str) -> Vec<u16> {
   OsStr::new(str).encode_wide().chain(Some(0).into_iter()).collect::<Vec<_>>()
}

impl Window {
   pub fn new(config: &Config) -> Result<Window, String> {
      unsafe {
         let class_name = match register_window_class() {
            Ok(class_name) => class_name,
            Err(err) => return Err(err),
         };

         let screen_width = user32::GetSystemMetrics(winapi::SM_CXSCREEN) as u32;
         let screen_height = user32::GetSystemMetrics(winapi::SM_CYSCREEN) as u32;

         let window_width = config.width * config.scale;
         let window_height = config.height * config.scale;
         let window_left = (screen_width - window_width) / 2;
         let window_top = (screen_height - window_height) / 2;

         let mut rc = winapi::RECT {
            left: window_left as winapi::LONG, right: (window_left + window_width) as winapi::LONG,
            top: window_top as winapi::LONG, bottom: (window_top + window_height) as winapi::LONG,
         };

         let style = winapi::WS_CAPTION | winapi::WS_SYSMENU | winapi::WS_MINIMIZEBOX;
         user32::AdjustWindowRect(&mut rc, style, winapi::FALSE);

         let handle = user32::CreateWindowExW(0,
                                              class_name.as_ptr(),
                                              to_wstring(config.title.as_str()).as_ptr() as winapi::LPCWSTR,
                                              style,
                                              rc.left, rc.top,
                                              rc.right - rc.left, rc.bottom - rc.top,
                                              ptr::null_mut(),
                                              ptr::null_mut(),
                                              kernel32::GetModuleHandleW(ptr::null()),
                                              ptr::null_mut());
         
         let window_bmi = winapi::BITMAPINFO {
            bmiHeader: winapi::BITMAPINFOHEADER {
               biSize: mem::size_of::<winapi::BITMAPINFOHEADER>() as winapi::DWORD,
               biWidth: config.width as winapi::LONG,
               biHeight: config.height as winapi::LONG,
               biPlanes: 1,
               biBitCount: 32,
               biCompression: winapi::BI_RGB,
               biSizeImage: 0,
               biXPelsPerMeter: 0,
               biYPelsPerMeter: 0,
               biClrUsed: 0,
               biClrImportant: 0,
            },
            bmiColors: [],
         };

         let mut window_buffer: Vec<u32> = Vec::new();
         window_buffer.resize((window_width * window_height) as usize, 0 as u32);

         Ok(Window { 
            handle: handle,
            window_bmi: window_bmi,
            window_buffer: window_buffer,
            key_state: [false; 256],
            key_delta: [false; 256],
            canvas_width: config.width,
            canvas_height: config.height,
            window_width: window_width,
            window_height: window_height,
         })
      }
   }

   pub fn show(&self) {
      unsafe {
         user32::ShowWindow(self.handle, winapi::SW_SHOW);
      }
   }

   pub fn paint(&mut self, bitmap: &Bitmap, palette_colors: &Vec<Color>) {
      unsafe {
         let pixels = self.window_buffer.as_mut_ptr();
         let mut i = 0;

         for y in 0..self.canvas_height {
            for x in 0..self.canvas_width {
               *pixels.offset(i) = palette_colors[bitmap.pixel(x, self.canvas_height - y - 1) as usize].rgba;
               i += 1;
            }
         }

         let dc = user32::GetDC(self.handle);

         gdi32::StretchDIBits(dc,
                              0, 0, self.window_width as i32, self.window_height as i32,
                              0, 0, self.canvas_width as i32, self.canvas_height as i32,
                              mem::transmute::<*mut u32, *const winapi::VOID>(pixels),
                              &self.window_bmi,
                              winapi::DIB_RGB_COLORS,
                              winapi::SRCCOPY);

         user32::ReleaseDC(self.handle, dc);
      }
   }

   pub fn pump(&mut self) -> bool {
     unsafe {
        let mut msg = winapi::MSG {
            hwnd: 0 as winapi::HWND,
            message: 0 as winapi::UINT,
            wParam: 0 as winapi::WPARAM,
            lParam: 0 as winapi::LPARAM,
            time: 0 as winapi::DWORD,
            pt: winapi::POINT { x: 0, y: 0 },
        };

        ptr::write_bytes::<bool>(self.key_delta.as_mut_ptr(), 0, 256);

        while user32::PeekMessageW(&mut msg, 0 as winapi::HWND, 0, 0, winapi::PM_REMOVE) != winapi::FALSE {

            if msg.message == winapi::WM_QUIT {
                return false;
            }

            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);

            if let Some(event) = WIN_EVENT {
                match event {
                    Event::KeyDown(key) => {
                        self.key_state[key as usize] = true;
                        self.key_delta[key as usize] = true;
                    },
                    Event::KeyUp(key) => {
                        self.key_state[key as usize] = false;
                        self.key_delta[key as usize] = true;
                    },
                }
            }

            WIN_EVENT = None;
        }
        true
     }
   }
}


unsafe fn register_window_class() -> Result<Vec<u16>, String> {
   let class_name = to_wstring("Tiny Class");

   let class = winapi::WNDCLASSEXW {
      cbSize: mem::size_of::<winapi::WNDCLASSEXW>() as winapi::UINT,
      style: winapi::CS_HREDRAW | winapi::CS_VREDRAW | winapi::CS_OWNDC,
      lpfnWndProc: Some(wnd_callback),
      cbClsExtra: 0,
      cbWndExtra: 0,
      hInstance: kernel32::GetModuleHandleW(ptr::null()),
      hIcon: user32::LoadIconW(0 as winapi::HINSTANCE, winapi::IDI_APPLICATION),
      hCursor: user32::LoadCursorW(0 as winapi::HINSTANCE, winapi::IDI_APPLICATION), //user32::LoadCursor(0, winapi::IDC_ARROW),
      hbrBackground: gdi32::GetStockObject(winapi::BLACK_BRUSH) as winapi::HBRUSH,
      lpszMenuName: ptr::null(),
      lpszClassName: class_name.as_ptr(),
      hIconSm: ptr::null_mut(),
   };

   let result = user32::RegisterClassExW(&class);

   if result == 0u16 {
      return Err(String::from("Could not register class"));
   }

   Ok(class_name)
}

unsafe extern "system" fn wnd_callback(window: winapi::HWND, msg: winapi::UINT, wparam: winapi::WPARAM, lparam: winapi::LPARAM) -> winapi::LRESULT {

    match msg {
        winapi::WM_DESTROY => {
            println!("Closing window {}", window as i32);
            user32::PostQuitMessage(0);
        },

        winapi::WM_KEYDOWN => {
            println!("key down: {}", wparam);
            WIN_EVENT = Some(Event::KeyDown(wparam as u8));
        },

        winapi::WM_KEYUP => {
            WIN_EVENT = Some(Event::KeyUp(wparam as u8));
        },

        _ => (),
    }

    return user32::DefWindowProcW(window, msg, wparam, lparam)
}