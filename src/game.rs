
use tiny;

const MAP_WIDTH: usize = 80;
const MAP_HEIGHT: usize = 60;
const MAX_UNITS: usize = 1024;

pub struct Pos(u32, u32);

#[derive(Copy, Clone)]
pub struct Unit {
	next_free: Option<usize>,
}

impl Default for Unit {
	fn default() -> Unit {
		Unit {
			next_free: None,
		}
	}
}


#[derive(Copy, Clone)]
pub struct Cell {
	unit: Option<u32>,
}

impl Default for Cell {
	fn default() -> Cell {
		Cell {
			unit: None,
		}
	}
}


#[derive(Copy, Clone)]
pub struct Map {
	cells: [Cell; MAP_WIDTH * MAP_HEIGHT],
}

impl Default for Map {
	fn default() -> Map {
		Map {
			cells: [Default::default(); MAP_WIDTH * MAP_HEIGHT],
		}
	}
}



#[derive(Copy, Clone)]
pub struct Game {
	map: Map,
	units: [Unit; MAX_UNITS],
}

impl Game {
	pub fn new() -> Game {
		let mut game = Game {
			map: Default::default(),
			units: [Default::default(); MAX_UNITS],
		};

		for i in 0..(MAX_UNITS - 1) {
			game.units[i].next_free = Some(i + 1);
		}

		game
	}
}