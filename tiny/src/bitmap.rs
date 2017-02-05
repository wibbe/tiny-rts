
use super::*;

use std::cell::RefCell;
use std::ptr;
use std::path::Path;
use std::result::Result;

use image;
use image::GenericImage;

pub struct Bitmap {
   pixels: RefCell<Vec<u8>>,
   pub width: u32,
   pub height: u32,
}

impl Bitmap {
   pub fn new(w: u32, h: u32) -> Bitmap {
      let mut pixels: Vec<u8> = Vec::new();
      pixels.resize((w* h) as usize, 0 as u8);

      Bitmap {
         pixels: RefCell::new(pixels),
         width: w,
         height: h
      }
   }

   pub fn load(ctx: &Context, path: &Path) -> Result<Bitmap, String> {
      if let Ok(ref img) = image::open(path) {
         let (w, h) = img.dimensions();

         let bitmap = Bitmap::new(w, h);

         {
            let mut pixels = bitmap.pixels.borrow_mut();

            for (x, y, pixel) in img.pixels() {
               let color = ctx.palette_add(Color::new(pixel[0], pixel[1], pixel[2], pixel[3]));
               pixels[(x + y * w) as usize] = color;
            }
         }

         Ok(bitmap)
      } else {
         Err(String::from("Could not load image"))
      }
   }

   #[inline]
   pub fn pixel(&self, x: u32, y: u32) -> u8 {
      self.pixels.borrow()[(self.width * y + x) as usize]
   }
}


pub struct BitmapPainter<'a> {
   target: &'a mut Bitmap,
   clip: RefCell<Rect>,
}

impl<'a> BitmapPainter<'a> {
   pub fn new(target: &'a mut Bitmap) -> BitmapPainter {
      let w = target.width;
      let h = target.height;

      BitmapPainter {
         target: target,
         clip: RefCell::new(Rect::new_size(0, 0, w as i32, h as i32)),
      }
   }
}

impl<'a> Painter for BitmapPainter<'a> {
   fn clip_reset(&self) {
      *self.clip.borrow_mut() = Rect::new_size(0, 0, self.target.width as i32, self.target.height as i32);
   }

   fn clip_set(&self, rect: Rect) {
      *self.clip.borrow_mut() = rect.fit(0, 0, self.target.width as i32, self.target.height as i32);
   }

   fn clear(&self, color: u8) {
      unsafe {
         let len = (self.target.width * self.target.height) as usize;
         ptr::write_bytes(self.target.pixels.borrow_mut().as_mut_ptr(), color, len);
      }
   }

   fn pixel(&self, x: i32, y: i32, color: u8) {
      if self.clip.borrow().inside(x, y) {
         self.target.pixels.borrow_mut()[(x + y * self.target.width as i32) as usize] = color;
      }
   }

   fn rect_stroke(&self, rect: Rect, color: u8) {
      self.line(rect.left, rect.top, rect.right + 1, rect.top, color);
      self.line(rect.left, rect.bottom, rect.right + 1, rect.bottom, color);
      self.line(rect.left, rect.top, rect.left, rect.bottom, color);
      self.line(rect.right, rect.top, rect.right, rect.bottom, color);
   }

   fn rect_fill(&self, rect: Rect, color: u8) {
      let stride = self.target.width as isize;
      let start = (rect.top * self.target.width as i32) + rect.left;
      let len = (rect.right - rect.left) as usize;
      let mut pixels = self.target.pixels.borrow_mut();

      unsafe {
         let mut pos = start as isize;
         for _ in 0..(rect.bottom - rect.top) as i32 {
            ptr::write_bytes(pixels.as_mut_ptr().offset(pos), color, len);
            pos += stride;
         }
      }
   }

   fn line(&self, x0: i32, y0: i32, x1: i32, y1: i32, color: u8)
   {
      let sx = if x0 < x1 { 1 } else { -1 };
      let sy = if y0 < y1 { 1 } else { -1 };
      let dx = (x1 - x0).abs();
      let dy = (y1 - y0).abs();

      let mut err = dx - dy;
      let mut x = x0;
      let mut y = y0;
      
      let clip = self.clip.borrow();
      let mut pixels = self.target.pixels.borrow_mut();


      while x != x1 || y != y1 {
         if clip.inside(x, y) {
            pixels[(x + y * self.target.width as i32) as usize] = color;
         }

         let e2 = 2 * err;

         if e2 > -dy {
            err -= dy;
            x += sx;
         }
         if e2 < dx {
            err += dx;
            y += sy;
         }
      }
   }

   fn blit(&self, x0: i32, y0: i32, source: &Bitmap, source_rect: Rect, flags: u32, color: u8) {
      let clip = self.clip.borrow();

      let source_pixels = source.pixels.borrow();
      let mut target_pixels = self.target.pixels.borrow_mut();

      for y in source_rect.top..source_rect.bottom {
         for x in source_rect.left..source_rect.right {
            let target_x = x0 + (x - source_rect.left);
            let target_y = y0 + (y - source_rect.top);

            let sx = if (flags & DRAW_FLIP_H) > 0 { source.width as i32 - x } else { x };

            if clip.inside(target_x, target_y) {
               let target_idx = (target_x + target_y * self.target.width as i32) as usize;
               let source_idx = (sx + y * source.width as i32) as usize;
               let source = source_pixels[source_idx];
               let target  = target_pixels[target_idx];

               if flags & DRAW_MASK > 0 {
                  target_pixels[target_idx] = if source > 0 { color } else { target };
               } else {
                  target_pixels[target_idx] = if source > 0 { source } else { target };
               }
            }
         }
      }
   }

   fn text(&self, x: i32, y: i32, text: &str, color: u8, font: &Font) {
      let mut x_curr = x; 
      let mut y_curr = y;

      let chars_per_row = font.bitmap.width / font.char_width as u32;

      for ch in text.chars() {
         let idx = ch as u32;
         if idx < 256 {
            let ch_x = (idx % chars_per_row) as i32;
            let ch_y = (idx / chars_per_row) as i32;

            match ch {
               ' ' => x_curr += font.char_width,
               '\t' => x_curr += font.char_width,
               '\n' => {
                  x_curr = x;
                  y_curr += (font.char_height as f32 * 1.5) as i32;
               },
               _ => {
                  self.blit(x_curr, y_curr, &font.bitmap, Rect::new_size(ch_x * font.char_width, ch_y * font.char_height, font.char_width, font.char_height), DRAW_MASK, color);
                  x_curr += font.char_width;
               },
            }
         }
      }
   }
}