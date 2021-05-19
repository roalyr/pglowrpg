use crate::array_ops::noise_maps::NoiseMode;
use crate::layer_ops::river_mapping::{RgParams, RiverEntry};
use constants::world as cw;
use game_data_codec::LayerPack;
use text_ops::WS;

#[rustfmt::skip]
impl RgParams {
	pub fn set_paths(&mut self, lp: &mut LayerPack,) {
		//Maps for pathfinding must be copied into clean arrays from
		//existing composite data structures
		let terrain_map = get_terrain_map(lp);
		let random_map = get_random_map(lp);
		for i in 0..lp.wi.map_size {
			for j in 0..lp.wi.map_size {
				self.make_paths(i, j, lp, &terrain_map, &random_map);
			}
		}
	}

	fn make_paths(
		&mut self,
		i: usize,
		j: usize,
		lp: &mut LayerPack,
		terrain_map: &Vec<u8>,
		random_map: &Vec<u8>,
	) {
		//Aliases
		let index = lp.xy.ind(i, j);
		let wmask = lp.topography.read(lp.topography.WATERMASK, index);
		//To spawn or not to spawn?
		let random = pseudo_rng::get(0.0, 1.0, lp.wi.seed, index);
		let total_prob = self.prob(i, j, lp);
		if (random <= total_prob) && (wmask == cw::NO_WATER) {
			// Print the progress
			self.river_count_number += 1;
			WS.print_progress_rivers(
				self.river_count_number,
				self.river_est_number,
				20
			);
			//Set vector according to waterbodies presence
			//and randomization.
			self.vector_start(i, j);
			self.vector_end(lp);
			//Store initial vector data
			self.river_source = (self.dv.x0, self.dv.y0);
			self.river_end = (self.dv.x1, self.dv.y1);
			//Return if river is too short
			if self.vector_within_len(lp.wi.river_min_length) {return;}
			//Make pathfinding for nodes, get a queue,
			//do "windows"  between nodes, iterate and fill them
			let seg_len = lp.wi.river_segment_length;
			let mut joined_path = Vec::new();
			let mut segment_queue = Vec::new();
			let mut diag_flag = true;
			//Initial set of nodes
			let nodes = self.pathfinding_nodes(lp, seg_len, &terrain_map, diag_flag);
			//Rivers should go ortho, so that there are no gaps
			diag_flag = false;
			for node_pair in nodes.windows(2) {
				self.dv.x0 = node_pair[0].0;
				self.dv.y0 = node_pair[0].1;
				self.dv.x1 = node_pair[1].0;
				self.dv.y1 = node_pair[1].1;
				//Fill paths between nodes
				let path_array_seg = self.pathfinding_nodes(lp, 1, &random_map, diag_flag);
				segment_queue.push(path_array_seg);
			}
			//Take segment queue and map the content into a single path
			for entry in segment_queue.iter_mut() {
				for pos in entry.iter() {joined_path.push(*pos);}
			}
			//Remove duplicated cells
			joined_path.dedup();
			//Push river data to list
			self.rivers_paths.list.push(RiverEntry {
				path_array: joined_path,
				river_id: self.river_id,
				width: self.river_width,
				source: self.river_source,
				end: self.river_end,
			});
			//River id increment
			self.river_id = self
				.river_id
				.checked_add(1)
				.expect("ERROR: Overflow at river id += 1");
		}
	}

	fn pathfinding_nodes(
		&mut self,
		lp: &mut LayerPack,
		seg_len: usize,
		terrain_map: &Vec<u8>,
		diag_flag: bool,
	) -> Vec<pathfinding::Pos> {
		self.dv.path_heuristic = cw::RIVER_HEUR_INIT;
		//iter 1
		let result_init = pathfinding::make(&self.dv, &terrain_map, lp.wi.map_size, diag_flag, seg_len);
		let path_distance = self.distance();
		let estimated_heuristic = ((result_init.1 / (path_distance + 1)) as f32 * lp.wi.river_heuristic_factor) as usize;
		self.dv.path_heuristic = estimated_heuristic;
		//iter 2
		let result = pathfinding::make(&self.dv, &terrain_map, lp.wi.map_size, diag_flag, seg_len);
		result.0
	}
} //impl

fn get_random_map(lp: &mut LayerPack) -> Vec<u8> {
	//Random noise map for river path meandering
	//river segments would be using this
	let mut random_map = vec![cw::ZERO_U8; lp.layer_vec_len];
	let array_noise1 = crate::array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size1,
		lp.wi.seed,
		NoiseMode::Multi,
	);
	let array_noise2 = crate::array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size2 * 2.0,
		lp.wi.seed + 1000,
		NoiseMode::Perlin,
	);
	for (index, cell_v) in
		random_map.iter_mut().enumerate().take(lp.layer_vec_len)
	{
		*cell_v =
			(array_noise1[index] * cw::VAL_255_F32 * (1.0 - lp.wi.river_noise_blend)
				+ array_noise2[index] * cw::VAL_255_F32 * lp.wi.river_noise_blend) as u8;
	}
	random_map
}

fn get_terrain_map(lp: &mut LayerPack) -> Vec<u8> {
	//Write terrain map into a temporary array for future pathfinding
	//river nodes would be done on this
	let map_size = lp.wi.map_size;
	let mut terrain_map = vec![cw::ZERO_U8; lp.layer_vec_len];
	for i in 0..map_size {
		for j in 0..map_size {
			let index = lp.xy.ind(i, j);
			terrain_map[index] =
				lp.topography.read(lp.topography.TERRAIN, index) as u8;
		}
	}
	terrain_map
}
