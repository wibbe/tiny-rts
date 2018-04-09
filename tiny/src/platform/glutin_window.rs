
mod gl {
   include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

extern crate glutin;

use self::glutin::GlContext;
use std::boxed::Box;
use std::ptr;

use super::super::*;

pub struct Window {
   events_loop: Box<glutin::EventsLoop>,
   window: Box<glutin::GlWindow>,

   canvas_buffer: Vec<u32>,
   pub key_state: [bool; 256],
   pub key_delta: [bool; 256],
   canvas_width: u32,
   canvas_height: u32,
   window_width: u32,
   window_height: u32,
}

impl Window {
   pub fn new(config: &Config) -> Result<Window, String> {
      let window_width = config.width * config.scale;
      let window_height = config.height * config.scale;

      let mut canvas_buffer: Vec<u32> = Vec::new();
      canvas_buffer.resize((config.width * config.height) as usize, 0 as u32);

      let events_loop = Box::new(glutin::EventsLoop::new());
      let window = glutin::WindowBuilder::new()
         .with_dimensions(window_width, window_height)
         .with_title(config.title.to_string())
         .with_visibility(true);

      let context = glutin::ContextBuilder::new();

      let gl_window = match glutin::GlWindow::new(window, context, &events_loop) {
         Ok(win) => Box::new(win),
         Err(_) => return Err("Could not create OpenGL Window".to_string()),
      };

      unsafe {
         match gl_window.make_current() {
            Err(_) => return Err("Could not make OpenGL context current".to_string()),
            _ => (),
         }

        gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
        gl::ClearColor(0.0, 1.0, 0.0, 1.0);
      }

      Ok(Window { 
         events_loop: events_loop,
         window: gl_window,

         canvas_buffer: canvas_buffer,
         key_state: [false; 256],
         key_delta: [false; 256],
         canvas_width: config.width,
         canvas_height: config.height,
         window_width: window_width,
         window_height: window_height,
      })
   }

   pub fn show(&self) {
      self.window.show();
   }

   pub fn hide(&self) {
      self.window.hide();
   }

   pub fn paint(&mut self, _bitmap: &Bitmap, _palette_colors: &Vec<Color>) {
      unsafe {
         self.window.make_current().unwrap();
         gl::Clear(gl::COLOR_BUFFER_BIT);
      }

      self.window.swap_buffers().unwrap();
   }

   pub fn pump(&mut self) -> bool {
      let mut running = true;
      let window = &self.window;

      unsafe { ptr::write_bytes::<bool>(self.key_delta.as_mut_ptr(), 0, 256); }

      let mut key_delta = &mut self.key_delta;
      let mut key_state = &mut self.key_state;

      self.events_loop.poll_events(|event| {
         match event {
            glutin::Event::WindowEvent{ event, .. } => match event {
               glutin::WindowEvent::Closed => running = false,
               glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
               glutin::WindowEvent::KeyboardInput { input, .. } => if let Some(virtual_keycode) = input.virtual_keycode {
                  let key = keycode_glutin_to_tiny(virtual_keycode);

                  match input.state {
                     glutin::ElementState::Pressed => {
                        key_state[key as usize] = true;
                        key_delta[key as usize] = true;
                     },
                     glutin::ElementState::Released => {
                        key_state[key as usize] = false;
                        key_delta[key as usize] = true;
                     },                        
                  }
               },
               _ => (),
            },
            _ => (),
         }
      });

      running
   }
}

fn keycode_glutin_to_tiny(keyCode: glutin::VirtualKeyCode) -> Key {
   match keyCode {
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
   }
}