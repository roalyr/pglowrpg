use crate::layer_ops::river_mapping::*;

use std::cmp::Ordering;

//▒▒▒▒▒▒▒▒▒▒▒ MAP PATHS ▒▒▒▒▒▒▒▒▒▒▒
pub fn map_paths(rg: &mut RgParams, lp: &mut LayerPack) {
	for river_entry in rg.rivers_paths.list.clone().iter().rev() {
		map_rivers_reverse(rg, lp, river_entry);
	}
}

//▒▒▒▒▒▒▒▒▒▒ RIVER MAPPING ▒▒▒▒▒▒▒▒▒▒▒
fn map_rivers_reverse(
	rg: &mut RgParams, lp: &mut LayerPack, river_entry: &RiverEntry,
) {
	//Aliases
	let path_array = &river_entry.path_array;

	//Store temporary values, must be here
	rg.river_id = river_entry.river_id;
	rg.river_width = river_entry.width;
	rg.river_source = river_entry.source;
	rg.river_end = river_entry.end;

	let index_river_source =
		lp.xy.ind(rg.river_source.0, rg.river_source.1);
	let index_end = lp.xy.ind(rg.river_end.0, rg.river_end.1);

	let mut river_length = 0;

	//Cold run to figure out river lengths and truncate short ones
	for n in path_array.windows(2) {
		//Must be here
		river_length += 1;

		//Aliases
		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_current = lp.xy.ind(i0, j0);
		let index_downstr = lp.xy.ind(i1, j1);

		let temp_current =
			lp.climate.read(lp.climate.TEMPERATURE, index_current);
		let river_elem_current =
			lp.rivers.read(lp.rivers.ELEMENT, index_current);
		let river_elem_downstr =
			lp.rivers.read(lp.rivers.ELEMENT, index_downstr);
		let river_id_downstr = lp.rivers_id.read(index_downstr);
		let wmask_current =
			lp.topography.read(lp.topography.WATERMASK, index_current);

		//Stop if the temperature is too low
		let temp = translate::get_abs(
			temp_current as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		if temp <= TEMP_POLAR {
			break;
		}

		//Minimum sink size check and stop if sink is big enough
		if wmask_current > lp.wi.river_sink_min_pool_size_pow {
			break;
		}

		//Check if river spawns on an existing one
		//also terminates rivers upon crossing
		//but allows loops, which should be adressed below
		//enabling makes rivers disappear sometimes
		if (river_elem_current != NO_RIVER) && (river_length == 1) {
			break;
		}

		match river_elem_downstr {
			NO_RIVER => {}
			RIVER_BODY => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_WATERFALL => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_WATERFALLS_MUL => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_SOURCE => {
				if rg.river_id != river_id_downstr {
					break;
				}
			}
			RIVER_END => {}
			_ => {
				println!("Elem downstream: {:?}", river_elem_downstr);
				println!("{:?}", lp.rivers.expose(index_downstr));
				panic!("Unexpected river element type value");
			}
		}
	}

	//Main run
	if river_length >= lp.wi.river_min_length {
		//Reset
		let mut river_length = 0;
		//Push to the queue for future
		erosion_initiate(rg, lp, rg.river_id);

		for n in path_array.windows(2) {
			//Must be here
			river_length += 1;

			//Aliases
			let i0 = n[0].0;
			let j0 = n[0].1;
			let i1 = n[1].0;
			let j1 = n[1].1;

			let index_current = lp.xy.ind(i0, j0);
			let index_downstr = lp.xy.ind(i1, j1);

			let temp_current =
				lp.climate.read(lp.climate.TEMPERATURE, index_current);
			let river_elem_current =
				lp.rivers.read(lp.rivers.ELEMENT, index_current);
			let river_elem_downstr =
				lp.rivers.read(lp.rivers.ELEMENT, index_downstr);
			let river_id_downstr = lp.rivers_id.read(index_downstr);
			let wmask_current = lp
				.topography
				.read(lp.topography.WATERMASK, index_current);
			let wmask_downstr = lp
				.topography
				.read(lp.topography.WATERMASK, index_downstr);
			let terrain_current =
				lp.topography.read(lp.topography.TERRAIN, index_current);
			let terrain_downstr =
				lp.topography.read(lp.topography.TERRAIN, index_downstr);

			//Stop if the temperature is too low
			let temp = translate::get_abs(
				temp_current as f32,
				255.0,
				lp.wi.abs_temp_min as f32,
				lp.wi.abs_temp_max as f32,
			) as isize;

			if temp <= TEMP_POLAR {
				break;
			}

			//Minimum sink size check and stop if sink is big enough
			if wmask_current > lp.wi.river_sink_min_pool_size_pow {
				break;
			}

			//Check if river spawns on an existing one
			//also terminates rivers upon crossing
			//but allows loops, which should be adressed below
			//enabling makes rivers disappear sometimes
			if (river_elem_current != NO_RIVER) && (river_length == 1) {
				break;
			}

			//Mark upstream neighbor
			//skip first cell, which is source
			if river_length > 1 {
				let i0_prev = rg.upstream_neighbor.0;
				let j0_prev = rg.upstream_neighbor.1;

				let upstream_neighbor =
					neighbor_flag(i0, j0, i0_prev, j0_prev);
				lp.rivers.write(
					upstream_neighbor,
					lp.rivers.UPSTREAM,
					index_current,
				);
			}

			//Remember for next step
			rg.upstream_neighbor = (i0, j0);

			//map according to mask
			match river_elem_downstr {
				NO_RIVER => {
					sort_uninterrupted(
						rg,
						lp,
						index_current,
						index_river_source,
					);

					//Write down downstream neighbor direction
					let downstream_neighbor =
						neighbor_flag(i0, j0, i1, j1);
					lp.rivers.write(
						downstream_neighbor,
						lp.rivers.DOWNSTREAM,
						index_current,
					);

					//If the end of the river is on land (map border)
					//then make sure the last cell is marked
					if (wmask_downstr == NO_WATER)
						&& (index_downstr == index_end)
					{
						lp.rivers.write(
							RIVER_BODY,
							lp.rivers.ELEMENT,
							index_downstr,
						);
					}
				}

				RIVER_BODY => {
					sort_crossing(
						rg,
						lp,
						index_current,
						index_river_source,
						index_downstr,
					);

					//Make a waterfall if elevation differs
					if terrain_downstr < terrain_current {
						lp.rivers.write(
							RIVER_WATERFALL,
							lp.rivers.ELEMENT,
							index_downstr,
						);
					}

					//Break river if it is not a loop, let loops go on
					if rg.river_id != river_id_downstr {
						break;
					}
				}

				RIVER_WATERFALL => {
					sort_crossing(
						rg,
						lp,
						index_current,
						index_river_source,
						index_downstr,
					);

					//Make a waterfalls if elevation diff & waterfall exist
					if terrain_downstr < terrain_current {
						lp.rivers.write(
							RIVER_WATERFALLS_MUL,
							lp.rivers.ELEMENT,
							index_downstr,
						);
					}

					//Break river if it is not a loop, let loops go on
					if rg.river_id != river_id_downstr {
						break;
					}
				}

				RIVER_WATERFALLS_MUL => {
					sort_crossing(
						rg,
						lp,
						index_current,
						index_river_source,
						index_downstr,
					);

					//Break river if it is not a loop, let loops go on
					if rg.river_id != river_id_downstr {
						break;
					}
				}

				RIVER_SOURCE => {
					sort_crossing(
						rg,
						lp,
						index_current,
						index_river_source,
						index_downstr,
					);

					//Break river if it is not a loop, let loops go on
					if rg.river_id != river_id_downstr {
						break;
					}
				}

				RIVER_END => {
					sort_uninterrupted(
						rg,
						lp,
						index_current,
						index_river_source,
					);

					//Write down downstream neighbor direction
					let downstream_neighbor =
						neighbor_flag(i0, j0, i1, j1);
					lp.rivers.write(
						downstream_neighbor,
						lp.rivers.DOWNSTREAM,
						index_current,
					);
				}

				_ => {
					println!(
						"Elem downstream: {:?}",
						river_elem_downstr
					);
					println!("{:?}", lp.rivers.expose(index_downstr));
					panic!("Unexpected river mask value");
				}
			} //match
		} //for
	} //if
}

//▒▒▒▒▒▒▒▒▒ SORT INTERSECTIONS ▒▒▒▒▒▒▒▒▒
//UNINTERRUPTED
fn sort_uninterrupted(
	rg: &mut RgParams, lp: &mut LayerPack, index_current: usize,
	index_river_source: usize,
) {
	lp.rivers
		.write(RIVER_BODY, lp.rivers.ELEMENT, index_current);
	lp.rivers
		.write(RIVER_SOURCE, lp.rivers.ELEMENT, index_river_source);

	lp.rivers_id.write(rg.river_id, index_current);

	lp.rivers
		.write(rg.river_width, lp.rivers.WIDTH, index_current);
}

//CROSSING
fn sort_crossing(
	rg: &mut RgParams, lp: &mut LayerPack, index_current: usize,
	index_river_source: usize, index_downstr: usize,
) {
	lp.rivers
		.write(RIVER_BODY, lp.rivers.ELEMENT, index_current);
	lp.rivers
		.write(RIVER_SOURCE, lp.rivers.ELEMENT, index_river_source);

	lp.rivers_id.write(rg.river_id, index_current);

	lp.rivers
		.write(rg.river_width, lp.rivers.WIDTH, index_current);

	//Modify river downstream
	width_routine(rg, lp, index_current, index_downstr);
	erosion_adjust(rg, lp, index_current, index_downstr);
}

//▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒
//WIDTH ROUTINE
fn width_routine(
	rg: &mut RgParams, lp: &mut LayerPack, _index_current: usize,
	index_downstr: usize,
) {
	//Find the downstream river in queue and its width
	let result = rg.rivers_paths.width_queue.iter().rev().by_ref().find(
		|WidthEntry {
		     river_id_downstr, ..
		 }| { *river_id_downstr == lp.rivers_id.read(index_downstr) },
	);

	//Get the width value from river downstream
	let width_downstr = match result {
		Some(x) => {
			//If in queue - return its last recorded value
			x.width_new
		}
		None => {
			//If not in queue - just take its width as is
			lp.rivers.read(lp.rivers.WIDTH, index_downstr)
		}
	};

	//Increment the width downstream
	let mut width_downstr_new = width_downstr.saturating_add(1);

	//Bound upper value by 12 order
	if width_downstr_new > RIVER_WIDTH_ORDER_MAX {
		width_downstr_new = RIVER_WIDTH_ORDER_MAX;
	}

	//Store new value for future
	rg.rivers_paths.width_queue.push(WidthEntry {
		river_id_downstr: lp.rivers_id.read(index_downstr),
		width_new: width_downstr_new,
	});
}

//EROSION ROUTINES
fn erosion_adjust(
	rg: &mut RgParams, lp: &mut LayerPack, index_current: usize,
	index_downstr: usize,
) {
	//Aliasess
	let river_id_downstr = lp.rivers_id.read(index_downstr);
	let terrain_current =
		lp.topography.read(lp.topography.TERRAIN, index_current);
	let terrain_downstr =
		lp.topography.read(lp.topography.TERRAIN, index_downstr);

	//Add difference in topography to queue
	let terrain_diff = match terrain_downstr.cmp(&terrain_current) {
		Ordering::Greater => terrain_downstr - terrain_current,
		Ordering::Equal => 0,
		Ordering::Less => return,
	};

	//Store for future use
	rg.rivers_paths.erosion_queue.push(ErosionEntry {
		river_id_downstr,
		terrain_diff,
	});
}

fn erosion_initiate(
	rg: &mut RgParams, _lp: &mut LayerPack, river_id: u16,
) {
	rg.rivers_paths.erosion_queue.push(ErosionEntry {
		river_id_downstr: river_id,
		terrain_diff: 0,
	});
}

fn neighbor_flag(i0: usize, j0: usize, i1: usize, j1: usize) -> u16 {
	let di: isize = i1 as isize - i0 as isize;
	let dj: isize = j1 as isize - j0 as isize;

	let neighbor = match (di, dj) {
		//Zero value is for none, at source and end
		(0, 0) => panic!("river neighbor downstream matches current"),
		(1, 0) => 1,   //N
		(1, 1) => 2,   //NE
		(0, 1) => 3,   //E
		(-1, 1) => 4,  //SE
		(-1, 0) => 5,  //S
		(-1, -1) => 6, //SW
		(0, -1) => 7,  //W
		(1, -1) => 8,  //NW

		(_, _) => panic!("unexpected neighbor {:?}", (di, dj)),
	};
	neighbor
}
