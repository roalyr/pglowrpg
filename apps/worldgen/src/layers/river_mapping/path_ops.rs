use crate::layers::river_mapping::*;
use crate::worldgen;
use constants::world_constants::*;
use std::f32::consts::PI;

//▒▒▒▒▒▒▒▒▒▒▒▒ INIT PATHS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn set_paths(
	rg: &mut RgParams,
	_wg_str: &strings::worldgen_strings::Stuff,
) {
	for i in 0..rg.lp.wi.map_size {
		for j in 0..rg.lp.wi.map_size {
			make_paths(i, j, rg);
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒ MAKE PATHS ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn make_paths(
	i: usize,
	j: usize,
	rg: &mut RgParams,
) {
	let index = rg.xy.ind(i, j);

	let random = prng::get(0.0, 1.0, rg.lp.wi.seed, index);

	let total_prob = prob(i, j, rg);

	if (random <= total_prob) && (rg.wmask_map[index] == NO_WATER) {
		//UI
		rg.river_count_number += 1;
		progress::five_percent_step(
			rg.river_count_number,
			rg.river_est_number,
		);

		//set vector according to waterbodies presence
		//and randomization.
		vector_start(rg, i, j);
		vector_end(rg);

		//store initial vector data
		rg.river_source = (rg.dv.x0, rg.dv.y0);
		rg.river_end = (rg.dv.x1, rg.dv.y1);

		//return if river is too short
		if vector_within_len(rg, rg.lp.wi.river_min_length) {
			return;
		}

		//make pathfinding for nodes, get a queue
		//do "windows"  between nodes, iterate and fill them
		let nodes = pathfinding_nodes(rg);

		let mut segment_queue = Vec::new();
		let mut joined_path = Vec::new();

		//segment goes between those nodes
		for node_pair in nodes.windows(2) {
			//set temporary vector points for each segment
			rg.dv.x0 = node_pair[0].0;
			rg.dv.y0 = node_pair[0].1;
			rg.dv.x1 = node_pair[1].0;
			rg.dv.y1 = node_pair[1].1;

			let path_array_seg = pathfinding_segments(rg);
			segment_queue.push(path_array_seg);
		}

		//take segment queue and map the content into a single path
		for entry in segment_queue.iter_mut() {
			for pos in entry.iter() {
				//println!("x {:?}, y {:?}", pos.0, pos.1);
				joined_path.push(*pos);
			}
		}

		//remove duplicates
		joined_path.dedup();

		//push river data to list
		rg.rivers.list.push(RiverEntry {
			path_array: joined_path,
			river_id: rg.river_id,
			width: rg.river_width,
			source: rg.river_source,

			end: rg.river_end,
		});

		//river id increment
		rg.river_id = rg
			.river_id
			.checked_add(1)
			.expect("Overflow at river id += 1");
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒
//LIST MAKER
fn path_segments(rg: &mut RgParams) {
	let path_array_seg = pathfinding_segments(rg);
	//keep this check? maybe add bool flag on return
	if path_array_seg.is_empty() {
		return;
	}

	//push river data to list
	rg.rivers.list.push(RiverEntry {
		path_array: path_array_seg.to_vec(),
		river_id: rg.river_id,
		width: rg.river_width,
		source: rg.river_source,

		end: rg.river_end,
	});
}

//ACTUAL PATHFINDING
fn pathfinding_segments(rg: &mut RgParams) -> Vec<path::Pos> {
	rg.dv.path_heuristic = RIVER_HEUR_INIT;

	//rivers go ortho
	let diag_flag = false;

	//iter 1
	let result_init = path::make(
		&rg.dv,
		&rg.rtopog_map,
		rg.lp.wi.map_size,
		diag_flag,
		1,
	);

	let path_distance = distance(rg);

	let estimated_heuristic =
		((result_init.1 / (path_distance + 1)) as f32
			* rg.lp.wi.river_heuristic_factor) as usize;

	rg.dv.path_heuristic = estimated_heuristic;

	//iter 2
	let result = path::make(
		&rg.dv,
		&rg.rtopog_map,
		rg.lp.wi.map_size,
		diag_flag,
		1,
	);
	result.0
}

//NODES
fn pathfinding_nodes(rg: &mut RgParams) -> Vec<path::Pos> {
	rg.dv.path_heuristic = RIVER_HEUR_INIT;

	//nodes go ortho and dia
	let diag_flag = true;

	//iter 1
	let result_init = path::make(
		&rg.dv,
		&rg.topog_map,
		rg.lp.wi.map_size,
		diag_flag,
		rg.lp.wi.river_segment_length,
	);

	let path_distance = distance(rg);

	let estimated_heuristic =
		((result_init.1 / (path_distance + 1)) as f32
			* rg.lp.wi.river_heuristic_factor) as usize;

	rg.dv.path_heuristic = estimated_heuristic;

	//iter 2
	let result = path::make(
		&rg.dv,
		&rg.topog_map,
		rg.lp.wi.map_size,
		diag_flag,
		rg.lp.wi.river_segment_length,
	);
	//println!("nodes {:?}", result.0);
	result.0
}
