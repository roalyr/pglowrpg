use crate::layer_ops::river_mapping::{
	ErosionEntry, RgParams, RiverEntry, WidthEntry,
};
use constants::generic as cg;
use constants::world as cw;
use game_data_codec::LayerPack;
use std::cmp::Ordering;
use unit_systems::translate;

#[rustfmt::skip]
impl RgParams {
	pub fn map_paths(&mut self, lp: &mut LayerPack,) {
		for river_entry in self.rivers_paths.list.clone().iter().rev() {
			self.map_rivers_reverse(lp, river_entry);
		}
	}

	fn map_rivers_reverse(
		&mut self,
		lp: &mut LayerPack,
		river_entry: &RiverEntry,
	) {
		let path_array = &river_entry.path_array;
		//Store temporary values, must be here
		self.river_id = river_entry.river_id;
		self.river_width = river_entry.width;
		self.river_source = river_entry.source;
		self.river_end = river_entry.end;
		let index_river_source = lp.index.get(self.river_source.0, self.river_source.1);
		let index_end = lp.index.get(self.river_end.0, self.river_end.1);
		let mut river_length = cg::ZERO_USIZE;
		//Cold run to figure out river lengths and truncate short ones
		for n in path_array.windows(2) {
			//Must be here
			river_length += 1;
			let i0 = n[0].0; let j0 = n[0].1; let i1 = n[1].0; let j1 = n[1].1;
			let index_current = lp.index.get(i0, j0);
			let index_downstr = lp.index.get(i1, j1);
			let temp_current = lp.climate.read(lp.climate.TEMPERATURE, index_current);
			let river_elem_current = lp.rivers.read(lp.rivers.ELEMENT, index_current);
			let river_elem_downstr = lp.rivers.read(lp.rivers.ELEMENT, index_downstr);
			let river_id_downstr = lp.rivers_id.read(index_downstr);
			let wmask_current = lp.topography.read(lp.topography.WATERMASK, index_current);
			//Stop if the temperature is too low
			let temp = translate::get_abs(
				temp_current as f32,
				cg::VAL_255_F32,
				lp.wi.abs_temp_min as f32,
				lp.wi.abs_temp_max as f32,
			) as isize;
			if temp <= cw::TEMP_POLAR {break;}
			//Minimum sink size check and stop if sink is big enough
			if wmask_current > lp.wi.river_sink_min_pool_size_pow {break;}
			//Check if river spawns on an existing one
			//also terminates rivers upon crossing
			//but allows loops, which should be adressed below
			//enabling makes rivers disappear sometimes
			if (river_elem_current != cw::NO_RIVER) && (river_length == 1) {break;}
			match river_elem_downstr {
				cw::NO_RIVER => {}
				cw::RIVER_BODY => {if self.river_id != river_id_downstr {break;}}
				cw::RIVER_WATERFALL => {if self.river_id != river_id_downstr {break;}}
				cw::RIVER_WATERFALLS_MUL => {if self.river_id != river_id_downstr {break;}}
				cw::RIVER_SOURCE => {if self.river_id != river_id_downstr {break;}}
				cw::RIVER_END => {}
				_ => {
					println!("ERROR: Elem downstream: {:?}", river_elem_downstr);
					println!("{:?}", lp.rivers.expose(index_downstr));
					panic!("Unexpected river element type value");
				}
			}
		}
		//Main run
		if river_length >= lp.wi.river_min_length {
			//Reset
			let mut river_length = cg::ZERO_USIZE;
			//Push to the queue for future
			self.erosion_initiate(self.river_id);
			for n in path_array.windows(2) {
				//Must be here
				river_length += 1;
				let i0 = n[0].0; let j0 = n[0].1; let i1 = n[1].0; let j1 = n[1].1;
				let index_current = lp.index.get(i0, j0);
				let index_downstr = lp.index.get(i1, j1);
				let temp_current = lp.climate.read(lp.climate.TEMPERATURE, index_current);
				let river_elem_current = lp.rivers.read(lp.rivers.ELEMENT, index_current);
				let river_elem_downstr = lp.rivers.read(lp.rivers.ELEMENT, index_downstr);
				let river_id_downstr = lp.rivers_id.read(index_downstr);
				let wmask_current = lp.topography.read(lp.topography.WATERMASK, index_current);
				let wmask_downstr = lp.topography.read(lp.topography.WATERMASK, index_downstr);
				let terrain_current = lp.topography.read(lp.topography.TERRAIN, index_current);
				let terrain_downstr = lp.topography.read(lp.topography.TERRAIN, index_downstr);
				//Stop if the temperature is too low
				let temp = translate::get_abs(
					temp_current as f32,
					cg::VAL_255_F32,
					lp.wi.abs_temp_min as f32,
					lp.wi.abs_temp_max as f32,
				) as isize;
				if temp <= cw::TEMP_POLAR {break;}
				//Minimum sink size check and stop if sink is big enough
				if wmask_current > lp.wi.river_sink_min_pool_size_pow {break;}
				//Check if river spawns on an existing one
				//also terminates rivers upon crossing
				//but allows loops, which should be adressed below
				//enabling makes rivers disappear sometimes
				if (river_elem_current != cw::NO_RIVER) && (river_length == 1) {break;}
				//Mark upstream neighbor
				//skip first cell, which is source
				if river_length > 1 {
					let i0_prev = self.upstream_neighbor.0;
					let j0_prev = self.upstream_neighbor.1;
					let upstream_neighbor = neighbor_flag(i0, j0, i0_prev, j0_prev);
					lp.rivers.write(upstream_neighbor, lp.rivers.UPSTREAM, index_current);
				}
				self.upstream_neighbor = (i0, j0); //Remember for next step
				match river_elem_downstr {
					cw::NO_RIVER => {
						self.sort_uninterrupted(lp, index_current, index_river_source);
						let downstream_neighbor = neighbor_flag(i0, j0, i1, j1);
						lp.rivers.write(downstream_neighbor, lp.rivers.DOWNSTREAM, index_current,);
						//If the end of the river is on land (map border) then make sure the last cell is marked
						if (wmask_downstr == cw::NO_WATER) && (index_downstr == index_end) {
							lp.rivers.write(cw::RIVER_BODY, lp.rivers.ELEMENT, index_downstr);
						}
					}
					cw::RIVER_BODY => {
						self.sort_crossing(lp, index_current, index_river_source, index_downstr,);
						//Make a waterfall if elevation differs
						if terrain_downstr < terrain_current {
							lp.rivers.write(cw::RIVER_WATERFALL, lp.rivers.ELEMENT, index_downstr,);
						}
						if self.river_id != river_id_downstr {break;} //Break river if it is not a loop, let loops go on
					}
					cw::RIVER_WATERFALL => {
						self.sort_crossing(lp, index_current, index_river_source, index_downstr,);
						//Make a waterfalls if elevation diff & waterfall exist
						if terrain_downstr < terrain_current {
							lp.rivers.write(cw::RIVER_WATERFALLS_MUL, lp.rivers.ELEMENT,index_downstr,);
						}
						if self.river_id != river_id_downstr {break;} //Break river if it is not a loop, let loops go on
					}
					cw::RIVER_WATERFALLS_MUL => {
						self.sort_crossing(lp, index_current, index_river_source, index_downstr,);
						if self.river_id != river_id_downstr {break;} //Break river if it is not a loop, let loops go on
					}
					cw::RIVER_SOURCE => {
						self.sort_crossing(lp, index_current, index_river_source, index_downstr,);
						if self.river_id != river_id_downstr {break;} //Break river if it is not a loop, let loops go on
					}
					cw::RIVER_END => {
						self.sort_uninterrupted(lp, index_current, index_river_source);
						let downstream_neighbor = neighbor_flag(i0, j0, i1, j1);
						lp.rivers.write(downstream_neighbor, lp.rivers.DOWNSTREAM, index_current,);
					}
					_ => {
						println!("ERROR: Elem downstream: {:?}", river_elem_downstr);
						println!("{:?}", lp.rivers.expose(index_downstr));
						panic!("Unexpected river mask value");
					}
				} //match
			} //for
		} //if
	}

