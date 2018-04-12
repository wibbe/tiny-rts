extern crate libc;
extern crate image;

mod platform {  
   mod glutin_window;
   pub use self::glutin_window::*;
}

mod bitmap;

pub mod palette;
pub mod font;

pub use bitmap::*;

use std::cell::RefCell;
use std::result::Result;
use std::cmp;
use std::time::{Instant, Duration};
use std::thread;
use std::fmt;


#[derive(Copy, Clone, PartialEq)]
pub enum Key {
   Key1,
   Key2,
   Key3,
   Key4,
   Key5,
   Key6,
   Key7,
   Key8,
   Key9,
   Key0,
   A,
   B,
   C,
   D,
   E,
   F,
   G,
   H,
   I,
   J,
   K,
   L,
   M,
   N,
   O,
   P,
   Q,
   R,
   S,
   T,
   U,
   V,
   W,
   X,
   Y,
   Z,
   Escape,
   F1,
   F2,
   F3,
   F4,
   F5,
   F6,
   F7,
   F8,
   F9,
   F10,
   F11,
   F12,
   F13,
   F14,
   F15,
   Snapshot,
   Scroll,
   Pause,
   Insert,
   Home,
   Delete,
   End,
   PageDown,
   PageUp,
   Left,
   Up,
   Right,
   Down,
   Back,
   Return,
   Space,
   Compose,
   Caret,
   Numlock,
   Numpad0,
   Numpad1,
   Numpad2,
   Numpad3,
   Numpad4,
   Numpad5,
   Numpad6,
   Numpad7,
   Numpad8,
   Numpad9,
   AbntC1,
   AbntC2,
   Add,
   Apostrophe,
   Apps,
   At,
   Ax,
   Backslash,
   Calculator,
   Capital,
   Colon,
   Comma,
   Convert,
   Decimal,
   Divide,
   Equals,
   Grave,
   Kana,
   Kanji,
   LAlt,
   LBracket,
   LControl,
   LMenu,
   LShift,
   LWin,
   Mail,
   MediaSelect,
   MediaStop,
   Minus,
   Multiply,
   Mute,
   MyComputer,
   NavigateForward,
   NavigateBackward,
   NextTrack,
   NoConvert,
   NumpadComma,
   NumpadEnter,
   NumpadEquals,
   OEM102,
   Period,
   PlayPause,
   Power,
   PrevTrack,
   RAlt,
   RBracket,
   RControl,
   RMenu,
   RShift,
   RWin,
   Semicolon,
   Slash,
   Sleep,
   Stop,
   Subtract,
   Sysrq,
   Tab,
   Underline,
   Unlabeled,
   VolumeDown,
   VolumeUp,
   Wake,
   WebBack,
   WebFavorites,
   WebForward,
   WebHome,
   WebRefresh,
   WebSearch,
   WebStop,
   Yen,
}

#[derive(Copy, Clone, PartialEq)]
pub enum Mouse {
   Left,
   Right,
   Middle,
}


#[derive(Copy, Clone, PartialEq)]
pub struct Rect {
   pub left: i32,
   pub right: i32,
   pub top: i32,
   pub bottom: i32,
}

impl Rect {
   pub fn new(left: i32, top: i32, right: i32, bottom: i32) -> Rect {
      Rect {
         left: left,
         right: right,
         top: top,
         bottom: bottom,
      }
   }

   pub fn new_size(x: i32, y: i32, w: i32, h: i32) -> Rect {
      Rect {
         left: x,
         right: x + w,
         top: y,
         bottom: y + h,
      }
   }

   #[inline]
   pub fn inside(&self, x: i32, y: i32) -> bool {
       self.left <= x && x <= self.right && self.top <= y && y <= self.bottom
   }

   pub fn intersect(&self, r: Rect) -> Rect {
       Rect {
           left: cmp::min(r.right, cmp::max(self.left, r.left)),
           right: cmp::max(r.left, cmp::min(self.right, r.right)),
           top: cmp::min(r.bottom, cmp::max(self.top, r.top)),
           bottom: cmp::max(r.top, cmp::max(self.bottom, r.bottom)),
       }
   }

   #[inline]
   pub fn tr(&self, x: i32, y: i32) -> Rect {
       Rect {
           left: self.left + x,
           right: self.right + x,
           top: self.top + y,
           bottom: self.bottom + y,
       }
   }

   pub fn fit(&self, x: i32, y: i32, w: i32, h: i32) -> Rect {
       Rect {
           left: cmp::max(self.left, x),
           right: cmp::min(self.right, x + w),
           top: cmp::max(self.top, y),
           bottom: cmp::min(self.bottom, y + h),
       }
   }

   #[inline]
   pub fn grow(&self, w: i32, h: i32) -> Rect {
      Rect {
         left: self.left,
         right: self.right + w,
         top: self.top,
         bottom: self.bottom + h,
      }
   }

   #[inline]
   pub fn width(&self) -> i32 {
      self.right - self.left
   }

   #[inline]
   pub fn height(&self) -> i32 {
      self.bottom - self.top
   }
}

