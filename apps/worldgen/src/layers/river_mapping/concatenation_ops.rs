use crate::layers::river_mapping::*;
use crate::worldgen;
use constants::world_constants::*;

pub fn map_concatenation(rg: &mut RgParams, lp: &mut worldgen::LayerPack) {
	//let mut exclusion_segment_queue = Vec::new();
	let mut to_process_segment_queue = Vec::new();
	let cr = rg.rivers.concatenation_queue.clone();

	//Sort out the queue, see if rivers were concatenated before
	for entry in cr.iter().rev() {
		let found_entry = to_process_segment_queue.iter().find(
			|ConcatEntry {
			     segment_id_downstr: x,
			     ..
			 }| *x == entry.segment_id_downstr,
		);

		let found_entry_pos =
			to_process_segment_queue.iter().position(
				|ConcatEntry {
				     segment_id_downstr: x,
				     ..
				 }| *x == entry.segment_id_downstr,
			);

		match found_entry {
			Some(another_entry) => {
				//if same id recorded - take 
				if entry.topog_diff > another_entry.topog_diff {
					to_process_segment_queue
						[found_entry_pos.unwrap()] = *entry;
				}
			}
			None => {
				//exclusion_segment_queue.push(*entry);
				to_process_segment_queue.push(*entry);
			}
		};
	}

	println!("{:?}", to_process_segment_queue);

	//Enact mapping
	for entry in to_process_segment_queue.iter().rev() {
		//assign widths
		let segment_id = entry.segment_id_downstr;
		let topog_diff = entry.topog_diff;
		//println!("segment id {} with width {}", segment_id, width_new);

		println!("{:?}", segment_id);

		let river_entry = rg
			.rivers
			.list
			.iter()
			.by_ref()
			.find(|RiverEntry { segment_id: x, .. }| *x == segment_id)
			.expect("river entry not found 1 ")
			.clone();

		let mut path_array_next = river_entry.path_array;

		loop {
			if !path_array_next.is_empty() {
				path_array_next = fix_erosion(rg, path_array_next);
			} else {
				break;
			}
		}
	}
}

fn fix_erosion(
	rg: &mut RgParams,
	path_array: Vec<path::Pos>,
) -> Vec<path::Pos> {
	let mut path_array_downstr = Vec::new();

	let i_source = path_array[0].0;
	let j_source = path_array[0].1;
	let index_source = rg.xy.ind(i_source, j_source);
	
	//lower down the river if needed
	rg.topog_map[index_source] =
		rg.topog_map[index_source].saturating_sub(topog_diff);

	for n in path_array.windows(2) {
		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_downstr = rg.xy.ind(i1, j1);
		let index_current = rg.xy.ind(i0, j0);

		let cell_segment_id_current = rg.river_id_map[index_current];
		let cell_segment_id_downstr = rg.river_id_map[index_downstr];
		let cell_mask_current = rg.river_mask_map[index_downstr];
		
		//skip blank ID
		//if cell_segment_id_downstr == 0 {
		//continue;
		//}

		rg.ep.i0 = i0;
		rg.ep.j0 = j0;

		rg.ep.source_topog = rg.topog_map[index_current];

		//make sure we are below the source but still on land
		if rg.ep.source_topog > rg.topog_map[index_source] {
			rg.ep.source_topog = rg.topog_map[index_source];
		}
		if rg.ep.source_topog < 1 {
			rg.ep.source_topog = 1;
		}

		if (rg.topog_map[index_downstr] > rg.ep.source_topog)
			&& (rg.river_mask_map[index_current] != NO_RIVER)
		{
			rg.topog_map[index_downstr] = rg.ep.source_topog;

			for rad in 1..lp.wi.river_erosion_width {
				let dia = rad * 2;
				for ii in 0..dia {
					let shif_i: isize = ii as isize - rad as isize;

					rg.ep.i0 = (i0 as isize + shif_i) as usize;
					//rg.ep.i = ii;

					for jj in 0..dia {
						let shif_j: isize =
							jj as isize - rad as isize;

						rg.ep.j0 = (j0 as isize + shif_j) as usize;
						//rg.ep.j = jj;
						erosion(rg, lp, rad);
					}
				}
			}
		}
		
		//move to next segment
		if (cell_segment_id_current != cell_segment_id_downstr)
			&& (cell_mask_current != NO_RIVER)
		{
			let river_entry_downstr = rg
				.rivers
				.list
				.iter()
				.by_ref()
				.find(|RiverEntry { segment_id: x, .. }| {
					*x == cell_segment_id_downstr
				})
				.expect("river entry not found 3 ")
				.clone();

			path_array_downstr = river_entry_downstr.path_array;

			for pos in path_array_downstr.iter() {
				let i = pos.0;
				let j = pos.1;
				let index = rg.xy.ind(i, j);
				let cell_mask_next = rg.river_mask_map[index];
				let cell_id_next = rg.river_id_map[index];

				//map according to restrictions bc path is raw
				if (cell_mask_next != NO_RIVER)
					&& (cell_id_next == cell_segment_id_downstr)
				{
					rg.river_width_map[index] = cell_width_downstr;
				}
			}
		}
	}
}
