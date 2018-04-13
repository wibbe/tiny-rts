
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
}

impl Cmd {
   pub fn new() -> Cmd {
      Cmd {
         vars: RefCell::new(HashMap::new()),
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
}