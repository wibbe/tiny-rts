
use super::super::*;

pub const BLACK: u8 = 1;
pub const VALHALLA: u8 = 2;
pub const LOULOU: u8 = 3;
pub const OILED_CEDAR: u8 = 4;
pub const ROPE: u8 = 5;
pub const TAHITI_GOLD: u8 = 6;
pub const TWINE: u8 = 7;
pub const PANCHO: u8 = 8;
pub const GOLDEN_FIZZ: u8 = 9;
pub const ATLANTIS: u8 = 10;
pub const CHRISTI: u8 = 12;
pub const ELF_GREEN: u8 = 12;
pub const DELL: u8 = 13;
pub const VERDIGRIS: u8 = 14;
pub const OPAL: u8 = 15;
pub const DEEP_KOAMARU: u8 = 16;
pub const VENICE_BLUE: u8 = 17;
pub const ROYAL_BLUE: u8 = 18;
pub const CORNFLOWER: u8 = 19;
pub const VIKING: u8 = 20;
pub const LIGHT_STEEL_BLUE: u8 = 21;
pub const WHITE: u8 = 22;
pub const HEATHER: u8 = 23;
pub const TOPAZ: u8 = 24;
pub const DIM_GRAY: u8 = 25;
pub const SMOKEY_ASH: u8 = 26;
pub const CLAIRVOYANT: u8 = 27;
pub const BROWN: u8 = 28;
pub const MANDY: u8 = 29;
pub const PLUM: u8 = 30;
pub const RAIN_FOREST: u8 = 31;
pub const STINGE: u8 = 32;

pub fn create_palette() -> Palette {
   Palette {
      colors: vec![
         Color::new(0, 0, 0, 0),
         Color::new(0, 0, 0, 255),
         Color::new(34, 32, 52, 255),
         Color::new(69, 40, 60, 255),
         Color::new(102, 57, 49, 255),
         Color::new(143, 86, 59, 255),
         Color::new(223, 113, 38, 255),
         Color::new(217, 160, 102, 255),
         Color::new(238, 195, 154, 255),
         Color::new(251, 242, 54, 255),
         Color::new(153, 229, 80, 255),
         Color::new(106, 190, 48, 255),
         Color::new(55, 148, 110, 255),
         Color::new(75, 105, 47, 255),
         Color::new(82, 75, 36, 255),
         Color::new(50, 60, 57, 255),
         Color::new(63, 63, 116, 255),
         Color::new(48, 96, 130, 255),
         Color::new(91, 110, 225, 255),
         Color::new(99, 155, 255, 255),
         Color::new(95, 205, 228, 255),
         Color::new(203, 219, 252, 255),
         Color::new(255, 255, 255, 255),
         Color::new(155, 173, 183, 255),
         Color::new(132, 126, 135, 255),
         Color::new(105, 106, 106, 255),
         Color::new(89, 86, 82, 255),
         Color::new(118, 66, 138, 255),
         Color::new(172, 50, 50, 255),
         Color::new(217, 87, 99, 255),
         Color::new(215, 123, 186, 255),
         Color::new(143, 151, 74, 255),
         Color::new(138, 111, 48, 255),
      ],
   }
}

pub fn names() -> Vec<String> {
   vec![
      String::from("Transparent"),
      String::from("Black"),
      String::from("Valhalla"),
      String::from("Loulou"),
      String::from("Oiled cedar"),
      String::from("Rope"),
      String::from("Tahiti gold"),
      String::from("Twine"),
      String::from("Pancho"),
      String::from("Golden fizz"),
      String::from("Atlantis"),
      String::from("Christi"),
      String::from("Elf green"),
      String::from("Dell"),
      String::from("Verdigris"),
      String::from("Opal"),
      String::from("Deep koamaru"),
      String::from("Venice blue"),
      String::from("Royal blue"),
      String::from("Cornflower"),
      String::from("Viking"),
      String::from("Light steel blue"),
      String::from("White"),
      String::from("Heather"),
      String::from("Topaz"),
      String::from("Dim gray"),
      String::from("Smokey ash"),
      String::from("Clairvoyant"),
      String::from("Brown"),
      String::from("Mandy"),
      String::from("Plum"),
      String::from("Rain forest"),
      String::from("Stinge"),
   ]
}
