
use tiny;
use tiny::{Rect, Font, Context, Painter};
use std::rc::{Rc};
use std::cell::{Cell, RefCell};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Var(Rc<Cell<i32>>);

impl Var {
   fn new(default_value: i32) -> Var {
      Var(Rc::new(Cell::new(default_value)))
   }

   #[inline]
   pub fn set(&self, value: i32) {
      self.0.set(value);
   }

   #[inline]
   pub fn get(&self) -> i32 {
      self.0.get()
   }
}

pub struct Function {

}


#[derive(Clone)]
pub struct Config {
   pub font: tiny::Font,
   pub background_color: u8,
   pub foreground_color: u8,
   pub cursor_color: u8,
   pub lines: usize,
}


pub struct Cmd {
   vars: RefCell<HashMap<String, Var>>,
   input: RefCell<Vec<char>>,
   cursor: RefCell<usize>,
   history: RefCell<Vec<String>>,
   scrolling: Cell<usize>,
   config: Config,
}

impl Cmd {
   pub fn new(config: Config) -> Cmd {
      Cmd {
         vars: RefCell::new(HashMap::new()),
         input: RefCell::new(Vec::new()),
         cursor: RefCell::new(0),
         history: RefCell::new(Vec::new()),
         scrolling: Cell::new(0),
         config: config,
      }
   }

   pub fn register_var(&self, name: &str, default_value: i32) -> Result<Var, String> {
      let mut vars = self.vars.borrow_mut();

      let name = name.to_string();
      if vars.contains_key(&name) {
         return Err("Variable already pressent".to_string());
      }

      let var = Var::new(default_value);
      vars.insert(name, var.clone());
      Ok(var.clone())
   }

   pub fn register_func(&self, _name: String) -> Result<(), String> {
      Ok(())
   }

   pub fn exec(&self, _line: String) -> Result<(), String> {
      Ok(())
   }

   pub fn echo(&self, text: String) {
      self.history.borrow_mut().push(text);

      if self.history.borrow().len() >= self.config.lines {
         self.scrolling.set(self.scrolling.get() + 1);
      }
   }

   pub fn step(&self, ctx: &Context) {
      let mut input = self.input.borrow_mut();
      let mut cursor = self.cursor.borrow_mut();

      {  // Handle text input
         let text_input = ctx.text_input();
         if !text_input.is_empty() {
            for ch in text_input.iter() {
               if input.len() == *cursor {
                  input.push(*ch);
                  *cursor += 1;
               } else {
                  input.insert(*cursor, *ch);
                  *cursor += 1;
               }
            }
         }
      }

      if ctx.key_pressed(tiny::Key::Back) && input.len() > 0 {
         if input.len() == *cursor {
            input.pop();
            *cursor -= 1;
         } else if *cursor > 0 {
            input.remove(*cursor - 1);
            *cursor -= 1;
         }
      }

      if ctx.key_pressed(tiny::Key::Home) {
         *cursor = 0;
      }

      if ctx.key_pressed(tiny::Key::End) {
         *cursor = input.len();
      }

      if ctx.key_pressed(tiny::Key::Return) {
         self.echo(format!(">{}", &input.iter().collect::<String>()));
         input.clear();
         *cursor = 0;
      }

      if ctx.key_pressed(tiny::Key::Left) {
         if *cursor > 0 {
            *cursor -= 1;
         }
      }

      if ctx.key_pressed(tiny::Key::Right) {
         if *cursor < input.len() {
            *cursor += 1;
         }
      }
   }

   pub fn paint(&self, painter: &Painter) {
      let line_height = self.config.font.line_height;
      let char_width = self.config.font.char_width;
      let char_height = self.config.font.char_height;

      let (w, h) = painter.size();
      let h = line_height * self.config.lines as i32;

      let background_rect = Rect::new(0, 0, w as i32, h + 2);

      painter.clip(None);
      painter.rect_fill(background_rect, self.config.background_color);
      
      painter.clip(Some(background_rect.tr(2, 2).grow(-4, -4)));

      let mut x_input = 2;
      let mut y_input = 2;

      for line in self.history.borrow().iter().skip(self.scrolling.get()).take(self.config.lines - 1) {
         painter.text(x_input, y_input, &line, self.config.foreground_color, &self.config.font);
         y_input += line_height;
      }

      x_input = 2;
      let (dx, _) = painter.char(x_input, y_input, '>', self.config.foreground_color, &self.config.font);
      x_input += dx;

      let mut pos = 0;
      let cursor = *self.cursor.borrow();
      for ch in self.input.borrow().iter() {
         if cursor == pos {
            painter.rect_fill(Rect::new_size(x_input, y_input, char_width, self.config.font.char_height), self.config.cursor_color);
         }

         let (dx, _) = painter.char(x_input, y_input, *ch, self.config.foreground_color, &self.config.font);
         x_input += dx;
         pos += 1;
      }

      if cursor == self.input.borrow().len() {
         painter.rect_fill(Rect::new_size(x_input, y_input, char_width, char_height), self.config.cursor_color);
      }

      painter.clip(None);
   }
}