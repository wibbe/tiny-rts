
extern crate tiny;

use tiny::*;
use tiny::palette::dawn_bringer as pal;

use std::path::Path;

struct App {
//   font: Font,
   mouse_pos: (u32, u32),
}


impl Application for App {
   fn new(ctx: &mut tiny::Context) -> Result<App, String> {
 //     let font_bitmap = tiny::Bitmap::load(ctx, Path::new("res/font.png")).unwrap();

      ctx.set_palette(pal::create_palette());

      Ok(App {
 //        font: tiny::Font::new(font_bitmap, 4, 7),
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

      let bw = 80;
      let bh = 25;
      let mut x = 0;
      let mut y = 0;      
      for color in 0..32 {
         let r = Rect::new_size(x, y, bw, bh);
//         let txt = self.font.measure(&names[color]);

         painter.clip(Some(r));
         painter.rect_fill(r, color as u8);

//         let text_color = if color as u8 == pal::WHITE { pal::BLACK } else { pal::WHITE };

//         let txt_x = r.width() / 2 - txt.width() / 2;
//         let txt_y = r.height() / 2 - txt.height() / 2;

         //painter.text(r.left + 1, r.top + 1, &names[color], text_color, &self.font);
//         painter.text(r.left + txt_x, r.top + txt_y, &names[color], text_color, &self.font);

         x += bw;
         if x >= 320 {
            x = 0;
            y += bh;
         }
      }
   }
}

fn main() {
   if let Err(err) = tiny::run::<App>("Dawn Bringer Palette", 320, 200, 3) {
      println!("Error: {}", err);
   }
}