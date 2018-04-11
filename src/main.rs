
extern crate tiny;

mod cmd;

use tiny::*;
use tiny::palette::standard as pal;

use std::path::Path;

struct App {
   font: Font,
   mouse_pos: (u32, u32),
}


impl Application for App {
   fn new(ctx: &mut tiny::Context) -> Result<App, String> {
      let font_bitmap = tiny::Bitmap::load(ctx, Path::new("res/font.png")).unwrap();

      ctx.set_palette(pal::create_palette());

      Ok(App {
         font: tiny::Font::new(font_bitmap, 4, 7),
         mouse_pos: (0, 0),
      })
   }
   
   fn step(&mut self, ctx: &tiny::Context) -> bool {
      self.mouse_pos = ctx.mouse_position();

      !ctx.key_down(tiny::Key::Escape)
   }

   fn paint(&self, painter: &tiny::Painter) {
      painter.clear(pal::BLACK);

      let names = pal::names();

      let mut x = 0;
      let mut y = 0;      
      for color in 0..32 {
         let r = Rect::new_size(x, y, 40, 40);
         let txt = self.font.measure(&names[color]);

         painter.clip(Some(r));
         painter.rect_fill(r, color as u8);

         let text_color = if color as u8 == pal::WHITE { pal::BLACK } else { pal::WHITE };
         painter.text(r.left + 1, r.top + 1, &names[color], text_color, &self.font);

         x += 40;
         if x >= 320 {
            x = 0;
            y += 40;
         }
      }
   }
}

fn main() {
   if let Err(err) = tiny::run::<App>("Tiny RTS", 320, 200, 3) {
      println!("Error: {}", err);
   }
}