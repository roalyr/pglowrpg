use crate::layers::river_mapping::*;
//use crate::worldgen;
use constants::world_constants::*;

pub fn prob(
	i: usize,
	j: usize,
	rg: &mut RgParams,
	lp: &mut worldgen::LayerPack,
) -> f32 {
	//Aliases
	let m_terrain = lp.topography.masks.terrain;
	let m_temp = lp.climate.masks.temperature;
	let m_rain = lp.climate.masks.rainfall;

	let index = rg.xy.ind(i, j);

	let terrain = lp.topography.read(m_terrain, index);
	let rainfall = lp.climate.read(m_rain, index);
	let temperature = lp.climate.read(m_temp, index);

	let rain_prob = f32::from(rainfall) / 255.0;
	let temp_prob = f32::from(temperature) / 255.0;
	let terrain_prob = f32::from(terrain) / 255.0;
	let temp_abs = translate::get_abs(
		temperature as f32,
		255.0,
		lp.wi.abs_temp_min as f32,
		lp.wi.abs_temp_max as f32,
	) as isize;

	if temp_abs <= TEMP_PERM_ICE {
		0.0
	} else {
		lp.wi.river_source_density
			* rain_prob * (0.75 + temp_prob * 0.25)
			* terrain_prob
	}
}
