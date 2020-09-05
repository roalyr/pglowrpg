//use crate::array_ops::noise_maps::point_multi;
use crate::layers::river_mapping::*;
use crate::worldgen;
use constants::world_constants::*;
//use prng::prng;

use std::cmp::Ordering;

//▒▒▒▒▒▒▒▒▒▒▒ MAP PATHS ▒▒▒▒▒▒▒▒▒▒▒
//PICK FROM LIST
pub fn map_paths(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) {
	for river_entry in rg.rivers.list.clone().iter().rev() {
		map_rivers_reverse(rg, lp, river_entry);
	}
}

//▒▒▒▒▒▒▒▒▒▒ river MAPPING ▒▒▒▒▒▒▒▒▒▒▒
//REVERSE
fn map_rivers_reverse(
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
	river_entry: &RiverEntry,
) {
	let path_array = &river_entry.path_array;
	rg.river_id = river_entry.river_id;

	rg.river_id = river_entry.river_id;
	rg.river_width = river_entry.width;
	rg.river_source = river_entry.source;

	rg.river_end = river_entry.end;

	//initiate
	let index_river_source =
		rg.xy.ind(rg.river_source.0, rg.river_source.1);
	let index_end = rg.xy.ind(rg.river_end.0, rg.river_end.1);
	let mut river_length = 0;

	//cold run to cull short rivers

	for n in path_array.windows(2) {
		river_length += 1;

		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_current = rg.xy.ind(i0, j0);
		let index_downstr = rg.xy.ind(i1, j1);

		//stop if the temperature is too low
		let temp = translate::get_abs(
			rg.temp_map[index_current] as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		if temp <= TEMP_POLAR {
			break;
		}

		//minimum sink size check and stop if sink is big enough
		if rg.wmask_map[index_current]
			> lp.wi.river_sink_min_pool_size_pow
		{
			break;
		}

		//check if river spawns on an existing one
		//also terminates rivers upon crossing
		//but allows loops, which should be adressed below
		//enabling makes rivers disappear sometimes
		if (river_mask_get(rg.river_mask_map[index_current])
			!= NO_RIVER) && (river_length == 1)
		{
			break;
		}

		//map according to mask
		match river_mask_get(rg.river_mask_map[index_downstr]) {
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			NO_RIVER => {}
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_BODY => {
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_WATERFALL => {
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_WATERFALLS_MUL => {
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_SOURCE => {
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_END => {}
			//▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒▒
			_ => {
				println!(
					"Mask downstream: {:?}",
					rg.river_mask_map[index_downstr]
				);
				panic!("Unexpected river mask value");
			}
		}
	}

	//check by river length
	if river_length <= lp.wi.river_min_length {
		//println!("{:?}", river_length);
		return;
	}

	//reset
	let mut river_length = 0;
	erosion_initiate(rg, rg.river_id);

	for n in path_array.windows(2) {
		//println!("▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒",);
		river_length += 1;

		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_current = rg.xy.ind(i0, j0);
		let index_downstr = rg.xy.ind(i1, j1);

		//stop if the temperature is too low
		let temp = translate::get_abs(
			rg.temp_map[index_current] as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		if temp <= TEMP_POLAR {
			break;
		}

		//minimum sink size check and stop if sink is big enough
		if rg.wmask_map[index_current]
			> lp.wi.river_sink_min_pool_size_pow
		{
			break;
		}

		//check if river spawns on an existing one
		//also terminates rivers upon crossing
		//but allows loops, which should be adressed below
		//enabling makes rivers disappear sometimes
		if (river_mask_get(rg.river_mask_map[index_current])
			!= NO_RIVER) && (river_length == 1)
		{
			break;
		}

		//mark upstream neighbor
		//skip first cell, which is source
		if river_length > 1 {
			let i0_prev = rg.upstream_neighbor.0;
			let j0_prev = rg.upstream_neighbor.1;

			river_upstream_set(
				&mut rg.river_mask_map[index_current],
				mark_neighbor(i0, j0, i0_prev, j0_prev),
			);
		}
		//remember for next step
		rg.upstream_neighbor = (i0, j0);

		//map according to mask
		match river_mask_get(rg.river_mask_map[index_downstr]) {
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			NO_RIVER => {
				sort_uninterrupted(
					rg,
					index_current,
					index_river_source,
				);

				//write down downstream neighbor direction
				river_downstream_set(
					&mut rg.river_mask_map[index_current],
					mark_neighbor(i0, j0, i1, j1),
				);

				//if the end of the river is on land (map border)
				//then make sure the last cell is marked
				if (rg.wmask_map[index_downstr] == NO_WATER)
					&& (index_downstr == index_end)
				{
					river_mask_set(
						&mut rg.river_mask_map[index_downstr],
						RIVER_BODY,
					);
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_BODY => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);
				if rg.topog_map[index_downstr]
					< rg.topog_map[index_current]
				{
					river_mask_set(
						&mut rg.river_mask_map[index_downstr],
						RIVER_WATERFALL,
					);
				}

				//break river if it is not a loop, let loops go on
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_WATERFALL => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);
				if rg.topog_map[index_downstr]
					< rg.topog_map[index_current]
				{
					river_mask_set(
						&mut rg.river_mask_map[index_downstr],
						RIVER_WATERFALLS_MUL,
					);
				}

				//break river if it is not a loop, let loops go on
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_WATERFALLS_MUL => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//break river if it is not a loop, let loops go on
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_SOURCE => {
				sort_crossing(
					rg,
					lp,
					index_current,
					index_river_source,
					index_downstr,
				);

				//break river if it is not a loop, let loops go on
				if rg.river_id != rg.river_id_map[index_downstr] {
					break;
				}
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			RIVER_END => {
				sort_uninterrupted(
					rg,
					index_current,
					index_river_source,
				);
				//write down downstream neighbor direction
				river_downstream_set(
					&mut rg.river_mask_map[index_current],
					mark_neighbor(i0, j0, i1, j1),
				);
			}
			//▒▒▒▒▒▒▒▒▒▒▒▒  ▒▒▒▒▒▒▒▒▒▒▒▒
			_ => {
				println!(
					"Mask downstream: {:?}",
					rg.river_mask_map[index_downstr]
				);
				panic!("Unexpected river mask value");
			}
		} //match
	}
	//println!("{:?}", river_length);
}

//▒▒▒▒▒▒▒▒▒ SORT INTERSECTIONS ▒▒▒▒▒▒▒▒▒
//UNINTERRUPTED
fn sort_uninterrupted(
	rg: &mut RgParams,
	index_current: usize,
	index_source: usize,
) {
	river_mask_set(&mut rg.river_mask_map[index_current], RIVER_BODY);
	river_mask_set(
		&mut rg.river_mask_map[index_source],
		RIVER_SOURCE,
	);

	rg.river_id_map[index_current] = rg.river_id;
	rg.river_id_map[index_current] = rg.river_id;
	rg.river_width_map[index_current] = rg.river_width;
}

//CROSSING
fn sort_crossing(
	rg: &mut RgParams,
	lp: &worldgen::LayerPack,
	index_current: usize,
	index_source: usize,
	index_downstr: usize,
) {
	//last cell in the river before termination
	river_mask_set(&mut rg.river_mask_map[index_current], RIVER_BODY);
	river_mask_set(
		&mut rg.river_mask_map[index_source],
		RIVER_SOURCE,
	);

	rg.river_id_map[index_current] = rg.river_id;
	rg.river_id_map[index_current] = rg.river_id;
	rg.river_width_map[index_current] = rg.river_width;

	//modify river downstream
	width_routine(rg, lp, index_current, index_downstr);
	erosion_adjust(rg, index_current, index_downstr);
}

//▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒
//WIDTH ROUTINE
fn width_routine(
	rg: &mut RgParams,
	_lp: &worldgen::LayerPack,
	_index_current: usize,
	index_downstr: usize,
) {
	//if rg.river_id_map[index_downstr] == 0 {
	//return;
	//}

	//find the downstream river in queue and its width
	let result = rg.rivers.width_queue.iter().rev().by_ref().find(
		|WidthEntry {
		     river_id_downstr, ..
		 }| { *river_id_downstr == rg.river_id_map[index_downstr] },
	);

	//Get the width value from river downstream
	let width_downstr = match result {
		Some(x) => {
			//if in queue - return its last recorded value
			x.width_new
		}
		None => {
			//if not in queue - just take its width as is
			rg.river_width_map[index_downstr]
		}
	};

	//increment the width downstream
	let mut width_downstr_new = width_downstr.saturating_add(1);

	//bound upper value by 12 order
	if width_downstr_new > RIVER_WIDTH_ORDER_MAX {
		width_downstr_new = RIVER_WIDTH_ORDER_MAX;
	}

	rg.rivers.width_queue.push(WidthEntry {
		river_id_downstr: rg.river_id_map[index_downstr],
		width_new: width_downstr_new,
	});
}

//EROSION ROUTINES
fn erosion_adjust(
	rg: &mut RgParams,
	index_current: usize,
	index_downstr: usize,
) {
	//if rg.river_id_map[index_downstr] == 0 {
	//return;
	//}

	//add difference in topography to queue
	let terrain_diff = match rg.topog_map[index_downstr]
		.cmp(&rg.topog_map[index_current])
	{
		Ordering::Greater => {
			rg.topog_map[index_downstr] - rg.topog_map[index_current]
		}
		Ordering::Equal => 0,
		Ordering::Less => return,
	};

	rg.rivers.erosion_queue.push(ErosionEntry {
		river_id_downstr: rg.river_id_map[index_downstr],
		terrain_diff,
	});
}

fn erosion_initiate(
	rg: &mut RgParams,
	river_id: u16,
) {
	rg.rivers.erosion_queue.push(ErosionEntry {
		river_id_downstr: river_id,
		terrain_diff: 0,
	});
}

fn mark_neighbor(
	i0: usize,
	j0: usize,
	i1: usize,
	j1: usize,
) -> u8 {
	let di: isize = i1 as isize - i0 as isize;
	let dj: isize = j1 as isize - j0 as isize;

	let neighbor = match (di, dj) {
		(0, 0) => panic!("neighbor downstream matches current"),
		(1, 0) => 0,   //N
		(1, 1) => 1,   //NE
		(0, 1) => 2,   //E
		(-1, 1) => 3,  //SE
		(-1, 0) => 4,  //S
		(-1, -1) => 5, //SW
		(0, -1) => 6,  //W
		(1, -1) => 7,  //NW

		(_, _) => panic!("unexpected neighbor {:?}", (di, dj)),
	};

	neighbor
}
