
extern crate gl_generator;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::path::Path;

fn generate_gl_bindings() {
    let dest = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dest).join("gl_bindings.rs");
    let mut file = File::create(&path).unwrap();

    Registry::new(Api::Gl, (1, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

fn main() {
   generate_gl_bindings();
}
