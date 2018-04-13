
use tiny;
use tiny::{Rect, Font, Context, Painter};
use std::rc::{Rc};
use std::cell::{RefCell};
use std::collections::HashMap;

#[derive(Clone, Debug)]
pub struct Var(Rc<RefCell<i32>>);

impl Var {
   fn new() -> Var {
      Var(Rc::new(RefCell::new(0)))
   }

   pub fn set(&self, value: i32) {
      *self.0.borrow_mut() = value;
   }

   pub fn get(&self) -> i32 {
      *self.0.borrow()
   }
}

pub struct Function {

}


pub struct Cmd {
   vars: RefCell<HashMap<String, Var>>,
   input: RefCell<Vec<char>>,
   cursor: RefCell<u32>,
}

impl Cmd {
   pub fn new() -> Cmd {
      Cmd {
         vars: RefCell::new(HashMap::new()),
         input: RefCell::new(Vec::new()),
         cursor: RefCell::new(0),
      }
   }

   pub fn register_var(&self, name: &str) -> Result<Var, String> {
      let mut vars = self.vars.borrow_mut();

      let name = name.to_string();
      if vars.contains_key(&name) {
         return Err("Variable already pressent".to_string());
      }

      let var = Var::new();
      vars.insert(name, var.clone());
      Ok(var.clone())
   }

   pub fn register_func(&self, _name: String) -> Result<(), String> {
      Ok(())
   }

   pub fn exec(&self, _line: String) -> Result<(), String> {
      Ok(())
   }

   pub fn step(&self, ctx: &Context) {
      {  // Handle text input
         let text_input = ctx.text_input();
         if !text_input.is_empty() {
            let mut input = self.input.borrow_mut();
            for ch in text_input.iter() {
               input.push(*ch);
            }
         }
      }

      if ctx.key_pressed(tiny::Key::Back) {
         self.input.borrow_mut().pop();
      }
   }

   pub fn paint(&self, painter: &Painter, font: &Font, background_color: u8, foreground_color: u8) {
      let (w, h) = painter.size();
      let h = font.line_height * 10;

      let background_rect = Rect::new(0, 0, w as i32, h);
      let input_rect = Rect::new(0, h - font.line_height, w as i32, h);

      painter.clip(Some(background_rect));
      painter.rect_fill(Rect::new(0, 0, w as i32, h), background_color);

      let mut x_input = 2;
      let mut y_input = h - font.line_height;

      for ch in self.input.borrow().iter() {
         let (dx, dy) = painter.char(x_input, y_input, *ch, foreground_color, font);
         x_input += dx;
      }

      painter.clip(None);
   }
}