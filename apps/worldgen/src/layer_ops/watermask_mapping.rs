use constants::generic as cg;
use constants::world as cw;
use game_data_codec::LayerPack;
use unit_systems::translate;

pub fn get(lp: &mut LayerPack) {
	let blank = vec![cw::NO_WATER; lp.layer_vec_len as usize];
	let mut ff = floodfill::FloodFill::new(&blank, lp.wi.map_size);
	//Write down an exclusion map for dry and icy regions.
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			let index = lp.index.get(i, j);
			//Get elevation data.
			let elev = translate::get_abs(
				lp.topography.read(lp.topography.TERRAIN, index) as f32,
				cg::VAL_255_F32,
				lp.wi.abs_elev_min as f32,
				lp.wi.abs_elev_max as f32,
			) as u32;
			//Get temperature data.
			let temp = translate::get_abs(
				lp.climate.read(lp.climate.TEMPERATURE, index) as f32,
				cg::VAL_255_F32,
				lp.wi.abs_temp_min as f32,
				lp.wi.abs_temp_max as f32,
			) as i32;
			//Areas within permanent ice where no waterbodies can be.
			if temp <= cw::TEMP_PERM_ICE {
				ff.exclusion_map[index] = true;
			}
			//Areas above global waterlevel.
			if elev > lp.wi.waterlevel {
				ff.exclusion_map[index] = true;
			}
		}
	}
	//Map the areas which are outside of exclusion zones
	for j in 0..lp.wi.map_size {
		for i in 0..lp.wi.map_size {
			if !ff.exclusion_map[lp.index.get(i, j)] {
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
	for y in ff.y_min..=ff.y_max {
		for x in ff.x_min..=ff.x_max {
			if ff.region_map[lp.index.get(x, y)] {
				//The stored value is the x in 2^x = size
				//This way it can be stored in a 8-bit integer and
				//provide an estimated value (in ranges) to the region size.
				let val = translate::get_pow_2_size(ff.region_size);
				lp.topography.write(
					val as u16,
					lp.topography.WATERMASK,
					lp.index.get(x as u32, y as u32),
				)
			}
		}
	}
}
