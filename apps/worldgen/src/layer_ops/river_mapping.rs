pub mod erosion_ops;
pub mod mapping_ops;
pub mod path_ops;
pub mod probability_ops;
pub mod vector_ops;
pub mod waterbody_ops;
pub mod width_ops;

use constants::generic as cg;
use constants::world as cw;
use game_data_codec::LayerPack;

//▒▒▒▒▒▒▒▒▒▒▒▒ STRUCTURES ▒▒▒▒▒▒▒▒▒▒▒▒▒
//River entry stores all the relevant river data.
#[derive(Clone)]
pub struct RiverEntry {
	river_id: u32,
	path_array: Vec<pathfinding::Pos>,
	width: u16,
	source: (usize, usize),
	end: (usize, usize),
}

//For river width generation only.
#[derive(Clone, Debug)]
pub struct WidthEntry {
	river_id_downstr: u32,
	width_new: u16,
}

//For river erosion generation only.
#[derive(Copy, Clone, Debug)]
pub struct ErosionEntry {
	river_id_downstr: u32,
	terrain_diff: u16,
}

//Structure to hold the lists and queues.
pub struct RiversPaths {
	list: Vec<RiverEntry>,
	width_queue: Vec<WidthEntry>,
	erosion_queue: Vec<ErosionEntry>,
}

//Global rivergen param structure for data transfer.
pub struct RgParams {
	river_id: u32,
	river_width: u16,
	river_source: (usize, usize),
	river_end: (usize, usize),
	river_est_number: usize,
	river_count_number: usize,
	upstream_neighbor: (usize, usize),
	water_bodies_present: bool,
	dv: pathfinding::DirVector,
	rivers_paths: RiversPaths,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MAIN ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn get(lp: &mut LayerPack) {
	let mut rg = RgParams {
		river_id: cg::ID_MAP_MIN_U32,
		river_width: cw::RIVER_MIN_WIDTH,
		river_source: (0, 0),
		river_end: (0, 0),
		river_est_number: 0,
		river_count_number: 0,
		upstream_neighbor: (0, 0),
		water_bodies_present: false,
		dv: pathfinding::DirVector {
			x0: 0,
			y0: 0,
			x1: lp.wi.map_size / 2, //end in the center
			y1: lp.wi.map_size / 2, //end in the center
			r: cg::ONE_USIZE,
			hit: false,
			path_heuristic: cw::RIVER_HEUR_INIT,
		},
		rivers_paths: RiversPaths {
			list: Vec::new(),
			width_queue: Vec::new(),
			erosion_queue: Vec::new(),
		},
	};

	//Perform rivergen.
	rg.estimate_water_bodies(lp);
	rg.estimate_sources_number(lp);
	rg.set_paths(lp);
	rg.map_paths(lp);
	rg.map_width(lp);
	rg.map_erosion(lp);
}
