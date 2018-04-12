
extern crate tiny;

mod cmd;
mod game;

use tiny::*;
use tiny::font;
use tiny::palette::dawn_bringer as pal;



struct App {
   game: Box<game::Game>,
   font: Font,
   show_profiling: bool,
   mouse_pos: (u32, u32),
}


impl Application for App {
   fn new(ctx: &mut tiny::Context) -> Result<App, String> {
      ctx.set_palette(pal::create_palette());

      Ok(App {
         game: Box::new(game::Game::new()),
         font: font::default_font(),
         show_profiling: false,
         mouse_pos: (0, 0),
      })
   }
   
   fn step(&mut self, ctx: &tiny::Context) -> bool {
      self.mouse_pos = ctx.mouse_position();

      if ctx.mouse_pressed(tiny::Mouse::Left) {
         println!("Left Mouse Clicked");
      }

      if ctx.key_pressed(tiny::Key::F1) {
         self.show_profiling = !self.show_profiling;
      }

      !ctx.key_down(tiny::Key::Escape)
   }

   fn paint(&self, ctx: &tiny::Context, painter: &tiny::Painter) {
      painter.clear(pal::BLACK);

      let names = pal::names();

      let bw = 80;
      let bh = 25;
      let mut x = 0;
      let mut y = 0;      
      for color in 1..33 {
         let r = Rect::new_size(x, y, bw, bh);
         let txt = self.font.measure(&names[color]);

         painter.clip(Some(r));
         painter.rect_fill(r, color as u8);

         let text_color = if color as u8 == pal::WHITE { pal::BLACK } else { pal::WHITE };

         let txt_x = r.width() / 2 - txt.width() / 2;
         let txt_y = r.height() / 2 - txt.height() / 2;
         painter.text(r.left + txt_x, r.top + txt_y, &names[color], text_color, &self.font);

         x += bw;
         if x >= 320 {
            x = 0;
            y += bh;
         }
      }

      if self.show_profiling {
         ctx.draw_timing(painter, &self.font, pal::VALHALLA, pal::WHITE);
      }
   }
}

fn main() {
   if let Err(err) = tiny::run::<App>("Tiny RTS", 320, 200, 3) {
      println!("Error: {}", err);
   }
}