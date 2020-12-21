use codec::LayerPack;
use constants_world::*;
use coords::Index;
use units::translate;

pub fn get(lp: &mut LayerPack) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};
	let blank = vec![NO_WATER; lp.layer_vec_len];
	let mut ff = floodfill::FloodFill::new(&blank, lp.wi.map_size);
	//Write down an exclusion map for dry and icy regions
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			let index = xy.ind(i, j);
			let elev = translate::get_abs(
				lp.topography.read(lp.topography.TERRAIN, index) as f32,
				255.0,
				lp.wi.abs_elev_min as f32,
				lp.wi.abs_elev_max as f32,
			) as usize;
			let temp = translate::get_abs(
				lp.climate.read(lp.climate.TEMPERATURE, index) as f32,
				255.0,
				lp.wi.abs_temp_min as f32,
				lp.wi.abs_temp_max as f32,
			) as isize;
			if temp <= TEMP_PERM_ICE {
				ff.exclusion_map[index] = true;
			}
			if elev > lp.wi.waterlevel {
				ff.exclusion_map[index] = true;
			}
		}
	}
	//Map the areas which are outside of exclusion zones
	for i in 0..lp.wi.map_size {
		for j in 0..lp.wi.map_size {
			if !ff.exclusion_map[xy.ind(i, j)] {
				ff.map(i, j);
				write_map(lp, &mut ff);
			}
		}
	}
}

fn write_map(
	lp: &mut LayerPack,
	ff: &mut floodfill::FloodFill<u16>,
) {
	let xy = Index {
		map_size: lp.wi.map_size,
	};
	for x in ff.x_min..=ff.x_max {
		for y in ff.y_min..=ff.y_max {
			if ff.region_map[xy.ind(x, y)] {
				//The stored value is the x in 2^x = size
				let val = translate::get_pow_2_size(ff.region_size);
				lp.topography.write(
					val as u16,
					lp.topography.WATERMASK,
					xy.ind(x as usize, y as usize),
				)
			}
		}
	}
}
