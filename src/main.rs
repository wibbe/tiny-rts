
extern crate tiny;

use tiny::*;
use std::path::Path;

struct App {
   font: Font,
}


impl tiny::Application for App {
   fn new(ctx: &tiny::Context) -> App {
      let font_bitmap = Bitmap::load(ctx, Path::new("res/font.png")).unwrap();

      App {
         font: Font::new(font_bitmap, 4, 7)
      }
   }
   
   fn step(&mut self, ctx: &tiny::Context) -> bool {
      !ctx.key_down(tiny::Key::Escape)
   }

   fn paint(&self, painter: &tiny::Painter) {
      painter.clear(tiny::BLACK);
      painter.text(135, 90, "Hello World!", WHITE, &self.font);
   }
}

fn main() {
   if let Err(err) = tiny::run::<App>("Tiny RTS", 320, 200, 3) {
      println!("Error: {}", err);
   }
}