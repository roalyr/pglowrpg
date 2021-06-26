use crate::data_ops::get_layerpack;
use game_data_codec::LayerPack;
use io_ops::readron::commands;

//Common shared data for all the functions
pub struct GameData {
	//World
	pub lp: LayerPack,
	//Commands
	pub commands: commands::Stuff,
	pub commands_vec: Vec<String>,
	//Coordinates
	//+x is east, +y is north
	pub x: u32,
	pub y: u32,
	pub index: usize,
	//Temporary working variables
	//World data abs values
	pub temp: i32,
	pub rain: u32,
	pub elev: u32,
	pub water: u16,
	pub biome: u8,
	pub bioreg_id: u32,
	pub georeg_id: u32,
	pub river_id: u32,
	pub river_width: u16,
	pub river_element: u16,
	pub river_upstream: u16,
	pub river_downstream: u16,
	//Other in-game variables
	pub map_render_size: usize,
}

pub struct GameStrings {
	//Temporary value
	pub s: String,
	//Basic strings
	pub coord_str: String,
	pub temp_str: String,
	pub biome_str: String,
	pub bioreg_id_str: String,
	pub georeg_id_str: String,
	pub rain_str: String,
	pub elev_str: String,
	pub water_str: String,
	pub river_str: String,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ INITS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn init_gd(
	gs: &GameStrings,
	commands: commands::Stuff,
) -> Option<GameData> {
	let lp = match get_layerpack(&gs) {
		Some(lp) => lp,
		_ => return None,
	};
	let x = lp.wi.map_size / 2;
	let y = x;
	let index = lp.index.get(x, y);

	let gd = GameData {
		//Move previously cloned structs here
		lp,
		//Commands
		commands,
		commands_vec: Vec::new(),
		//Coordinates
		x,
		y,
		index,
		//Temporary working variables
		//World data abs values
		temp: 0,
		rain: 0,
		elev: 0,
		water: 0,
		biome: 0,
		bioreg_id: 0,
		georeg_id: 0,
		river_id: 0,
		river_width: 0,
		river_element: 0,
		river_upstream: 0,
		river_downstream: 0,
		//Other in-game variables
		map_render_size: 13,
	};
	Some(gd)
}
//Strings for printing
pub fn init_gs() -> GameStrings {
	GameStrings {
		//Temporary string
		s: String::new(),
		//Basic strings
		coord_str: String::new(),
		temp_str: String::new(),
		biome_str: String::new(),
		bioreg_id_str: String::new(),
		georeg_id_str: String::new(),
		rain_str: String::new(),
		elev_str: String::new(),
		water_str: String::new(),
		river_str: String::new(),
	}
}
