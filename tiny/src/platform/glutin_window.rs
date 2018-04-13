
mod gl {
   include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

extern crate glutin;

use self::glutin::GlContext;
use std::boxed::Box;
use std::mem;
use std::ptr;

use super::super::*;

pub struct Window {
   events_loop: Box<glutin::EventsLoop>,
   window: Box<glutin::GlWindow>,

   background_color: Color,
  
   pub key_state: [bool; 256],
   pub key_delta: [bool; 256],

   pub mouse_state: [bool; 3],
   pub mouse_delta: [bool; 3],

   pub mouse_x: u32,
   pub mouse_y: u32,

   canvas_buffer: Vec<u32>,
   canvas_width: u32,
   canvas_height: u32,
   canvas_tex: u32,

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
         //.with_min_dimensions(window_width, window_height)
         //.with_max_dimensions(window_width, window_height)
         .with_title(config.title.to_string())
         .with_visibility(true);

      let context = glutin::ContextBuilder::new();

      let gl_window = match glutin::GlWindow::new(window, context, &events_loop) {
         Ok(win) => Box::new(win),
         Err(_) => return Err("Could not create OpenGL Window".to_string()),
      };

      let mut canvas_tex = 0;

      unsafe {
         match gl_window.make_current() {
            Err(_) => return Err("Could not make OpenGL context current".to_string()),
            _ => (),
         }

         gl::load_with(|symbol| gl_window.get_proc_address(symbol) as *const _);
         gl::ClearColor(0.0, 1.0, 0.0, 1.0);

         // Generate canvas texture
         gl::Enable(gl::TEXTURE_2D);
         gl::GenTextures(1, &mut canvas_tex);
         gl::BindTexture(gl::TEXTURE_2D, canvas_tex);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MIN_FILTER as u32, gl::NEAREST as i32);
         gl::TexParameteri(gl::TEXTURE_2D, gl::TEXTURE_MAG_FILTER as u32, gl::NEAREST as i32);
      }

      gl_window.show();
      gl_window.resize(window_width, window_height);

      // Restrict the size of the window
      if let Some((w, h)) = gl_window.get_outer_size() {
         println!("Outer size {}x{}", w, h);
         gl_window.set_min_dimensions(Some((w, h)));
         gl_window.set_max_dimensions(Some((w, h)));
      }


      if let Some((w, h)) = gl_window.get_inner_size() {
         println!("Window size: {}x{} ({}x{})", w, h, window_width, window_height);
      }


