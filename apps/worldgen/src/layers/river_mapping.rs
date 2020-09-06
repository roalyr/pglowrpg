pub mod erosion_ops;
pub mod mapping_ops;
//pub mod mask_ops;
pub mod path_ops;
pub mod probability_ops;
pub mod vector_ops;
pub mod waterbody_ops;
pub mod width_ops;

use erosion_ops::*;
use mapping_ops::*;
//use mask_ops::*;
use path_ops::*;
use probability_ops::*;
use vector_ops::*;
use waterbody_ops::*;
use width_ops::*;

use crate::array_ops::noise_maps;
use crate::array_ops::translate;
use crate::worldgen;

//use constants::general::*;
use constants::world_constants::*;
use coords::Index;
use io::progress;
use io::toml::strings;

use line_drawing::BresenhamCircle;
//use std::convert::TryInto;

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
	//coordinate system function
	xy: Index,

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
	lp: &mut worldgen::LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
) {
	//initiate the parameter structure
	let mut rg = RgParams {
		//coordinate system function
		xy: Index {
			map_size: lp.wi.map_size,
		},

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
	estimate_sources_number(&mut rg, lp, wg_str);
	set_paths(&mut rg, lp, wg_str);
	map_paths(&mut rg, lp);
	map_width(&mut rg, lp);
	map_erosion(&mut rg, lp);

}

//▒▒▒▒▒▒▒▒ RIVER QUANTITY ESTIMATION ▒▒▒▒▒▒▒▒
fn estimate_sources_number(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
) {
	//Aliases
	let m_watermask = lp.topography.masks.watermask;

	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			let index = rg.xy.ind(i, j);

			let random = prng::get(0.0, 1.0, lp.wi.seed, index);

			let total_prob = prob(i, j, rg, lp);

			if (random <= total_prob)
				&& (lp.topography.read(m_watermask, index)
					== NO_WATER)
			{
				rg.river_est_number += 1;
			}
		}
	}

	println!("{}{}", wg_str.rg1, rg.river_est_number);
}