	fn sort_uninterrupted(
		&mut self,
		lp: &mut LayerPack,
		index_current: usize,
		index_river_source: usize,
	) {
		lp.rivers.write(cw::RIVER_BODY, lp.rivers.ELEMENT, index_current);
		lp.rivers.write(cw::RIVER_SOURCE, lp.rivers.ELEMENT, index_river_source);
		lp.rivers.write(self.river_width, lp.rivers.WIDTH, index_current);
		lp.rivers_id.write(self.river_id, index_current);
	}

	fn sort_crossing(
		&mut self,
		lp: &mut LayerPack,
		index_current: usize,
		index_river_source: usize,
		index_downstr: usize,
	) {
		lp.rivers.write(cw::RIVER_BODY, lp.rivers.ELEMENT, index_current);
		lp.rivers.write(cw::RIVER_SOURCE, lp.rivers.ELEMENT, index_river_source);
		lp.rivers.write(self.river_width, lp.rivers.WIDTH, index_current);
		lp.rivers_id.write(self.river_id, index_current);
		//Modify river downstream
		self.width_routine(lp, index_downstr);
		self.erosion_adjust(lp, index_current, index_downstr);
	}

	fn width_routine(
		&mut self,
		lp: &mut LayerPack,
		index_downstr: usize,
	) {
		//Find the downstream river in queue and its width
		let result = self.rivers_paths.width_queue.iter().rev().by_ref().find(
			|WidthEntry {
			   river_id_downstr, ..
			 }| *river_id_downstr == lp.rivers_id.read(index_downstr),
		);
		//Get the width value from river downstream
		let width_downstr = match result {
			Some(x) => {x.width_new} //If in queue - return its last recorded value
			None => {lp.rivers.read(lp.rivers.WIDTH, index_downstr)} //If not in queue - just take its width as is
		};
		//Increment the width downstream
		let mut width_downstr_new = width_downstr.saturating_add(1);
		//Bound upper value by 12 order
		if width_downstr_new > cw::RIVER_MAX_WIDTH {
			width_downstr_new = cw::RIVER_MAX_WIDTH;
		}
		//Store new value for future
		self.rivers_paths.width_queue.push(WidthEntry {
			river_id_downstr: lp.rivers_id.read(index_downstr),
			width_new: width_downstr_new,
		});
	}