      Ok(Window { 
         events_loop: events_loop,
         window: gl_window,

         background_color: Color::new(0, 0, 0, 255),

         key_state: [false; 256],
         key_delta: [false; 256],

         mouse_state: [false; 3],
         mouse_delta: [false; 3],

         mouse_x: 0,
         mouse_y: 0,

         canvas_buffer: canvas_buffer,
         canvas_tex: canvas_tex,
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

   pub fn set_background_color(&mut self, color: Color) {
      self.background_color = color;
   }

   pub fn paint(&mut self, bitmap: &Bitmap, palette_colors: &Vec<Color>) -> Result<(), String> {

      // We start by updating the canvas buffer
      unsafe {
         let pixels = self.canvas_buffer.as_mut_ptr();
         let mut i = 0;

         for y in 0..self.canvas_height {
            for x in 0..self.canvas_width {
               *pixels.offset(i) = palette_colors[bitmap.pixel(x, self.canvas_height - y - 1) as usize].rgba;
               i += 1;
            }
         }
      }

      unsafe {
         if let Err(err) = self.window.make_current() {
            return Err(format!("Could not make OpenGL context current: {}", err).to_string());
         }

         gl::ClearColor(self.background_color.redf(),
                        self.background_color.greenf(),
                        self.background_color.bluef(),
                        1.0);
         gl::Clear(gl::COLOR_BUFFER_BIT);

         gl::Viewport(0, 0, self.window_width as i32, self.window_height as i32);
         gl::MatrixMode(gl::PROJECTION);
         gl::LoadIdentity();
         gl::Ortho(0.0, self.window_width as f64, 0.0, self.window_height as f64, 1.0, -1.0);
         gl::MatrixMode(gl::MODELVIEW);
         gl::LoadIdentity();

         gl::BindTexture(gl::TEXTURE_2D, self.canvas_tex);
         gl::TexImage2D(gl::TEXTURE_2D, 0, gl::RGBA as i32, self.canvas_width as i32, self.canvas_height as i32, 0, gl::RGBA, gl::UNSIGNED_BYTE, mem::transmute(self.canvas_buffer.as_ptr()));

         gl::Begin(gl::QUADS);
            gl::TexCoord2f(0.0, 0.0);
            gl::Vertex2f(0.0, 0.0);

            gl::TexCoord2f(1.0, 0.0);
            gl::Vertex2f(self.window_width as f32, 0.0);

            gl::TexCoord2f(1.0, 1.0);
            gl::Vertex2f(self.window_width as f32, self.window_height as f32);

            gl::TexCoord2f(0.0, 1.0);
            gl::Vertex2f(0.0, self.window_height as f32);
         gl::End();
      }

      if let Err(err) = self.window.swap_buffers() {
         return Err(format!("Could not swap window buffer: {}", err).to_string());
      }

      Ok(())
   }

   pub fn pump(&mut self) -> bool {
      let mut running = true;
      //let window = &self.window;

      unsafe { ptr::write_bytes::<bool>(self.key_delta.as_mut_ptr(), 0, 256); }
      unsafe { ptr::write_bytes::<bool>(self.mouse_delta.as_mut_ptr(), 0, 3); }

      let events_loop = &mut self.events_loop;
      let window = &mut self.window;
      let key_state = &mut self.key_state;
      let key_delta = &mut self.key_delta;
      let mouse_state = &mut self.mouse_state;
      let mouse_delta = &mut self.mouse_delta;
      let window_width = &mut self.window_width;
      let window_height = &mut self.window_height;
      let canvas_width = self.canvas_width;
      let canvas_height = self.canvas_height;
      let mouse_x = &mut self.mouse_x;
      let mouse_y = &mut self.mouse_y;

      events_loop.poll_events(|event| {
         match event {
            glutin::Event::WindowEvent{ event, .. } => match event {
               glutin::WindowEvent::Closed => running = false,
               
               glutin::WindowEvent::Resized(w, h) => {
                  *window_width = w;
                  *window_height = h;
                  window.resize(w, h);
               },

               glutin::WindowEvent::ReceivedCharacter(ch) => {
                  println!("Input: ({}) '{}'", ch as u32, ch);
               },

               glutin::WindowEvent::KeyboardInput { input, .. } => {
                  if let Some(virtual_keycode) = input.virtual_keycode {
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
                  }
               },

               glutin::WindowEvent::MouseInput { state, button, .. } => {
                  let button = match button {
                     glutin::MouseButton::Left => Some(Mouse::Left),
                     glutin::MouseButton::Right => Some(Mouse::Right),
                     glutin::MouseButton::Middle => Some(Mouse::Middle),
                     _ => None,
                  };

                  if let Some(button) = button {
                     match state {
                        glutin::ElementState::Pressed => {
                           mouse_state[button as usize] = true;
                           mouse_delta[button as usize] = true;
                        },
                        glutin::ElementState::Released => {
                           mouse_state[button as usize] = false;
                           mouse_delta[button as usize] = true;
                        },              
                     }
                  }
               },

               glutin::WindowEvent::CursorMoved { position, .. } => {
                  *mouse_x = ((position.0 / *window_width as f64) * canvas_width as f64) as u32;
                  *mouse_y = ((position.1 / *window_height as f64) * canvas_height as f64) as u32;
               },

               _ => (),
            },
            _ => (),
         }
      });

      running
   }
}

fn keycode_glutin_to_tiny(key_code: glutin::VirtualKeyCode) -> Key {
   match key_code {
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