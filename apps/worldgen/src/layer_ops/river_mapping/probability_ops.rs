use crate::layer_ops::river_mapping::RgParams;
use constants::generic as cg;
use constants::world as cw;
use game_data_codec::LayerPack;
use unit_systems::translate;

impl RgParams {
	pub fn estimate_sources_number(
		&mut self,
		lp: &mut LayerPack,
	) {
		for j in 0..lp.wi.map_size {
			for i in 0..lp.wi.map_size {
				let index = lp.index.get(i, j);
				let random = pseudo_rng::get(0.0, 1.0, lp.wi.seed, index);
				let total_prob = self.prob(i, j, lp);
				if (random <= total_prob)
					&& (lp.topography.read(lp.topography.WATERMASK, index)
						== cw::NO_WATER)
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
		let index = lp.index.get(i, j);
		let terrain = lp.topography.read(lp.topography.TERRAIN, index);
		let rainfall = lp.climate.read(lp.climate.RAINFALL, index);
		let temperature = lp.climate.read(lp.climate.TEMPERATURE, index);
		let rain_prob = f32::from(rainfall) / cg::VAL_255_F32;
		let temp_prob = f32::from(temperature) / cg::VAL_255_F32;
		let terrain_prob = f32::from(terrain) / cg::VAL_255_F32;
		let temp_abs = translate::get_abs(
			temperature as f32,
			cg::VAL_255_F32,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;
		if temp_abs <= cw::TEMP_PERM_ICE {
			cg::ZERO_F32
		} else {
			lp.wi.river_source_density
				* rain_prob
				* ((1.0 - cw::RIVER_SPAWN_TEMPERATURE_INFLUENCE)
					+ temp_prob * cw::RIVER_SPAWN_TEMPERATURE_INFLUENCE)
				* terrain_prob
		}
	}
} //impl
