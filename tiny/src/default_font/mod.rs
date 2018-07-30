
include!(concat!(env!("OUT_DIR"), "/font_4x7_data.rs"));
include!(concat!(env!("OUT_DIR"), "/font_4x10_data.rs"));


use super::*;

pub fn font_4x7_bitmap() -> Bitmap {
	bitmap::Bitmap::from_bitmask(&FONT_4X7_DATA, FONT_4X7_WIDTH, FONT_4X7_HEIGHT)
}

pub fn font_4x7() -> Font {
	Font::new(font_4x7_bitmap(), FONT_4X7_CHAR_WIDTH, FONT_4X7_CHAR_HEIGHT, FONT_4X7_LINE_HEIGHT)
}

pub fn font_4x10_bitmap() -> Bitmap {
	bitmap::Bitmap::from_bitmask(&FONT_4X10_DATA, FONT_4X10_WIDTH, FONT_4X10_HEIGHT)
}

pub fn font_4x10() -> Font {
	Font::new(font_4x10_bitmap(), FONT_4X10_CHAR_WIDTH, FONT_4X10_CHAR_HEIGHT, FONT_4X10_LINE_HEIGHT)
}