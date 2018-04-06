
mod gl {
	include!(concat!(env!("OUT_DIR"), "/gl_bindings.rs"));
}

extern crate glutin;

use self::glutin::GlContext;
use std::boxed::Box;
use super::super::*;

pub struct Window {
   events_loop: Box<glutin::EventsLoop>,
   window: Box<glutin::GlWindow>,

   canvas_buffer: Vec<u32>,
   pub key_state: [bool; 256],
   pub key_delta: [bool; 256],
   canvas_width: u32,
   canvas_height: u32,
   window_width: u32,
   window_height: u32,
}

impl Window {
	pub fn new(config: &Config) -> Result<Window, String> {
      let window_width = config.width * config.scale;
      let window_height = config.height * config.scale;

      let mut canvas_buffer: Vec<u32> = Vec::new();
      canvas_buffer.resize((config.width * config.height) as usize, 0 as u32);

		let events_loop = Box::new(glutin::EventsLoop::new());
		let window = glutin::WindowBuilder::new()
			.with_dimensions(window_width, window_height)
			.with_title(config.title.to_string())
			.with_visibility(true);

		let context = glutin::ContextBuilder::new();

		let gl_window = match glutin::GlWindow::new(window, context, &events_loop) {
			Ok(win) => Box::new(win),
			Err(_) => return Err("Could not create OpenGL Window".to_string()),
		};

      Ok(Window { 
      	events_loop: events_loop,
      	window: gl_window,

         canvas_buffer: canvas_buffer,
         key_state: [false; 256],
         key_delta: [false; 256],
         canvas_width: config.width,
         canvas_height: config.height,
         window_width: window_width,
         window_height: window_height,
      })
	}

   pub fn show(&self) {
   	self.window.show();
   }

   pub fn hide(&self) {
   	self.window.hide();
   }

   pub fn paint(&mut self, _bitmap: &Bitmap, _palette_colors: &Vec<Color>) {
   }

   pub fn pump(&mut self) -> bool {
   	let mut running = true;

   	let window = &self.window;

      self.events_loop.poll_events(|event| {
			match event {
            glutin::Event::WindowEvent{ event, .. } => match event {
            	glutin::WindowEvent::Closed => running = false,
            	glutin::WindowEvent::Resized(w, h) => window.resize(w, h),
            	_ => ()
         	},
         	_ => ()
         }
      });

   	running
   }
}