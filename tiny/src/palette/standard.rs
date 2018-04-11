
use super::super::*;

pub const TRANSPARENT: u8 = 0;
pub const PINKISH_TAN: u8 = 1;
pub const ORANGEY_RED: u8 = 2;
pub const ROUGE: u8 = 3;
pub const STRONG_PINK: u8 = 4;
pub const BUBBLEGUM_PINK: u8 = 5;
pub const PINK_PURPLE: u8 = 6;
pub const WARM_PURPLE: u8 = 7;
pub const BURGUNDY: u8 = 8;
pub const NAVY_BLUE: u8 = 9;
pub const BLUE_PURPLE: u8 = 10;
pub const MEDIUM_BLUE: u8 = 11;
pub const AZURE: u8 = 12;
pub const ROBINS_EGG: u8 = 13;
pub const BLUE_GREEN: u8 = 14;
pub const DARK_AQUA: u8 = 15;
pub const DARK_FOREST_GREEN: u8 = 16;
pub const BLACK: u8 = 17;
pub const CHARCOAL_GREY: u8 = 18;
pub const GREYISH_PURPLE: u8 = 19;
pub const LIGHT_PERIWINKLE: u8 = 20;
pub const WHITE: u8 = 21;
pub const GREENISH_GREY: u8 = 22;
pub const MEDIUM_GREY: u8 = 23;
pub const BROWN: u8 = 24;
pub const UMBER: u8 = 25;
pub const YELLOWISH_ORANGE: u8 = 26;
pub const YELLOWISH: u8 = 27;
pub const PEA_SOUP: u8 = 28;
pub const MUD_GREEN: u8 = 29;
pub const KELLEY_GREEN: u8 = 30;
pub const TOXIC_GREEN: u8 = 31;
pub const BRIGHT_TEAL: u8 = 32;

pub fn create_palette() -> Palette {
   Palette {
      colors: vec![
         Color { rgba: 0x00000000 },
         Color { rgba: 0xff90a0d6 },
         Color { rgba: 0xff1e3bfe },
         Color { rgba: 0xff322ca1 },
         Color { rgba: 0xff7a2ffa },
         Color { rgba: 0xffda9ffb },
         Color { rgba: 0xfff71ce6 },
         Color { rgba: 0xff7c2f99 },
         Color { rgba: 0xff1f0147 },
         Color { rgba: 0xff551105 },
         Color { rgba: 0xffec024f },
         Color { rgba: 0xffcb692d },
         Color { rgba: 0xffeea600 },
         Color { rgba: 0xffffeb6f },
         Color { rgba: 0xff9aa208 },
         Color { rgba: 0xff6a662a },
         Color { rgba: 0xff193606 },
         Color { rgba: 0xff000000 },
         Color { rgba: 0xff57494a },
         Color { rgba: 0xffa47b8e },
         Color { rgba: 0xffffc0b7 },
         Color { rgba: 0xffffffff },
         Color { rgba: 0xff9cbeac },
         Color { rgba: 0xff707c82 },
         Color { rgba: 0xff1c3b5a },
         Color { rgba: 0xff0765ae },
         Color { rgba: 0xff30aaf7 },
         Color { rgba: 0xff5ceaf4 },
         Color { rgba: 0xff00959b },
         Color { rgba: 0xff046256 },
         Color { rgba: 0xff3b9611 },
         Color { rgba: 0xff13e151 },
         Color { rgba: 0xffccfd08 },
      ],
   }
}

pub fn names() -> Vec<String> {
   vec![
      String::from("Transparent"),
      String::from("Pinkish Tan"),
      String::from("Orangey Red"),
      String::from("Rouge"),
      String::from("Strong Pink"),
      String::from("Bubblegum Pink"),
      String::from("Pink/Purple"),
      String::from("Warm Purple"),
      String::from("Burgundy"),
      String::from("Navy Blue"),
      String::from("Blue/Purple"),
      String::from("Medium Blue"),
      String::from("Azure"),
      String::from("Robin’s Egg"),
      String::from("Blue/Green"),
      String::from("Dark Aqua"),
      String::from("Dark Forest Green"),
      String::from("Black"),
      String::from("Charcoal Grey"),
      String::from("Greyish Purple"),
      String::from("Light Periwinkle"),
      String::from("White"),
      String::from("Greenish Grey"),
      String::from("Medium Grey"),
      String::from("Brown"),
      String::from("Umber"),
      String::from("Yellowish Orange"),
      String::from("Yellowish"),
      String::from("Pea Soup"),
      String::from("Mud Green"),
      String::from("Kelley Green"),
      String::from("Toxic Green"),
      String::from("Bright Teal"),
   ]
}
