//use crate::array_ops::translate;
use crate::worldgen;

use coords::Index;
use io_ops::toml::strings;

const ID_INIT: u16 = 1;

pub fn get(
	lp: &mut worldgen::LayerPack,
	_wg_str: &strings::worldgen_strings::Stuff,
) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};

	//Clean proxy maps for floodfill
	let mut wmask_map = vec![0u8; lp.layer_vec_len];
	let mut biome_map = vec![0u8; lp.layer_vec_len];

	for index in 0..lp.layer_vec_len {
		wmask_map[index] =
			lp.topography.read(lp.topography.WATERMASK, index) as u8;

		biome_map[index] = lp.biomes.read(index);
	}

	/////MUST BE U16!
	let mut greg_map = vec![ID_INIT; lp.layer_vec_len];

	//biome regions
	let mut ff_bi =
		floodfill::FloodFill::new(&biome_map, lp.wi.map_size);

	//water regions
	let mut ff_wm =
		floodfill::FloodFill::new(&wmask_map, lp.wi.map_size);

	let mut region_id: u16 = 0;

	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			//excluding water
			if !ff_bi.exclusion_map[lp.xy.ind(i, j)]
				&& (wmask_map[xy.ind(i, j)] == 0)
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
							greg_map
								[xy.ind(x as usize, y as usize)] = region_id as u16;
						}
					}
				}
			}
		}
	}

	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			//only water regions
			if !ff_wm.exclusion_map[xy.ind(i, j)]
				&& (wmask_map[xy.ind(i, j)] != 0)
			{
				ff_wm.map(i, j);

				region_id = match region_id.checked_add(1) {
					Some(x) => x,
					None => panic!("Region ID overflow (u16)"),
				};

				//regions have centers
				for x in ff_wm.x_min..=ff_wm.x_max {
					for y in ff_wm.y_min..=ff_wm.y_max {
						if ff_wm.region_map[xy.ind(x, y)] {
							greg_map
								[xy.ind(x as usize, y as usize)] = region_id as u16;
						}
					}
				}
			}
		}
	}

	//It makes sense to have two types of regions:
	//1. "hexagon" landmass biome regions which are too detailed
	//	(forest, desert, glacier so and so);
	//2. "as is" geographical region for exact shoreline
	//	(island, continent, lake, sea, etc);
	//Water biome and geographical regions should be the same
	//Algorithm:
	//1. Floodfill the water regions only first.
	//1.1. Write down a table for both bio and geo regions with the
	//	same information to match them for given water body.
	//2. Duplicate the map, region id number and exclusion maps.
	//3. First copy: do floodfill for the landmass for geo regions,
	//	and resume writing that table with landmass data
	//4. Second copy: do floodfill for the biomes for bio regions
	//	write data to table
	//5. Make bio region map into hexagons to smooth small details
	//6. Define narration rules for land/water bio/geo region comb.

	//make into hexagons
	//let greg_map_hex =
	//interpolate::hexagonify(greg_map.clone(), lp.wi.map_size);

	//make water region overlay
	//for i in 0..lp.wi.map_size {
	//for j in 0..lp.wi.map_size {
	//let index = xy.ind(i, j);
	//everywhere where there is land, put hexagons
	//if wmask_map[index] == 0 {
	//greg_map[index] = greg_map_hex[index];
	//}
	//}
	//}

	//println!("{}{}", wg_str.gr1, region_id);
	for index in 0..lp.layer_vec_len {
		lp.georeg_id.write(greg_map[index] as u16, index)
	}
}
