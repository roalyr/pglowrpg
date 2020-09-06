use crate::layers::river_mapping::*;
//use crate::worldgen;
use constants::world_constants::*;

pub fn prob(
	i: usize,
	j: usize,
	rg: &mut RgParams,
) -> f32 {
	let index = rg.xy.ind(i, j);
	let rain_prob = f32::from(rg.rain_map[index]) / 255.0;
	let temp_prob = f32::from(rg.temp_map[index]) / 255.0;
	let topog_prob = f32::from(rg.topog_map[index]) / 255.0;
	let temp = (rg.temp_map[index] as f32 / 255.0
		* (rg.lp.wi.abs_temp_max as f32
			- rg.lp.wi.abs_temp_min as f32)
		+ rg.lp.wi.abs_temp_min as f32) as isize;

	if temp <= TEMP_PERM_ICE {
		0.0
	} else {
		rg.lp.wi.river_source_density
		//into constants
			* rain_prob * (0.75 + temp_prob * 0.25)
			* topog_prob
	}
}
