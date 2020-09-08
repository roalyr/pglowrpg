use crate::layers::river_mapping::*;
//use crate::worldgen;
use crate::array_ops::noise_maps::NoiseMode::*;
use constants::world_constants::*;
use coords::Index;
//use std::f32::consts::PI;

//▒▒▒▒▒▒▒▒▒▒▒▒ INIT PATHS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn set_paths(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	_wg_str: &strings::worldgen_strings::Stuff,
) {
	//Maps for pathfinding
	let terrain_map = get_terrain_map(rg, lp);
	let random_map = get_random_map(rg, lp);
	println!("Accessory maps written",);

	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			make_paths(i, j, rg, lp, &terrain_map, &random_map);
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒ MAKE PATHS ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn make_paths(
	i: usize,
	j: usize,
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	terrain_map: &Vec<u8>,
	random_map: &Vec<u8>,
) {
	//Aliases
	let m_watermask = lp.topography.masks.watermask;

	let index = rg.xy.ind(i, j);
	let wmask = lp.topography.read(m_watermask, index);
	let map_size = lp.wi.map_size;

	//To spawn or not to spawn?
	let random = prng::get(0.0, 1.0, lp.wi.seed, index);
	let total_prob = prob(i, j, rg, lp);

	if (random <= total_prob) && (wmask == NO_WATER) {
		//UI
		rg.river_count_number += 1;
		progress::five_percent_step(
			rg.river_count_number,
			rg.river_est_number,
		);

		//Set vector according to waterbodies presence
		//and randomization.
		vector_start(rg, lp, i, j);
		vector_end(rg, lp);

		//Store initial vector data
		rg.river_source = (rg.dv.x0, rg.dv.y0);
		rg.river_end = (rg.dv.x1, rg.dv.y1);

		//Return if river is too short
		if vector_within_len(rg, lp.wi.river_min_length) {
			return;
		}

		//Make pathfinding for nodes, get a queue,
		//do "windows"  between nodes, iterate and fill them
		let mut seg_len = lp.wi.river_segment_length;
		let mut joined_path = Vec::new();
		let mut segment_queue = Vec::new();
		let mut diag_flag = true;

		//Initial set of nodes
		let mut nodes = pathfinding_nodes(
			rg,
			lp,
			seg_len,
			&terrain_map,
			diag_flag,
		);
		
		//Rivers should go ortho, so that there are no gaps
		diag_flag = false;

		for node_pair in nodes.windows(2) {
			rg.dv.x0 = node_pair[0].0;
			rg.dv.y0 = node_pair[0].1;
			rg.dv.x1 = node_pair[1].0;
			rg.dv.y1 = node_pair[1].1;
			
			//Fill paths between nodes
			let path_array_seg =
				pathfinding_nodes(rg, lp, 1, &random_map, diag_flag);
			segment_queue.push(path_array_seg);
		}

		//Take segment queue and map the content into a single path
		for entry in segment_queue.iter_mut() {
			for pos in entry.iter() {
				joined_path.push(*pos);
			}
		}

		//Remove duplicated cells
		joined_path.dedup();

		//Push river data to list
		rg.rivers_paths.list.push(RiverEntry {
			path_array: joined_path,
			river_id: rg.river_id,
			width: rg.river_width,
			source: rg.river_source,
			end: rg.river_end,
		});

		//River id increment
		rg.river_id = rg
			.river_id
			.checked_add(1)
			.expect("Overflow at river id += 1");
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒
//NODES
fn pathfinding_nodes(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	seg_len: usize,
	terrain_map: &Vec<u8>,
	diag_flag: bool,
) -> Vec<path::Pos> {
	rg.dv.path_heuristic = RIVER_HEUR_INIT;

	//iter 1
	let result_init = path::make(
		&rg.dv,
		&terrain_map,
		lp.wi.map_size,
		diag_flag,
		seg_len,
	);

	let path_distance = distance(rg);

	let estimated_heuristic = ((result_init.1 / (path_distance + 1))
		as f32 * lp.wi.river_heuristic_factor)
		as usize;

	rg.dv.path_heuristic = estimated_heuristic;

	//iter 2
	let result = path::make(
		&rg.dv,
		&terrain_map,
		lp.wi.map_size,
		diag_flag,
		seg_len,
	);
	result.0
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MAPS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn get_random_map(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) -> Vec<u8> {
	//Random noise map for river path meandering
	//river segments would be using this
	let mut random_map = vec![0; lp.layer_vec_len];

	let array_noise1 = crate::array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size1,
		lp.wi.seed,
		Multi,
	);

	let array_noise2 = crate::array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size2 * 2.0,
		lp.wi.seed + 1000,
		Perlin,
	);

	for (index, cell_v) in
		random_map.iter_mut().enumerate().take(lp.layer_vec_len)
	{
		*cell_v = (array_noise1[index]
			* 255.0 * (1.0 - lp.wi.river_noise_blend)
			+ array_noise2[index] * 255.0 * lp.wi.river_noise_blend)
			as u8;
	}
	random_map
}

pub fn get_terrain_map(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) -> Vec<u8> {
	//Write terrain map into a temporary array for future pathfinding
	//river nodes would be done on this
	let m_terrain = lp.topography.masks.terrain;

	let map_size = lp.wi.map_size;

	let mut terrain_map = vec![0; lp.layer_vec_len];

	let xy = Index { map_size };
	for i in 0..map_size {
		for j in 0..map_size {
			let index = xy.ind(i, j);
			terrain_map[index] =
				lp.topography.read(m_terrain, index) as u8;
		}
	}
	terrain_map
}
