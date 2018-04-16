
include!(concat!(env!("OUT_DIR"), "/default_font_data.rs"));


use super::*;

pub fn default_font_bitmap() -> Bitmap {
	bitmap::Bitmap::from_bitmask(&DEFAULT_FONT_DATA, DEFAULT_FONT_WIDTH, DEFAULT_FONT_HEIGHT)
}

pub fn default_font() -> Font {
	Font::new(default_font_bitmap(), DEFAULT_CHAR_WIDTH, DEFAULT_CHAR_HEIGHT, DEFAULT_LINE_HEIGHT)
}