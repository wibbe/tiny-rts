
#[windows_subsystem(windows)]

use std::ptr;

use std::mem;
use std::ffi::{OsStr};
use std::os::windows::ffi::OsStrExt;

use super::winapi;
use super::gdi32;
use super::user32;
use super::kernel32;

use super::super::*;

#[derive(Copy, Clone)]
enum Event {
    KeyDown(u8),
    KeyUp(u8),
    MouseMove(i32, i32),
    MouseDown(Mouse),
    MouseUp(Mouse),
    Text(char),
}

static mut WIN_EVENT: Option<Event> = None;

pub const COLOR_OFFSET_R: u32 = 16;
pub const COLOR_OFFSET_G: u32 = 8;
pub const COLOR_OFFSET_B: u32 = 0;
pub const COLOR_OFFSET_A: u32 = 24;


pub struct Window {
   handle: winapi::HWND,
   window_bmi: winapi::BITMAPINFO,
   window_buffer: Vec<u32>,

   background_color: Color,
   
   pub key_state: [bool; 256],
   pub key_delta: [bool; 256],

   pub mouse_state: [bool; 3],
   pub mouse_delta: [bool; 3],

   pub text_input: Vec<char>,

   pub mouse_x: u32,
   pub mouse_y: u32,

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

            background_color: Color::new(0, 0, 0, 255),

            key_state: [false; 256],
            key_delta: [false; 256],

            text_input: Vec::with_capacity(8),

            mouse_state: [false; 3],
            mouse_delta: [false; 3],

            mouse_x: 0,
            mouse_y: 0,

