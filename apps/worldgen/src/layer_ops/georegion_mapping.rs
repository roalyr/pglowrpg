use constants::world as cw;
use game_data_codec::LayerPack;
use unit_systems::coords::Index;

//This code is unfinished

pub fn get(lp: &mut LayerPack) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};
	//Clean proxy maps for floodfill
	let mut wmask_map = vec![cw::ZERO_U8; lp.layer_vec_len];
	let mut biome_map = vec![cw::ZERO_U8; lp.layer_vec_len];
	for index in 0..lp.layer_vec_len {
		wmask_map[index] = lp.topography.read(lp.topography.WATERMASK, index) as u8;
		biome_map[index] = lp.biomes.read(index);
	}
	// Init.
	let mut region_id: u32 = cw::ID_MAP_MIN_U32;
	let mut greg_map = vec![cw::ID_MAP_NO_U32; lp.layer_vec_len];
	// Floodfill on biomes and watermask.
	let mut ff_bi = floodfill::FloodFill::new(&biome_map, lp.wi.map_size);
	let mut ff_wm = floodfill::FloodFill::new(&wmask_map, lp.wi.map_size);
	// Land regions only.
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			if !ff_bi.exclusion_map[lp.xy.ind(i, j)] && (wmask_map[xy.ind(i, j)] == 0)
			{
				ff_bi.map(i, j);
				for x in ff_bi.x_min..=ff_bi.x_max {
					for y in ff_bi.y_min..=ff_bi.y_max {
						if ff_bi.region_map[xy.ind(x, y)] {
							greg_map[xy.ind(x as usize, y as usize)] = region_id;
						}
					}
				}
				region_id =
					region_id.checked_add(1).expect("ERROR: Region ID overflow");
			}
		}
	}
	// Waterbody regions only.
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			if !ff_wm.exclusion_map[xy.ind(i, j)] && (wmask_map[xy.ind(i, j)] != 0) {
				ff_wm.map(i, j);
				for x in ff_wm.x_min..=ff_wm.x_max {
					for y in ff_wm.y_min..=ff_wm.y_max {
						if ff_wm.region_map[xy.ind(x, y)] {
							greg_map[xy.ind(x as usize, y as usize)] = region_id;
						}
					}
				}
				region_id =
					region_id.checked_add(1).expect("ERROR: Region ID overflow");
			}
		}
	}
	for index in 0..lp.layer_vec_len {
		lp.georeg_id.write(greg_map[index], index)
	}
}
