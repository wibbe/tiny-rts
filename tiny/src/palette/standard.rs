
/*
      #d6a090  rgb(84.0%, 62.6%, 56.4%)   pinkish tan
      #fe3b1e  rgb(99.6%, 23.1%, 11.8%)   orangey red
      #a12c32  rgb(63.3%, 17.5%, 19.6%)   rouge
      #fa2f7a  rgb(98.0%, 18.5%, 47.7%)   strong pink
      #fb9fda  rgb(98.2%, 62.4%, 85.5%)   bubblegum pink
      #e61cf7  rgb(90.1%, 11.1%, 96.8%)   pink/purple
      #992f7c  rgb(59.9%, 18.6%, 48.7%)   warm purple
      #47011f  rgb(27.9%, 0.5%, 12.5%)    burgundy
      #051155  rgb(2.3%, 6.7%, 33.3%)  navy blue
      #4f02ec  rgb(31.2%, 0.9%, 92.3%)    blue/purple
      #2d69cb  rgb(17.7%, 41.3%, 79.5%)   medium blue
      #00a6ee  rgb(0.3%, 64.9%, 93.3%)    azure
      #6febff  rgb(43.5%, 91.9%, 99.9%)   robin’s egg
      #08a29a  rgb(3.2%, 63.6%, 60.5%)    blue/green
      #2a666a  rgb(16.7%, 40.1%, 41.5%)   dark aqua
      #063619  rgb(2.4%, 21.2%, 10.0%)    dark forest green
      #000000  rgb(0.0%, 0.0%, 0.0%)   black
      #4a4957  rgb(29.0%, 28.6%, 34.3%)   charcoal grey
      #8e7ba4  rgb(55.5%, 48.3%, 64.4%)   greyish purple
      #b7c0ff  rgb(71.7%, 75.3%, 99.8%)   light periwinkle
      #ffffff  rgb(100.0%, 100.0%, 100.0%)   white
      #acbe9c  rgb(67.4%, 74.4%, 61.2%)   greenish grey
      #827c70  rgb(50.9%, 48.8%, 44.0%)   medium grey
      #5a3b1c  rgb(35.2%, 23.3%, 11.0%)   brown
      #ae6507  rgb(68.2%, 39.6%, 2.8%)    umber
      #f7aa30  rgb(96.8%, 66.4%, 19.0%)   yellowish orange
      #f4ea5c  rgb(95.5%, 91.5%, 36.0%)   yellowish
      #9b9500  rgb(60.6%, 58.4%, 0.4%)    pea soup
      #566204  rgb(33.9%, 38.4%, 1.7%)    mud green
      #11963b  rgb(7.0%, 58.8%, 23.1%)    kelley green
      #51e113  rgb(31.7%, 88.0%, 7.6%)    toxic green
      #08fdcc  rgb(3.5%, 99.1%, 79.8%)    bright teal
*/

use super::super::*;

pub const PINKISH_TAN: u8 = 0;
pub const ORANGEY_RED: u8 = 1;
pub const ROUGE: u8 = 2;
pub const STRONG_PINK: u8 = 3;
pub const BUBBLEGUM_PINK: u8 = 4;
pub const PINK_PURPLE: u8 = 5;
pub const WARM_PURPLE: u8 = 6;
pub const BURGUNDY: u8 = 7;
pub const NAVY_BLUE: u8 = 8;
pub const BLUE_PURPLE: u8 = 9;
pub const MEDIUM_BLUE: u8 = 10;
pub const AZURE: u8 = 11;
pub const ROBINS_EGG: u8 = 12;
pub const BLUE_GREEN: u8 = 13;
pub const DARK_AQUA: u8 = 14;
pub const DARK_FOREST_GREEN: u8 = 15;
pub const BLACK: u8 = 16;
pub const CHARCOAL_GREY: u8 = 17;
pub const GREYISH_PURPLE: u8 = 18;
pub const LIGHT_PERIWINKLE: u8 = 19;
pub const WHITE: u8 = 20;
pub const GREENISH_GREY: u8 = 21;
pub const MEDIUM_GREY: u8 = 22;
pub const BROWN: u8 = 23;
pub const UMBER: u8 = 24;
pub const YELLOWISH_ORANGE: u8 = 25;
pub const YELLOWISH: u8 = 26;
pub const PEA_SOUP: u8 = 27;
pub const MUD_GREEN: u8 = 28;
pub const KELLEY_GREEN: u8 = 29;
pub const TOXIC_GREEN: u8 = 30;
pub const BRIGHT_TEAL: u8 = 31;

pub fn create_palette() -> Palette {
   Palette {
      colors: vec![
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
      String::from("pinkish tan"),
      String::from("orangey red"),
      String::from("rouge"),
      String::from("strong pink"),
      String::from("bubblegum pink"),
      String::from("pink/purple"),
      String::from("warm purple"),
      String::from("burgundy"),
      String::from("navy blue"),
      String::from("blue/purple"),
      String::from("medium blue"),
      String::from("azure"),
      String::from("robin’s egg"),
      String::from("blue/green"),
      String::from("dark aqua"),
      String::from("dark forest green"),
      String::from("black"),
      String::from("charcoal grey"),
      String::from("greyish purple"),
      String::from("light periwinkle"),
      String::from("white"),
      String::from("greenish grey"),
      String::from("medium grey"),
      String::from("brown"),
      String::from("umber"),
      String::from("yellowish orange"),
      String::from("yellowish"),
      String::from("pea soup"),
      String::from("mud green"),
      String::from("kelley green"),
      String::from("toxic green"),
      String::from("bright teal"),
   ]
}
