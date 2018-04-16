
use super::*;

#[derive(Clone)]
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