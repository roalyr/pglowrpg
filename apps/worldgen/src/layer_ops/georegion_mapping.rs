use codec::LayerPack;
use coords::Index;
use io_ops::toml::strings;

const ID_INIT: u16 = 1;

pub fn get(lp: &mut LayerPack) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};
	//Clean proxy maps for floodfill
	let mut wmask_map = vec![0u8; lp.layer_vec_len];
	let mut biome_map = vec![0u8; lp.layer_vec_len];
	for index in 0..lp.layer_vec_len {
		wmask_map[index] = lp.topography.read(lp.topography.WATERMASK, index) as u8;
		biome_map[index] = lp.biomes.read(index);
	}
	/////MUST BE U16!
	let mut greg_map = vec![ID_INIT; lp.layer_vec_len];
	//biome regions
	let mut ff_bi = floodfill::FloodFill::new(&biome_map, lp.wi.map_size);
	//water regions
	let mut ff_wm = floodfill::FloodFill::new(&wmask_map, lp.wi.map_size);
	let mut region_id: u16 = 0;
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			//excluding water
			if !ff_bi.exclusion_map[lp.xy.ind(i, j)] && (wmask_map[xy.ind(i, j)] == 0)
			{
				ff_bi.map(i, j);
				region_id = match region_id.checked_add(1) {
					Some(x) => x,
					None => panic!("Region ID overflow (u16)"),
				};
				//regions have centers
				for x in ff_bi.x_min..=ff_bi.x_max {
					for y in ff_bi.y_min..=ff_bi.y_max {
						if ff_bi.region_map[xy.ind(x, y)] {
							greg_map[xy.ind(x as usize, y as usize)] = region_id as u16;
						}
					}
				}
			}
		}
	}
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			//only water regions
			if !ff_wm.exclusion_map[xy.ind(i, j)] && (wmask_map[xy.ind(i, j)] != 0) {
				ff_wm.map(i, j);
				region_id = match region_id.checked_add(1) {
					Some(x) => x,
					None => panic!("Region ID overflow (u16)"),
				};
				//regions have centers
				for x in ff_wm.x_min..=ff_wm.x_max {
					for y in ff_wm.y_min..=ff_wm.y_max {
						if ff_wm.region_map[xy.ind(x, y)] {
							greg_map[xy.ind(x as usize, y as usize)] = region_id as u16;
						}
					}
				}
			}
		}
	}
	for index in 0..lp.layer_vec_len {
		lp.georeg_id.write(greg_map[index] as u16, index)
	}
}
