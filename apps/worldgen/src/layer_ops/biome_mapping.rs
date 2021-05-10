use constants::world::*;
use game_data_codec::LayerPack;
use unit_systems::translate;

pub fn get(lp: &mut LayerPack) {
	for index in 0..lp.layer_vec_len {
		let biome_type = match_biomes(lp, index);
		lp.biomes.write(biome_type, index);
	}
}

fn match_biomes(
	lp: &mut LayerPack,
	index: usize,
) -> u8 {
	//Get the absolute values for the climate.
	let temp = translate::get_abs(
		lp.climate.read(lp.climate.TEMPERATURE, index) as f32,
		255.0,
		lp.wi.abs_temp_min as f32,
		lp.wi.abs_temp_max as f32,
	) as isize;

	let rain = translate::get_abs(
		lp.climate.read(lp.climate.RAINFALL, index) as f32,
		255.0,
		lp.wi.abs_rain_min as f32,
		lp.wi.abs_rain_max as f32,
	) as usize;

	//Take anything on land that is above waterlevel,
	//waterlevel is "zero", and everything below it is submerged.
	let elev = (translate::get_abs(
		lp.topography.read(lp.topography.TERRAIN, index) as f32,
		255.0,
		lp.wi.abs_elev_min as f32,
		lp.wi.abs_elev_max as f32,
	) as usize)
		.saturating_sub(lp.wi.waterlevel);

	//Relative value (map value from 0 to 256) for waterlevel.
	//Used for making "solid" polar icecap surfaces by
	//modifying the yopographical map.
	let waterlevel_rel = if lp.wi.waterlevel < lp.wi.abs_elev_min {
		0
	} else {
		translate::get_rel(
			lp.wi.waterlevel as f32,
			255.0,
			lp.wi.abs_elev_min as f32,
			lp.wi.abs_elev_max as f32,
		) as u8
	};
	let watermask = lp.topography.read(lp.topography.WATERMASK, index);

	#[allow(overlapping_range_endpoints, clippy::match_overlapping_arm)]
	match temp {
		//▒▒▒▒▒▒▒▒▒▒ PERM ICE ▒▒▒▒▒▒▒▒▒▒
		TEMP_MIN..=TEMP_PERM_ICE => {
			if lp.topography.read(lp.topography.TERRAIN, index)
				< waterlevel_rel as u16
			{
				//Create topographical surface on polar icecaps.
				lp.topography
					.write(waterlevel_rel as u16, lp.topography.TERRAIN, index)
			}
			BIOME_POLAR_ICE_DESERT
		}
		//▒▒▒▒▒▒▒▒▒▒ POLAR ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_PERM_ICE..=TEMP_POLAR => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_WATER_ICECAP
				} else {
					//River basin biomes.
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_POLAR_ICE_DESERT,
						RAIN_DESERT..=RAIN_MAX => BIOME_POLAR_SNOWY_GLACIER,
						_ => unreachable!(),
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_POLAR_ICE_DESERT,
				RAIN_DESERT..=RAIN_MAX => BIOME_POLAR_SNOWY_GLACIER,
				_ => unreachable!(),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_POLAR_MOUNTAIN_TOP,
			_ => unreachable!(),
		},
		//▒▒▒▒▒▒▒▒▒ TUNDRA ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_POLAR..=TEMP_TUNDRA => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_ICY_WATERS
				} else {
					//River basin biomes.
					match rain {
						RAIN_MIN..=RAIN_MAX => BIOME_TUNDRA_GRASSLAND,
						_ => unreachable!(),
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_BARREN_TUNDRA,
				RAIN_DESERT..=RAIN_MAX => BIOME_TUNDRA_GRASSLAND,
				_ => unreachable!(),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TUNDRA_MOUNTAIN_TOP,
			_ => unreachable!(),
		},
		//▒▒▒▒▒▒▒▒▒▒ BOREAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TUNDRA..=TEMP_BOREAL => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_COLD_WATERS
				} else {
					//River basin biomes.
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_BOREAL_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_SWAMP,
						_ => unreachable!(),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_SWAMP,
				_ => unreachable!(),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_RAINFOREST,
				_ => unreachable!(),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_BOREAL_SHRUBLAND,
				_ => unreachable!(),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_WOODLAND => BIOME_COLD_DESERT,
				RAIN_WOODLAND..=RAIN_MAX => BIOME_BOREAL_ALPINE_GRASSLAND,
				_ => unreachable!(),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_BOREAL_MOUNTAIN_TOP,
			_ => unreachable!(),
		},
		//▒▒▒▒▒▒▒▒▒ TEMPERATE ▒▒▒▒▒▒▒▒▒▒
		TEMP_BOREAL..=TEMP_TEMPERATE => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_TEMPERATE_WATERS
				} else {
					//River basin biomes.
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_SWAMP,
						_ => unreachable!(),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_SWAMP,
				_ => unreachable!(),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_RAINFOREST,
				_ => unreachable!(),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TEMPERATE_SHRUBLAND,
				_ => unreachable!(),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_WOODLAND => BIOME_TEMPERATE_DESERT,
				RAIN_WOODLAND..=RAIN_MAX => BIOME_TEMPERATE_ALPINE_GRASSLAND,
				_ => unreachable!(),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TEMPERATE_MOUNTAIN_TOP,
			_ => unreachable!(),
		},
		//▒▒▒▒▒▒▒▒▒ TROPICAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TEMPERATE..=TEMP_MAX => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_TROPICAL_WATERS
				} else {
					//River basin biomes.
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_SWAMP,
						_ => unreachable!(),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_SWAMP,
				_ => unreachable!(),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_RAINFOREST,
				_ => unreachable!(),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TROPICAL_SHRUBLAND,
				_ => unreachable!(),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_GRASSLAND => BIOME_TROPICAL_DESERT,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TROPICAL_ALPINE_GRASSLAND,
				_ => unreachable!(),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TROPICAL_MOUNTAIN_TOP,
			_ => unreachable!(),
		}, //elev
		_ => unreachable!(),
	} //temp
}
