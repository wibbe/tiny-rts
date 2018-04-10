
extern crate tiny;

mod cmd;
mod game;

use std::path::Path;


struct App {
   font: tiny::Font,
   red:  u8,
}


impl tiny::Application for App {
   fn new(ctx: &tiny::Context) -> Result<App, String> {
      let font_bitmap = tiny::Bitmap::load(ctx, Path::new("res/font.png")).unwrap();

      Ok(App {
         font: tiny::Font::new(font_bitmap, 4, 7),
         red: ctx.palette_add(tiny::Color::new(255, 0, 0, 255)),
      })
   }
   
   fn step(&mut self, ctx: &tiny::Context) -> bool {
      !ctx.key_down(tiny::Key::Escape)
   }

   fn paint(&self, painter: &tiny::Painter) {
      painter.clear(tiny::BLACK);
      painter.text(136, 96, "Hello, World!", self.red, &self.font);
   }
}

fn main() {
   if let Err(err) = tiny::run::<App>("Tiny RTS", 320, 200, 3) {
      println!("Error: {}", err);
   }
}