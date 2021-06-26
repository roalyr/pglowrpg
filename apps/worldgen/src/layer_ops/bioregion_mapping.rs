use constants::generic as cg;
use game_data_codec::LayerPack;

//This code is unfinished

pub fn get(lp: &mut LayerPack) {
	//Clean proxy maps for floodfill
	let mut wmask_map = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	let mut biome_map = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	for ind in 0..lp.layer_vec_len as usize {
		wmask_map[ind] = lp.topography.read(lp.topography.WATERMASK, ind) as u8;
		biome_map[ind] = lp.biomes.read(ind);
	}
	// Init.
	let mut region_id: u32 = cg::ID_MAP_MIN_U32;
	let mut bioreg_map = vec![cg::ID_MAP_NO_U32; lp.layer_vec_len as usize];
	// Floodfill on biomes and watermask.
	let mut ff_bi = floodfill::FloodFill::new(&biome_map, lp.wi.map_size);
	let mut ff_wm = floodfill::FloodFill::new(&wmask_map, lp.wi.map_size);
	// Land regions only.
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			if !ff_bi.exclusion_map[lp.index.get(i, j)]
				&& (wmask_map[lp.index.get(i, j)] == 0)
			{
				ff_bi.map(i, j);
				for y in ff_bi.y_min..=ff_bi.y_max {
					for x in ff_bi.x_min..=ff_bi.x_max {
						if ff_bi.region_map[lp.index.get(x, y)] {
							bioreg_map[lp.index.get(x as u32, y as u32)] = region_id;
						}
					}
				}
				region_id =
					region_id.checked_add(1).expect("ERROR: Region ID overflow");
			}
		}
	}
	// Waterbody regions only.
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			if !ff_wm.exclusion_map[lp.index.get(i, j)]
				&& (wmask_map[lp.index.get(i, j)] != 0)
			{
				ff_wm.map(i, j);
				for y in ff_wm.y_min..=ff_wm.y_max {
					for x in ff_wm.x_min..=ff_wm.x_max {
						if ff_wm.region_map[lp.index.get(x, y)] {
							bioreg_map[lp.index.get(x as u32, y as u32)] = region_id;
						}
					}
				}
				region_id =
					region_id.checked_add(1).expect("ERROR: Region ID overflow");
			}
		}
	}
	for (ind, cell_v) in bioreg_map.iter().enumerate() {
		lp.bioreg_id.write(*cell_v, ind)
	}
}
