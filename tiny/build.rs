
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

fn generate_font(input_file: &str, output_file: &str, name: &str, width: u32, height: u32) {
   let out_dir = env::var("OUT_DIR").unwrap();
   let out_path = Path::new(&out_dir).join(output_file);
   let in_path = Path::new(input_file);

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
   writeln!(out, "pub const {}_WIDTH: u32 = {};", name, w).unwrap();
   writeln!(out, "pub const {}_HEIGHT: u32 = {};", name, h).unwrap();
   writeln!(out, "pub const {}_CHAR_WIDTH: u32 = {};", name, width).unwrap();
   writeln!(out, "pub const {}_CHAR_HEIGHT: u32 = {};", name, height).unwrap();
   writeln!(out, "pub const {}_LINE_HEIGHT: u32 = {};", name, height + 2).unwrap();
   writeln!(out, "pub static {}_DATA: [u8; {}] = [", name, data.len()).unwrap();

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
   generate_font("res/font-4x7.png", "font_4x7_data.rs", "FONT_4X7", 4, 7);
   generate_font("res/font-4x10.png", "font_4x10_data.rs", "FONT_4X10", 4, 10);
}
