
extern crate gl_generator;
extern crate image;

use gl_generator::{Registry, Api, Profile, Fallbacks, GlobalGenerator};
use std::env;
use std::fs::File;
use std::io::{Write, LineWriter};
use std::path::Path;

use image::GenericImage;

fn generate_gl_bindings() {
    let dest = env::var("OUT_DIR").unwrap();
    let path = Path::new(&dest).join("gl_bindings.rs");
    let mut file = File::create(&path).unwrap();

    Registry::new(Api::Gl, (1, 1), Profile::Core, Fallbacks::All, [])
        .write_bindings(GlobalGenerator, &mut file)
        .unwrap();
}

fn generate_default_font() {
   let out_dir = env::var("OUT_DIR").unwrap();
   let out_path = Path::new(&out_dir).join("default_font_data.rs");
   let in_path = Path::new("res/default.png");

   let img = image::open(in_path).unwrap();
   let (w, h) = img.dimensions();

   let mut data: Vec<u8> = Vec::new();
   let mut current = 0;
   let mut count = 0;

   println!("cargo:rerun-if-changed={}", in_path.display());

   for (_, _, pixel) in img.pixels() {
      let pixel = if pixel[3] > 0 { 1 } else { 0 };
      current = (current << 1) | pixel;

      count += 1;
      if count == 8 {
         data.push(current as u8);

         count = 0;
         current = 0;
      }
   }

   if count > 0 {
      data.push(current);
   }

   // Generate output file
   let out = File::create(&out_path).unwrap();
   let mut out = LineWriter::new(out);

   writeln!(out, "").unwrap();
   writeln!(out, "pub const DEFAULT_FONT_WIDTH: u32 = {};", w).unwrap();
   writeln!(out, "pub const DEFAULT_FONT_HEIGHT: u32 = {};", h).unwrap();
   writeln!(out, "pub const DEFAULT_CHAR_WIDTH: u32 = {};", 4).unwrap();
   writeln!(out, "pub const DEFAULT_CHAR_HEIGHT: u32 = {};", 7).unwrap();
   writeln!(out, "pub const DEFAULT_LINE_HEIGHT: u32 = {};", 9).unwrap();
   writeln!(out, "pub static DEFAULT_FONT_DATA: [u8; {}] = [", data.len()).unwrap();

   let mut count = 0;
   for pixel in data {
      if count == 0 {
         out.write(b"  ").unwrap();
      }

      write!(out, " {}, ", pixel).unwrap();
      count += 1;

      if count == 8 {
         count = 0;
         out.write(b"\n").unwrap();
      }
   }

   writeln!(out, "];").unwrap();
}

fn main() {
   generate_gl_bindings();
   generate_default_font();
}
