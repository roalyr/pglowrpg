use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_floodfill::FloodFill;
use lib_game_data_codec::LayerPack;
use lib_unit_systems::translate;

//TODO: improve georegions based on elevation too.
//TODO: gather region data (size, location, cetroid, etc.).
//TODO: second pass floodfill on final map to sort out overlayed regions.

pub fn get(lp: &mut LayerPack) {
	//Clean proxy maps for floodfill
	let mut wmask_map = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	let mut biome_map = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	let mut elev_map_mask = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	for ind in 0..lp.layer_vec_len as usize {
		wmask_map[ind] = lp.topography.read(lp.topography.WATERMASK, ind) as u8;
		biome_map[ind] = lp.biomes.read(ind);
		// Filter out and write mountains (above highlands altitude) mask.
		let elev = translate::get_abs(
			lp.topography.read(lp.topography.TERRAIN, ind) as f32,
			cg::VAL_255_F32,
			lp.wi.abs_elev_min as f32,
			lp.wi.abs_elev_max as f32,
		) as u32;
		elev_map_mask[ind] = match elev {
			x if x >= cw::ELEV_HIGHLANDS => 1,
			_ => 0,
		};
	}
	// Init.
	let mut region_id: u32 = cg::ID_MAP_MIN_U32;
	let mut bioreg_map = vec![cg::ID_MAP_NO_U32; lp.layer_vec_len as usize];
	// Floodfill on biomes, watermask and elevation mask.
	let mut ff_bi = FloodFill::new(&biome_map, lp.wi.map_size);
	let mut ff_wm = FloodFill::new(&wmask_map, lp.wi.map_size);
	let mut ff_el = FloodFill::new(&elev_map_mask, lp.wi.map_size);
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
				region_id = region_id
					.checked_add(1)
					.expect("ERROR: Region ID overflow (biome regions)");
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
				region_id = region_id
					.checked_add(1)
					.expect("ERROR: Region ID overflow (water body regions)");
			}
		}
	}
	// Mountainous regions only.
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			if !ff_el.exclusion_map[lp.index.get(i, j)]
				&& (elev_map_mask[lp.index.get(i, j)] != 0)
			{
				ff_el.map(i, j);
				for y in ff_el.y_min..=ff_el.y_max {
					for x in ff_el.x_min..=ff_el.x_max {
						if ff_el.region_map[lp.index.get(x, y)] {
							bioreg_map[lp.index.get(x as u32, y as u32)] = region_id;
						}
					}
				}
				region_id = region_id
					.checked_add(1)
					.expect("ERROR: Region ID overflow (mountain regions)");
			}
		}
	}
	for (ind, cell_v) in bioreg_map.iter().enumerate() {
		lp.bioreg_id.write(*cell_v, ind)
	}
}
