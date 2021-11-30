use lib_constants::generic as cg;
use lib_floodfill::FloodFill;
use lib_game_data_codec::LayerPack;

//This code is unfinished

pub fn get(lp: &mut LayerPack) {
	//Clean proxy maps for floodfill
	let mut wmask_map = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	for (ind, cell_v) in wmask_map.iter_mut().enumerate() {
		*cell_v = lp.topography.read(lp.topography.WATERMASK, ind) as u8;
	}
	// Init.
	let mut region_id: u32 = cg::ID_MAP_MIN_U32;
	let mut greg_map = vec![cg::ID_MAP_NO_U32; lp.layer_vec_len as usize];
	// Floodfill on watermask.
	let mut ff_wm = FloodFill::new(&wmask_map, lp.wi.map_size);
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			if !ff_wm.exclusion_map[lp.index.get(i, j)]
			//&& (wmask_map[lp.index.get(i, j)] != 0)
			{
				ff_wm.map(i, j);
				for y in ff_wm.y_min..=ff_wm.y_max {
					for x in ff_wm.x_min..=ff_wm.x_max {
						if ff_wm.region_map[lp.index.get(x, y)] {
							greg_map[lp.index.get(x as u32, y as u32)] = region_id;
						}
					}
				}
				// Sort regions by wattermask and size into continents,
				// islands, seas, lakes, etc...

				region_id =
					region_id.checked_add(1).expect("ERROR: Region ID overflow");
			}
		}
	}
	for (ind, cell_v) in greg_map.iter().enumerate() {
		lp.georeg_id.write(*cell_v, ind)
	}
}