	fn erosion_adjust(
		&mut self,
		lp: &mut LayerPack,
		index_current: usize,
		index_downstr: usize,
	) {
		let river_id_downstr = lp.rivers_id.read(index_downstr);
		let terrain_current = lp.topography.read(lp.topography.TERRAIN, index_current);
		let terrain_downstr = lp.topography.read(lp.topography.TERRAIN, index_downstr);
		//Add difference in topography to queue
		let terrain_diff = match terrain_downstr.cmp(&terrain_current) {
			Ordering::Greater => terrain_downstr - terrain_current,
			Ordering::Equal => 0,
			Ordering::Less => return,
		};
		//Store for future use
		self.rivers_paths.erosion_queue.push(ErosionEntry {
			river_id_downstr,
			terrain_diff,
		});
	}
	
	fn erosion_initiate(
		&mut self,
		river_id: u32,
	) {
		self.rivers_paths.erosion_queue.push(ErosionEntry {
			river_id_downstr: river_id,
			terrain_diff: 0,
		});
	}
} //impl

fn neighbor_flag(
	i0: usize,
	j0: usize,
	i1: usize,
	j1: usize,
) -> u16 {
	let di: isize = i1 as isize - i0 as isize;
	let dj: isize = j1 as isize - j0 as isize;
	let neighbor = match (di, dj) {
		//Zero value is for none, at source and end
		(0, 0) => {
			panic!("ERROR: river neighbor downstream matches current (loops on self)")
		}
		(0, 1) => 1,   //N
		(1, 1) => 2,   //NE
		(1, 0) => 3,   //E
		(1, -1) => 4,  //SE
		(0, -1) => 5,  //S
		(-1, -1) => 6, //SW
		(-1, 0) => 7,  //W
		(-1, 1) => 8,  //NW
		(_, _) => {
			panic!("ERROR: unexpected neighbor direction x, y: {:?}", (di, dj))
		}
	};
	neighbor
}
