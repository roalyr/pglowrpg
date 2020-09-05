use crate::layers::river_mapping::*;
//use crate::worldgen;
//use constants::world_constants::*;

pub fn map_width(rg: &mut RgParams) {
	//Aliases
	//Maps
	let topog_map = rg.lp.topography;
	let rivers_map = rg.lp.rivers;
	let rivers_id_map = rg.lp.rivers_id;

	//Maps masks
	let m_terrain = rg.lp.topography.masks.terrain;
	let m_river_elem = rg.lp.rivers.masks.element;
	let m_river_width = rg.lp.rivers.masks.width;

	//Must be cloned
	let width_queue = rg.rivers_paths.width_queue.clone();
	let mut exclusion_queue = Vec::new();
	let mut to_do_queue = Vec::new();

	//Sort out the queue, take the latest values
	for entry in width_queue.iter().rev() {
		let found_entry = exclusion_queue
			.iter()
			.find(|x| x == &&entry.river_id_downstr);

		match found_entry {
			Some(_) => {}
			None => {
				exclusion_queue.push(entry.river_id_downstr);
				to_do_queue.push(entry);
			}
		};
	}

	//Enact mapping
	for entry in to_do_queue.iter().rev() {
		//Aliases
		let river_id_downstr = entry.river_id_downstr;
		let width_new = entry.width_new;

		//Skip if there is a null ID in queue somehow
		if river_id_downstr == NONE_ID_U16 {
			continue;
		}

		let river_entry = rg
			.rivers_paths
			.list
			.iter()
			.by_ref()
			.find(|RiverEntry { river_id: x, .. }| {
				*x == river_id_downstr
			})
			.expect("river entry not found in width to do queue")
			.clone();

		let path_array = river_entry.path_array;

		for pos in path_array.iter() {
			//Aliases
			let i = pos.0;
			let j = pos.1;
			let index = rg.xy.ind(i, j);

			let river_element = rivers_map.read(m_river_elem, index);
			let river_id = rivers_id_map.read(index);

			//Map width
			if (river_element != NO_RIVER)
				&& (river_id == river_id_downstr)
			{
				rg.lp.rivers.write(width_new, m_river_width, index);
			}
		}

		//Run a few more iterations
		let mut path_array_next = path_array;

		//Loop guard
		let mut iter_guard = 10;

		loop {
			if !path_array_next.is_empty() {
				path_array_next = fix_width(rg, path_array_next);
			} else {
				break;
			}

			if iter_guard <= 0 {
				break;
			}
			iter_guard -= 1;
		}
	}
}

fn fix_width(
	rg: &mut RgParams,
	path_array: Vec<path::Pos>,
) -> Vec<path::Pos> {
	//Aliases
	//Maps
	let topog_map = rg.lp.topography;
	let rivers_map = rg.lp.rivers;
	let rivers_id_map = rg.lp.rivers_id;

	//Maps masks
	let m_terrain = rg.lp.topography.masks.terrain;
	let m_river_elem = rg.lp.rivers.masks.element;
	let m_river_width = rg.lp.rivers.masks.width;

	let mut path_array_downstr = Vec::new();

	for n in path_array.windows(2) {
		//Aliases
		let i0 = n[0].0;
		let j0 = n[0].1;
		let i1 = n[1].0;
		let j1 = n[1].1;

		let index_downstr = rg.xy.ind(i1, j1);
		let index_current = rg.xy.ind(i0, j0);

		let cell_river_id_current = rivers_id_map.read(index_current);
		let cell_river_id_downstr = rivers_id_map.read(index_downstr);
		let cell_element_current =
			rivers_map.read(m_river_elem, index_current);
		let cell_width_downstr =
			rivers_map.read(m_river_width, index_downstr);
		let cell_width_current =
			rivers_map.read(m_river_width, index_current);

		//Skip blank ID
		if cell_river_id_downstr == NONE_ID_U16 {
			continue;
		}

		//Write data to list
		rg.rivers_paths
			.list
			.iter_mut()
			.find(|RiverEntry { river_id: x, .. }| {
				*x == cell_river_id_downstr
			})
			.expect("river entry not found in width fix to do queue")
			.width = cell_width_downstr;

		//Pick the last cell before crossing.
		if (cell_river_id_current != cell_river_id_downstr)
			&& (cell_element_current != NO_RIVER)
			&& (cell_width_downstr < cell_width_current)
		{
			let cell_width_downstr = cell_width_current;
			let river_entry_downstr = rg
				.rivers_paths
				.list
				.iter()
				.by_ref()
				.find(|RiverEntry { river_id: x, .. }| {
					*x == cell_river_id_downstr
				})
				.expect(
					"river entry downstream not found in width fix",
				)
				.clone();

			path_array_downstr = river_entry_downstr.path_array;

			for pos in path_array_downstr.iter() {
				let i = pos.0;
				let j = pos.1;
				let index = rg.xy.ind(i, j);
				let cell_element_next =
					rivers_map.read(m_river_elem, index);
				let cell_id_next = rivers_id_map.read(index);

				//Map width
				if (cell_element_next != NO_RIVER)
					&& (cell_id_next == cell_river_id_downstr)
				{
					rg.lp.rivers.write(
						cell_width_downstr,
						m_river_width,
						index,
					);
				}
			}
		}
	}
	path_array_downstr
}
