use crate::commands_ops::commands_init;
use crate::data_ops::get_layerpack;
use lib_game_data_codec as gdc;
use lib_game_data_codec::LayerPack;

pub struct GameData {
	pub lp: LayerPack,
	pub commands_struct: commands_init::CommandsStrings,
	pub commands_list: Vec<String>,
	pub x: u32,
	pub y: u32,
	pub index: usize,
	pub map_render_size: usize,
}

pub struct WorldData {
	pub x: u32,
	pub y: u32,
	pub index: usize,
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
	pub flora: Vec<gdc::entities::PlantGroup>,
}

pub struct GameStrings {
	// Temporary string to be used as a placeholder.
	pub s: String,
	// Basic strings from world data.
	pub coord_str: String,
	pub temp_str: String,
	pub biome_str: String,
	pub bioreg_id_str: String,
	pub georeg_id_str: String,
	pub rain_str: String,
	pub elev_str: String,
	pub water_str: String,
	pub river_str: String,
	pub flora_str: String,
}

////////////////////////////////////////////////////////////////////////////////
// INIT
pub fn init_gd(
	gs: &GameStrings,
	commands_struct: commands_init::CommandsStrings,
) -> Option<GameData> {
	// Initializing some data.
	let lp = match get_layerpack(&gs) {
		Some(lp) => lp,
		_ => return None,
	};
	// Temporary values for player starting pos.
	let x = lp.wi.map_size / 2;
	let y = x;
	let index = lp.index.get(x, y);

	let gd = GameData {
		lp,
		commands_struct,
		commands_list: Vec::new(),
		x,
		y,
		index,
		map_render_size: 6,
	};
	Some(gd)
}

// World data abs values.
pub fn init_wd() -> WorldData {
	WorldData {
		x: 0,
		y: 0,
		index: 0,
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
		flora: Vec::new(),
	}
}

// Strings for printing.
pub fn init_gs() -> GameStrings {
	GameStrings {
		// Temporary string
		s: String::new(),
		// Basic strings (generated from world data)
		coord_str: String::new(),
		temp_str: String::new(),
		biome_str: String::new(),
		bioreg_id_str: String::new(),
		georeg_id_str: String::new(),
		rain_str: String::new(),
		elev_str: String::new(),
		water_str: String::new(),
		river_str: String::new(),
		flora_str: String::new(),
	}
}
