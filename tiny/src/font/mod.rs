
include!(concat!(env!("OUT_DIR"), "/default_font_data.rs"));


use super::*;
/*
pub fn create_default_font() -> Font {

}
*/

pub fn default_font_bitmap() -> Bitmap {
	bitmap::Bitmap::from_bitmask(&DEAFULT_FONT_DATA, DEFAULT_FONT_WIDTH, DEFAULT_FONT_HEIGHT)
}