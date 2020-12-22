pub mod erosion_ops;
pub mod mapping_ops;
pub mod path_ops;
pub mod probability_ops;
pub mod vector_ops;
pub mod waterbody_ops;
pub mod width_ops;

use crate::array_ops::noise_maps;
use codec::LayerPack;
use constants_world::*;
use io_ops::toml::strings;
use ui::progress;
use units::translate;

use line_drawing::BresenhamCircle;

//▒▒▒▒▒▒▒▒▒▒▒▒ STRUCTURES ▒▒▒▒▒▒▒▒▒▒▒▒▒
//river entry stores all the relevant river data
#[derive(Clone)]
pub struct RiverEntry {
	river_id: u16,              //global river id value
	path_array: Vec<path::Pos>, //cells coords
	width: u16,                 //river width value
	source: (usize, usize),     //value for river source
	end: (usize, usize),        //value for river end
}

//for river width generation only
#[derive(Clone, Debug)]
pub struct WidthEntry {
	river_id_downstr: u16,
	width_new: u16,
}

//for river erosion generation only
#[derive(Copy, Clone, Debug)]
pub struct ErosionEntry {
	river_id_downstr: u16,
	terrain_diff: u16,
}

//structure to hold the lists and queues
pub struct RiversPaths {
	list: Vec<RiverEntry>,
	width_queue: Vec<WidthEntry>,
	erosion_queue: Vec<ErosionEntry>,
}

//global rivergen param structure for data transfer
pub struct RgParams {
	//temporary value storage
	river_id: u16,    //check for overflow?
	river_width: u16, //saturated
	river_source: (usize, usize),
	river_end: (usize, usize),
	river_est_number: usize,
	river_count_number: usize,
	upstream_neighbor: (usize, usize),
	//nested structures
	dv: path::DirVector,
	rivers_paths: RiversPaths,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MAIN ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn get(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
) {
	//initiate the parameter structure
	let mut rg = RgParams {
		//temporary value storage
		river_id: INIT_ID_U16,
		river_width: RIVER_MIN_WIDTH,
		river_source: (0, 0),
		river_end: (0, 0),
		river_est_number: 0,
		river_count_number: 0,
		upstream_neighbor: (0, 0),
		//nested structures
		dv: path::DirVector {
			x0: 0,
			y0: 0,
			x1: lp.wi.map_size / 2, //make end in the center
			y1: lp.wi.map_size / 2, //make end in the center
			r: ONE_USIZE,
			hit: false,
			path_heuristic: RIVER_HEUR_INIT,
		},
		rivers_paths: RiversPaths {
			list: Vec::new(),
			width_queue: Vec::new(),
			erosion_queue: Vec::new(),
		},
	};
	//perform rivergen
	rg.estimate_sources_number(lp, wg_str);
	rg.set_paths(lp, wg_str);
	rg.map_paths(lp);
	rg.map_width(lp);
	rg.map_erosion(lp);
}
