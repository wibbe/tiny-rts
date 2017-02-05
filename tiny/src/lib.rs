extern crate libc;
extern crate image;

#[cfg(target_os = "windows")]
mod platform {
    extern crate winapi;
    extern crate kernel32;
    extern crate user32;
    extern crate shell32;
    extern crate gdi32;

    mod windows;
    pub use self::windows::*;
}

mod bitmap;
pub use bitmap::*;

use std::cell::RefCell;
use std::result::Result;
use std::cmp;
use std::time::{Instant, Duration};
use std::thread;
use std::fmt;


#[derive(Copy, Clone, PartialEq)]
pub enum Key {
    Left = 37,
    Right = 39,
    Up = 38,
    Down = 40,
    Space = 32,
    Escape = 27,
    Ctrl = 17,
    Shift = 16,
    Enter = 13,
    Backspace = 8,
    Period = 188,
    Point = 190,
    Minus = 189,
    Num0 = 48,
    Num1 = 49,
    Num2 = 50,
    Num3 = 51,
    Num4 = 52,
    Num5 = 53,
    Num6 = 54,
    Num7 = 55,
    Num8 = 56,
    Num9 = 57,
    A = 65,
    B = 66,
    C = 67,
    D = 68,
    E = 69,
    F = 70,
    G = 71,
    H = 72,
    I = 73,
    J = 74,
    K = 75,
    L = 76,
    M = 77,
    N = 78,
    O = 79,
    P = 80,
    Q = 81,
    R = 82,
    S = 83,
    T = 84,
    U = 85,
    V = 86,
    W = 87,
    X = 88,
    Y = 89,
    Z = 90
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
      Color { rgba: (a as u32) << 24 | (r as u32) << 16 | (g as u32) << 8 | b as u32 }
   }

   pub fn red(&self) -> u8 {
      ((self.rgba >> 16) & 0xff) as u8
   }

   pub fn green(&self) -> u8 {
      ((self.rgba >> 8) & 0xff) as u8
   }

   pub fn blue(&self) -> u8 {
      (self.rgba & 0xff) as u8
   }

   pub fn alpha(&self) -> u8 {
      ((self.rgba >> 24) & 0xff) as u8
   }
}

pub struct Font {
    pub bitmap: Bitmap,
    pub char_width: i32,
    pub char_height: i32,
}

impl Font {
   pub fn new(bitmap: Bitmap, char_width: u32, char_height: u32) -> Font {
      Font {
         bitmap: bitmap,
         char_width: char_width as i32,
         char_height: char_height as i32,
      }
   }
}

struct Palette {
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
   fn clip_reset(&self);
   fn clip_set(&self, rect: Rect);

   fn clear(&self, color: u8);

   fn pixel(&self, x: i32, y: i32, color: u8);
   
   fn line(&self, x1: i32, y1: i32, x2: i32, y2: i32, color: u8);

   fn rect_stroke(&self, rect: Rect, color: u8);
   fn rect_fill(&self, rect: Rect, color: u8);

   fn blit(&self, x: i32, y: i32, source: &Bitmap, source_rect: Rect, flags: u32, color: u8);

   fn text(&self, x: i32, y: i32, text: &str, color: u8, font: &Font);
}

pub trait Application : Sized {
   fn new(ctx: &Context) -> Self;

   fn step(&mut self, ctx: &Context) -> bool { !ctx.key_pressed(Key::Escape) }
   fn paint(&self, painter: &Painter);
}


use std::sync::atomic::{AtomicBool, ATOMIC_BOOL_INIT};
static IS_TINY_CONTEXT_ALIVE: AtomicBool = ATOMIC_BOOL_INIT;


pub const TRANSPARENT: u8 = 0;
pub const BLACK: u8 = 1;
pub const WHITE: u8 = 2;


pub struct Context {
   palette: RefCell<Palette>,
   canvas: Bitmap,
   window: platform::Window,

   pub show_performance: bool,

   pub frame_time: f64,
   pub step_time: f64,
   pub paint_time: f64,
   pub blit_time: f64,
}

impl Context {
   fn new(window: platform::Window, config: &Config) -> Context {
      Context {
         palette: RefCell::new(Palette::new()),
         canvas: Bitmap::new(config.width, config.height),
         window: window,
         show_performance: false,
         frame_time: 0.0,
         step_time: 0.0,
         paint_time: 0.0,
         blit_time: 0.0,
      }
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

   // Initialize the application
   let mut app = T::new(&context);
   context.window.show();

   let target_frame_time = 33_333_333u32; // An fps of 30Hz

   // Main loop
   'main: loop {
      let frame_now = Instant::now();

      // Handle messages
      if !context.window.pump() {
         break;
      }

      {  // Step the application
         let step_now = Instant::now();
         
         if !app.step(&mut context) {
             break 'main;
         }

         context.step_time = to_milisec(step_now.elapsed());
      }

      {  // Let the application paint to the canvas
         let paint_now = Instant::now();

         let p = BitmapPainter::new(&mut context.canvas);
         app.paint(&p);

         context.paint_time = to_milisec(paint_now.elapsed());
      }

      {  // Blit canvas to the window
         let blit_now = Instant::now();
         context.window.paint(&context.canvas, &context.palette.borrow_mut().colors);
         context.blit_time = to_milisec(blit_now.elapsed());
      }

      let frame_duration = frame_now.elapsed();
      context.frame_time = to_milisec(frame_duration);

      // Sleep to force the frame time to 33ms
      if frame_duration.as_secs() == 0 && frame_duration.subsec_nanos() < target_frame_time {
         thread::sleep(Duration::new(0, target_frame_time - frame_duration.subsec_nanos()));
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
