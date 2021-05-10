use crate::layer_ops::river_mapping::RgParams;
use constants::world::*;
use game_data_codec::LayerPack;
use unit_systems::translate;

impl RgParams {
	pub fn estimate_sources_number(
		&mut self,
		lp: &mut LayerPack,
	) {
		for i in 0..lp.wi.map_size {
			for j in 0..lp.wi.map_size {
				let index = lp.xy.ind(i, j);
				let random = pseudo_rng::get(0.0, 1.0, lp.wi.seed, index);
				let total_prob = self.prob(i, j, lp);
				if (random <= total_prob)
					&& (lp.topography.read(lp.topography.WATERMASK, index) == NO_WATER)
				{
					self.river_est_number += 1;
				}
			}
		}
		//println!("{}{}", wg_str.rg1, rg.river_est_number);
	}

	pub fn prob(
		&self,
		i: usize,
		j: usize,
		lp: &mut LayerPack,
	) -> f32 {
		//Aliases
		let index = lp.xy.ind(i, j);
		let terrain = lp.topography.read(lp.topography.TERRAIN, index);
		let rainfall = lp.climate.read(lp.climate.RAINFALL, index);
		let temperature = lp.climate.read(lp.climate.TEMPERATURE, index);
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
				* rain_prob
				* (0.75 + temp_prob * 0.25)
				* terrain_prob
		}
	}
} //impl