impl fmt::Display for Rect {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "(left: {}, right: {}, top: {}, bottom: {})", self.left, self.right, self.top, self.bottom)
    }
}


#[derive(Copy, Clone, PartialEq)]
pub struct Color {
    pub rgba: u32,
}

impl Color {
   pub fn new(r: u8, g: u8, b: u8, a: u8) -> Color {
      Color { rgba: (a as u32) << 24 | (b as u32) << 16 | (g as u32) << 8 | r as u32 }
   }

   #[inline]
   pub fn red(&self) -> u8 {
      (self.rgba & 0xff) as u8
   }

   #[inline]
   pub fn green(&self) -> u8 {
      ((self.rgba >> 8) & 0xff) as u8
   }

   #[inline]
   pub fn blue(&self) -> u8 {
      ((self.rgba >> 16) & 0xff) as u8
   }

   #[inline]
   pub fn alpha(&self) -> u8 {
      ((self.rgba >> 24) & 0xff) as u8
   }

   #[inline]
   pub fn redf(&self) -> f32 {
      self.red() as f32 / 255.0
   }

   #[inline]
   pub fn greenf(&self) -> f32 {
      self.green() as f32 / 255.0
   }

   #[inline]
   pub fn bluef(&self) -> f32 {
      self.blue() as f32 / 255.0
   }

   #[inline]
   pub fn alphaf(&self) -> f32 {
      self.alpha() as f32 / 255.0
   }
}

pub struct Font {
    pub bitmap: Bitmap,
    pub char_width: i32,
    pub char_height: i32,
    pub line_height: i32,
}

impl Font {
   pub fn new(bitmap: Bitmap, char_width: u32, char_height: u32, line_height: u32) -> Font {
      Font {
         bitmap: bitmap,
         char_width: char_width as i32,
         char_height: char_height as i32,
         line_height: line_height as i32,
      }
   }

   pub fn measure(&self, text: &str) -> Rect {
      let mut x_curr = 0;
      let mut x_max = 0;
      let mut y_max = self.char_height;

      for ch in text.chars() {
         let idx = ch as u32;
         if idx < 256 {
            match ch {
               ' ' => x_curr += self.char_width,
               '\t' => x_curr += self.char_width,
               '\n' => {
                  x_curr = 0;
                  y_max += self.line_height;
               },
               _ => x_curr += self.char_width,
            }

            if x_curr > x_max {
               x_max = x_curr;
            }
         }
      }

      Rect::new_size(0, 0, x_max, y_max)
   }
}


#[derive(Clone)]
pub struct Palette {
   colors: Vec<Color>,
}

impl Palette {
   fn new() -> Palette {
      Palette {
         colors: vec![Color::new(0, 0, 0, 0), Color::new(0, 0, 0, 255), Color::new(255, 255, 255, 255)],
      }
   }

   fn add_color(&mut self, color: Color) -> u8 {
      for i in 0..self.colors.len() {
         if self.colors[i].rgba == color.rgba {
            return i as u8;
         }
      }

      self.colors.push(color);
      (self.colors.len() - 1) as u8
   }
}

pub struct Config {
   title: String,
   width: u32,
   height: u32,
   scale: u32,
}

pub const DRAW_FLIP_H: u32 = (1 << 1);
pub const DRAW_MASK: u32 = (1 << 2);

pub trait Painter {
   fn clip(&self, rect: Option<Rect>);

   fn clear(&self, color: u8);

   fn pixel(&self, x: i32, y: i32, color: u8);
   
   fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: u8);

   fn rect_stroke(&self, rect: Rect, color: u8);
   fn rect_fill(&self, rect: Rect, color: u8);

   fn blit(&self, x: i32, y: i32, source: &Bitmap, source_rect: Rect, flags: u32, color: u8);

   fn text(&self, x: i32, y: i32, text: &str, color: u8, font: &Font);
}

pub trait Application : Sized {
   fn new(ctx: &mut Context) -> Result<Self, String>;

   fn step(&mut self, ctx: &Context) -> bool { !ctx.key_pressed(Key::Escape) }
   fn paint(&self, ctx: &Context, painter: &Painter);
}


use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
static IS_TINY_CONTEXT_ALIVE: AtomicBool = ATOMIC_BOOL_INIT;


pub const TRANSPARENT: u8 = 0;
pub const BLACK: u8 = 1;
pub const WHITE: u8 = 2;


pub struct Context {
   palette: RefCell<Palette>,
   window: platform::Window,

   pub frame_time: f64,
   pub step_time: f64,
   pub paint_time: f64,
   pub blit_time: f64,
   pub sleep_time: f64,
}

impl Context {
   fn new(window: platform::Window, config: &Config) -> Context {
      Context {
         palette: RefCell::new(Palette::new()),
         window: window,
         frame_time: 0.0,
         step_time: 0.0,
         paint_time: 0.0,
         blit_time: 0.0,
         sleep_time: 0.0,
      }
   }

   pub fn set_palette(&mut self, palette: Palette) {
      *self.palette.borrow_mut() = palette;
   }

