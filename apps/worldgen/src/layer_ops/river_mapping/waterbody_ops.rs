use crate::layer_ops::river_mapping::*;

//▒▒▒▒▒▒▒▒▒▒ WITH OR WITHOUT ▒▒▒▒▒▒▒▒▒▒▒
pub fn with_water(rg: &mut RgParams, lp: &mut LayerPack) {
	let iter = rg.dv.x0 + rg.dv.y0;
	let random =
		prng::get(0.0, 1.0, lp.wi.seed, rg.dv.x0 * rg.dv.y0 * iter);

	if random < lp.wi.river_rand_vectors {
		without_water(rg, lp);
	} else {
		seek_waterbodies(rg, lp);
	}
}

pub fn without_water(rg: &mut RgParams, lp: &mut LayerPack) {
	//if there is a pole - go for biased placement
	without_water_no_pole(rg, lp);
}

//▒▒▒▒▒▒▒▒▒▒▒▒ DETAILS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn without_water_no_pole(rg: &mut RgParams, lp: &mut LayerPack) {
	project_randomly(rg, lp);
}

//▒▒▒▒▒▒▒▒▒▒▒▒ ROUTINES ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn seek_waterbodies(rg: &mut RgParams, lp: &mut LayerPack) {
	//Aliases
	let size = lp.wi.map_size;

	//take a bit more than sqrt(2)
	let diag = size * 15 / 10;

	loop {
		for (i, j) in BresenhamCircle::new(
			rg.dv.x0 as i32,
			rg.dv.y0 as i32,
			rg.dv.r as i32,
		) {
			let i = i as usize;
			let j = j as usize;

			if (i < size) && (j < size) {
				let index = lp.xy.ind(i, j);
				let wmask =
					lp.topography.read(lp.topography.WATERMASK, index);
				let temp =
					lp.climate.read(lp.climate.TEMPERATURE, index);

				let temp_abs = translate::get_abs(
					temp as f32,
					255.0,
					lp.wi.abs_temp_min as f32,
					lp.wi.abs_temp_max as f32,
				) as isize;

				if (wmask >= lp.wi.river_attr_pool_size_pow)
					&& (temp_abs > TEMP_POLAR)
				{
					rg.dv.x1 = i;
					rg.dv.y1 = j;
					rg.dv.hit = true;
					break;
				}
			}
		}
		if !rg.dv.hit {
			rg.dv.r += 1;
		} else {
			//reset stuff
			rg.dv.r = ONE_USIZE;
			rg.dv.hit = false;
			break;
		}

		//stop if no prev. check worked
		if rg.dv.r >= diag {
			break;
		}
	}
}

fn project_randomly(rg: &mut RgParams, lp: &mut LayerPack) {
	//Aliases
	let vec_angle = lp.wi.river_vect_angle;
	let vec_deviation = lp.wi.river_vect_angle_max_deviation;
	let noise_factor = lp.wi.river_vect_angle_noise;
	let size = lp.wi.map_size as f32;

	let radius = size * 1.5;
	let random = prng::get(
		0.0,
		1.0,
		lp.wi.seed,
		rg.dv.x1 * rg.dv.y0 + rg.dv.y1 * rg.dv.x0,
	);

	let mut shift = 2.0
		* std::f32::consts::PI
		* noise_maps::point_multi(
			noise_factor,
			lp.wi.seed,
			rg.dv.x0,
			rg.dv.y0,
		);

	if random > 0.5 {
		shift = -shift;
	}

	let mut vec_angle_mod: f32 = vec_angle + shift;

	//limit the deviation of the river vector to keep it aligned
	if vec_angle_mod > vec_angle + vec_deviation {
		vec_angle_mod = vec_angle + vec_deviation;
	}

	if vec_angle_mod < vec_angle - vec_deviation {
		vec_angle_mod = vec_angle - vec_deviation;
	}

	//println!("{:?}", vec_angle_mod);

	let x0 = rg.dv.x0 as f32;
	let y0 = rg.dv.y0 as f32;

	let xr = radius * vec_angle_mod.cos();
	let yr = radius * vec_angle_mod.sin();

	let mut x1 = x0 + xr;
	let mut y1 = y0 + yr;

	if x1 >= size {
		x1 = size - 1.0;
	}

	if y1 >= size {
		y1 = size - 1.0;
	}

	if x1 < 0.0 {
		x1 = 0.0;
	}

	if y1 < 0.0 {
		y1 = 0.0;
	}

	rg.dv.x1 = x1 as usize;
	rg.dv.y1 = y1 as usize;
}