            canvas_width: config.width,
            canvas_height: config.height,
            window_width: window_width,
            window_height: window_height,
         })
      }
   }

   #[allow(dead_code)]
   pub fn show(&self) {
      unsafe {
         user32::ShowWindow(self.handle, winapi::SW_SHOW);
      }
   }

   #[allow(dead_code)]
   pub fn hide(&self) {
      unsafe {
         user32::ShowWindow(self.handle, winapi::SW_HIDE);
      }
   }

   #[allow(dead_code)]
   pub fn set_background_color(&mut self, color: Color) {
      self.background_color = color;
   }

   pub fn paint(&mut self, bitmap: &Bitmap, palette_colors: &Vec<Color>) -> Result<(), String> {
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

      Ok(())
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
         ptr::write_bytes::<bool>(self.mouse_delta.as_mut_ptr(), 0, 3);

         self.text_input.clear();

         while user32::PeekMessageW(&mut msg, 0 as winapi::HWND, 0, 0, winapi::PM_REMOVE) != winapi::FALSE {

            if msg.message == winapi::WM_QUIT {
                return false;
            }

            user32::TranslateMessage(&mut msg);
            user32::DispatchMessageW(&mut msg);

            if let Some(event) = WIN_EVENT {
               match event {
                  Event::KeyDown(key) => {
                     if let Some(key) = keycode_win32_to_tiny(key) {
                        self.key_state[key as usize] = true;
                        self.key_delta[key as usize] = true;
                     }
                  },
                    
                  Event::KeyUp(key) => {
                     if let Some(key) = keycode_win32_to_tiny(key) {
                        self.key_state[key as usize] = false;
                        self.key_delta[key as usize] = true;
                     }
                  },

                  Event::Text(ch) => {

                  },

                  Event::MouseMove(x, y) => {
                     self.mouse_x = ((x as f64 / self.window_width as f64) * self.canvas_width as f64) as u32;
                     self.mouse_y = ((y as f64 / self.window_height as f64) * self.canvas_height as f64) as u32;
                  },

                  Event::MouseDown(button) => {
                     self.mouse_state[button as usize] = true;
                     self.mouse_delta[button as usize] = true;
                  },

                  Event::MouseUp(button) => {
                     self.mouse_state[button as usize] = false;
                     self.mouse_delta[button as usize] = true;
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
         user32::PostQuitMessage(0);
      },

      winapi::WM_KEYDOWN => {
         println!("key down: {}", wparam);
         WIN_EVENT = Some(Event::KeyDown(wparam as u8));
      },

      winapi::WM_KEYUP => {
         WIN_EVENT = Some(Event::KeyUp(wparam as u8));
      },

      winapi::WM_CHAR => {
         WIN_EVENT = Some(Event::Text(wparam as u8 as char));
         println!("Text: ({}) '{}'", wparam as u32, wparam as u8 as char);
      }

      winapi::WM_MOUSEMOVE => {
         WIN_EVENT = Some(Event::MouseMove(winapi::windowsx::GET_X_LPARAM(lparam) as i32, winapi::windowsx::GET_Y_LPARAM(lparam) as i32));
      },

      winapi::WM_LBUTTONUP => {
         WIN_EVENT = Some(Event::MouseUp(Mouse::Left));
      },

      winapi::WM_LBUTTONDOWN => {
         WIN_EVENT = Some(Event::MouseDown(Mouse::Left));
      },

      winapi::WM_MBUTTONUP => {
         WIN_EVENT = Some(Event::MouseUp(Mouse::Middle));
      },

      winapi::WM_MBUTTONDOWN => {
         WIN_EVENT = Some(Event::MouseDown(Mouse::Middle));
      },

      winapi::WM_RBUTTONUP => {
         WIN_EVENT = Some(Event::MouseUp(Mouse::Right));
      },

      winapi::WM_RBUTTONDOWN => {
         WIN_EVENT = Some(Event::MouseDown(Mouse::Right));
      },

      _ => (),
    }

    return user32::DefWindowProcW(window, msg, wparam, lparam)
}

fn keycode_win32_to_tiny(key_code: u8) -> Option<Key> {
   match key_code {
      8 => Some(Key::Back),
      9 => Some(Key::Tab),
      13 => Some(Key::Return),

      16 => Some(Key::LShift),
      17 => Some(Key::LControl),

      27 => Some(Key::Escape),

      32 => Some(Key::Space),
      33 => Some(Key::PageUp),
      34 => Some(Key::PageDown),
      35 => Some(Key::End),
      36 => Some(Key::Home),
      37 => Some(Key::Left),
      38 => Some(Key::Up),
      39 => Some(Key::Right),
      40 => Some(Key::Down),

      45 => Some(Key::Insert),
      46 => Some(Key::Delete),

      112 => Some(Key::F1),
      113 => Some(Key::F2),
      114 => Some(Key::F3),
      115 => Some(Key::F4),
      116 => Some(Key::F5),
      117 => Some(Key::F6),
      118 => Some(Key::F7),
      119 => Some(Key::F8),
      120 => Some(Key::F9),
      121 => Some(Key::F10),
      122 => Some(Key::F11),
      123 => Some(Key::F12),

      _ => None,
/*
      glutin::VirtualKeyCode::Key1 => Key::Key1,
      glutin::VirtualKeyCode::Key2 => Key::Key2,
      glutin::VirtualKeyCode::Key3 => Key::Key3,
      glutin::VirtualKeyCode::Key4 => Key::Key4,
      glutin::VirtualKeyCode::Key5 => Key::Key5,
      glutin::VirtualKeyCode::Key6 => Key::Key6,
      glutin::VirtualKeyCode::Key7 => Key::Key7,
      glutin::VirtualKeyCode::Key8 => Key::Key8,
      glutin::VirtualKeyCode::Key9 => Key::Key9,
      glutin::VirtualKeyCode::Key0 => Key::Key0,
      glutin::VirtualKeyCode::A => Key::A,
      glutin::VirtualKeyCode::B => Key::B,
      glutin::VirtualKeyCode::C => Key::C,
      glutin::VirtualKeyCode::D => Key::D,
      glutin::VirtualKeyCode::E => Key::E,
      glutin::VirtualKeyCode::F => Key::F,
      glutin::VirtualKeyCode::G => Key::G,
      glutin::VirtualKeyCode::H => Key::H,
      glutin::VirtualKeyCode::I => Key::I,
      glutin::VirtualKeyCode::J => Key::J,
      glutin::VirtualKeyCode::K => Key::K,
      glutin::VirtualKeyCode::L => Key::L,
      glutin::VirtualKeyCode::M => Key::M,
      glutin::VirtualKeyCode::N => Key::N,
      glutin::VirtualKeyCode::O => Key::O,
      glutin::VirtualKeyCode::P => Key::P,
      glutin::VirtualKeyCode::Q => Key::Q,
      glutin::VirtualKeyCode::R => Key::R,
      glutin::VirtualKeyCode::S => Key::S,
      glutin::VirtualKeyCode::T => Key::T,
      glutin::VirtualKeyCode::U => Key::U,
      glutin::VirtualKeyCode::V => Key::V,
      glutin::VirtualKeyCode::W => Key::W,
      glutin::VirtualKeyCode::X => Key::X,
      glutin::VirtualKeyCode::Y => Key::Y,
      glutin::VirtualKeyCode::Z => Key::Z,
      glutin::VirtualKeyCode::Escape => Key::Escape,
      glutin::VirtualKeyCode::F1 => Key::F1,
      glutin::VirtualKeyCode::F2 => Key::F2,
      glutin::VirtualKeyCode::F3 => Key::F3,
      glutin::VirtualKeyCode::F4 => Key::F4,
      glutin::VirtualKeyCode::F5 => Key::F5,
      glutin::VirtualKeyCode::F6 => Key::F6,
      glutin::VirtualKeyCode::F7 => Key::F7,
      glutin::VirtualKeyCode::F8 => Key::F8,
      glutin::VirtualKeyCode::F9 => Key::F9,
      glutin::VirtualKeyCode::F10 => Key::F10,
      glutin::VirtualKeyCode::F11 => Key::F11,
      glutin::VirtualKeyCode::F12 => Key::F12,
      glutin::VirtualKeyCode::F13 => Key::F13,
      glutin::VirtualKeyCode::F14 => Key::F14,
      glutin::VirtualKeyCode::F15 => Key::F15,
      glutin::VirtualKeyCode::Snapshot => Key::Snapshot,
      glutin::VirtualKeyCode::Scroll => Key::Scroll,
      glutin::VirtualKeyCode::Pause => Key::Pause,
      glutin::VirtualKeyCode::Insert => Key::Insert,
      glutin::VirtualKeyCode::Home => Key::Home,
      glutin::VirtualKeyCode::Delete => Key::Delete,
      glutin::VirtualKeyCode::End => Key::End,
      glutin::VirtualKeyCode::PageDown => Key::PageDown,
      glutin::VirtualKeyCode::PageUp => Key::PageUp,
      glutin::VirtualKeyCode::Left => Key::Left,
      glutin::VirtualKeyCode::Up => Key::Up,
      glutin::VirtualKeyCode::Right => Key::Right,
      glutin::VirtualKeyCode::Down => Key::Down,
      glutin::VirtualKeyCode::Back => Key::Back,
      glutin::VirtualKeyCode::Return => Key::Return,
      glutin::VirtualKeyCode::Space => Key::Space,
      glutin::VirtualKeyCode::Compose => Key::Compose,
      glutin::VirtualKeyCode::Caret => Key::Caret,
      glutin::VirtualKeyCode::Numlock => Key::Numlock,
      glutin::VirtualKeyCode::Numpad0 => Key::Numpad0,
      glutin::VirtualKeyCode::Numpad1 => Key::Numpad1,
      glutin::VirtualKeyCode::Numpad2 => Key::Numpad2,
      glutin::VirtualKeyCode::Numpad3 => Key::Numpad3,
      glutin::VirtualKeyCode::Numpad4 => Key::Numpad4,
      glutin::VirtualKeyCode::Numpad5 => Key::Numpad5,
      glutin::VirtualKeyCode::Numpad6 => Key::Numpad6,
      glutin::VirtualKeyCode::Numpad7 => Key::Numpad7,
      glutin::VirtualKeyCode::Numpad8 => Key::Numpad8,
      glutin::VirtualKeyCode::Numpad9 => Key::Numpad9,
      glutin::VirtualKeyCode::AbntC1 => Key::AbntC1,
      glutin::VirtualKeyCode::AbntC2 => Key::AbntC2,
      glutin::VirtualKeyCode::Add => Key::Add,
      glutin::VirtualKeyCode::Apostrophe => Key::Apostrophe,
      glutin::VirtualKeyCode::Apps => Key::Apps,
      glutin::VirtualKeyCode::At => Key::At,
      glutin::VirtualKeyCode::Ax => Key::Ax,
      glutin::VirtualKeyCode::Backslash => Key::Backslash,
      glutin::VirtualKeyCode::Calculator => Key::Calculator,
      glutin::VirtualKeyCode::Capital => Key::Capital,
      glutin::VirtualKeyCode::Colon => Key::Colon,
      glutin::VirtualKeyCode::Comma => Key::Comma,
      glutin::VirtualKeyCode::Convert => Key::Convert,
      glutin::VirtualKeyCode::Decimal => Key::Decimal,
      glutin::VirtualKeyCode::Divide => Key::Divide,
      glutin::VirtualKeyCode::Equals => Key::Equals,
      glutin::VirtualKeyCode::Grave => Key::Grave,
      glutin::VirtualKeyCode::Kana => Key::Kana,
      glutin::VirtualKeyCode::Kanji => Key::Kanji,
      glutin::VirtualKeyCode::LAlt => Key::LAlt,
      glutin::VirtualKeyCode::LBracket => Key::LBracket,
      glutin::VirtualKeyCode::LControl => Key::LControl,
      glutin::VirtualKeyCode::LMenu => Key::LMenu,
      glutin::VirtualKeyCode::LShift => Key::LShift,
      glutin::VirtualKeyCode::LWin => Key::LWin,
      glutin::VirtualKeyCode::Mail => Key::Mail,
      glutin::VirtualKeyCode::MediaSelect => Key::MediaSelect,
      glutin::VirtualKeyCode::MediaStop => Key::MediaStop,
      glutin::VirtualKeyCode::Minus => Key::Minus,
      glutin::VirtualKeyCode::Multiply => Key::Multiply,
      glutin::VirtualKeyCode::Mute => Key::Mute,
      glutin::VirtualKeyCode::MyComputer => Key::MyComputer,
      glutin::VirtualKeyCode::NavigateForward => Key::NavigateForward,
      glutin::VirtualKeyCode::NavigateBackward => Key::NavigateBackward,
      glutin::VirtualKeyCode::NextTrack => Key::NextTrack,
      glutin::VirtualKeyCode::NoConvert => Key::NoConvert,
      glutin::VirtualKeyCode::NumpadComma => Key::NumpadComma,
      glutin::VirtualKeyCode::NumpadEnter => Key::NumpadEnter,
      glutin::VirtualKeyCode::NumpadEquals => Key::NumpadEquals,
      glutin::VirtualKeyCode::OEM102 => Key::OEM102,
      glutin::VirtualKeyCode::Period => Key::Period,
      glutin::VirtualKeyCode::PlayPause => Key::PlayPause,
      glutin::VirtualKeyCode::Power => Key::Power,
      glutin::VirtualKeyCode::PrevTrack => Key::PrevTrack,
      glutin::VirtualKeyCode::RAlt => Key::RAlt,
      glutin::VirtualKeyCode::RBracket => Key::RBracket,
      glutin::VirtualKeyCode::RControl => Key::RControl,
      glutin::VirtualKeyCode::RMenu => Key::RMenu,
      glutin::VirtualKeyCode::RShift => Key::RShift,
      glutin::VirtualKeyCode::RWin => Key::RWin,
      glutin::VirtualKeyCode::Semicolon => Key::Semicolon,
      glutin::VirtualKeyCode::Slash => Key::Slash,
      glutin::VirtualKeyCode::Sleep => Key::Sleep,
      glutin::VirtualKeyCode::Stop => Key::Stop,
      glutin::VirtualKeyCode::Subtract => Key::Subtract,
      glutin::VirtualKeyCode::Sysrq => Key::Sysrq,
      glutin::VirtualKeyCode::Tab => Key::Tab,
      glutin::VirtualKeyCode::Underline => Key::Underline,
      glutin::VirtualKeyCode::Unlabeled => Key::Unlabeled,
      glutin::VirtualKeyCode::VolumeDown => Key::VolumeDown,
      glutin::VirtualKeyCode::VolumeUp => Key::VolumeUp,
      glutin::VirtualKeyCode::Wake => Key::Wake,
      glutin::VirtualKeyCode::WebBack => Key::WebBack,
      glutin::VirtualKeyCode::WebFavorites => Key::WebFavorites,
      glutin::VirtualKeyCode::WebForward => Key::WebForward,
      glutin::VirtualKeyCode::WebHome => Key::WebHome,
      glutin::VirtualKeyCode::WebRefresh => Key::WebRefresh,
      glutin::VirtualKeyCode::WebSearch => Key::WebSearch,
      glutin::VirtualKeyCode::WebStop => Key::WebStop,
      glutin::VirtualKeyCode::Yen => Key::Yen,
*/
   }
}