   pub fn palette(&self) -> Palette {
      self.palette.borrow().clone()
   }

   pub fn palette_add(&self, color: Color) -> u8 {
       self.palette.borrow_mut().add_color(color)
   }

   pub fn key_down(&self, key: Key) -> bool {
       self.window.key_state[key as usize]
   }

   pub fn key_pressed(&self, key: Key) -> bool {
       self.window.key_state[key as usize] && self.window.key_delta[key as usize]
   }

   pub fn mouse_down(&self, mouse: Mouse) -> bool {
      self.window.mouse_state[mouse as usize]
   }

   pub fn mouse_pressed(&self, mouse: Mouse) -> bool {
       self.window.mouse_state[mouse as usize] && self.window.mouse_delta[mouse as usize]
   }

   pub fn mouse_position(&self) -> (u32, u32) {
      (self.window.mouse_x, self.window.mouse_y)
   }

   pub fn set_background_color(&mut self, color: Color) {
      self.window.set_background_color(color);
   }

   pub fn draw_timing(&self, painter: &Painter, font: &Font, background_color: u8, foreground_color: u8) {
      let text = format!("FRAME: {:4.1} MS\nPAINT: {:4.1} MS\n STEP: {:4.1} MS\n BLIT: {:4.1} MS\nSLEEP: {:4.1} MS", self.frame_time, self.paint_time, self.step_time, self.blit_time, self.sleep_time);
      let text_rect = font.measure(&text);
      let background_rect = text_rect.tr(2, 2).grow(4, 4);

      painter.clip(Some(background_rect));
      painter.rect_fill(background_rect, background_color);
      painter.text(background_rect.left + 2, background_rect.top + 2, &text, foreground_color, font);
      painter.clip(None);
   }
}


fn to_milisec(duration: Duration) -> f64 {
   (duration.as_secs() as f64 * 1_000f64) + (duration.subsec_nanos() as f64 / 1_000_000f64)
}


pub fn run<T: Application>(title: &str, width: u32, height: u32, scale: u32) -> Result<(), String> {
   use std::sync::atomic::Ordering;
   let was_alive = IS_TINY_CONTEXT_ALIVE.swap(true, Ordering::Relaxed);
   if was_alive {
      return Err("Cannot initialize Tiny more than once at a time".to_owned());
   }


   let config = Config {
      title: String::from(title),
      width: width,
      height: height,
      scale: scale,
   };

   println!("Starting '{}' with resolution {}x{} at scale {}", config.title, config.width, config.height, config.scale);

   let mut context = match platform::Window::new(&config) {
      Ok(window) => Context::new(window, &config),
      Err(err) => return Err(err),
   };

   let mut canvas = Bitmap::new(config.width, config.height);

   // Initialize the application
   let mut app = match T::new(&mut context) {
      Ok(app) => app,
      Err(err) => return Err(err),
   };
   
   context.window.show();

   let target_frame_time = 33_333_333u32; // An fps of 30Hz

   let mut step_time;
   let mut paint_time = 0.0;
   let mut blit_time = 0.0;
   let mut frame_time = 0.0;
   let mut sleep_time = 0.0;

   // Main loop
   'main: loop {
      let frame_now = Instant::now();

      // Handle messages
      if !context.window.pump() {
         break;
      }

      {  // Step the application
         let step_now = Instant::now();
         
         if !app.step(&context) {
             break 'main;
         }

         step_time = to_milisec(step_now.elapsed());
      }

      {  // Let the application paint to the canvas
         let paint_now = Instant::now();

         let p = BitmapPainter::new(&mut canvas);
         app.paint(&context, &p);

         paint_time = to_milisec(paint_now.elapsed());
      }

      {  // Blit canvas to the window
         let blit_now = Instant::now();

         if let Err(err) = context.window.paint(&canvas, &context.palette.borrow_mut().colors) {
            return Err(err);            
         }

         blit_time = to_milisec(blit_now.elapsed());
      }

      let frame_duration = frame_now.elapsed();
      frame_time = to_milisec(frame_duration);

      context.step_time = context.step_time * 0.98 + step_time * 0.02;
      context.paint_time = context.paint_time * 0.98 + paint_time * 0.02;
      context.blit_time = context.blit_time * 0.98 + blit_time * 0.02;
      context.frame_time = context.frame_time * 0.98 + frame_time * 0.02;

      // Sleep to force the frame time to 33ms
      if frame_duration.as_secs() == 0 && frame_duration.subsec_nanos() < target_frame_time {
         let sleep_duration = Duration::new(0, target_frame_time - frame_duration.subsec_nanos());
         
         sleep_time = to_milisec(sleep_duration);
         context.sleep_time = context.sleep_time * 0.98 + sleep_time * 0.02;
         
         thread::sleep(sleep_duration);
      }
   }

   let was_alive = IS_TINY_CONTEXT_ALIVE.swap(false, Ordering::Relaxed);
   assert!(was_alive);
   Ok(())
}

#[cfg(test)]
mod tests {
   #[test]
   fn it_works() {
   }
}
