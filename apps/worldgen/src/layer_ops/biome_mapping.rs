use codec::LayerPack;
use constants_world::*;
use units::translate;

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

	//take above waterlevel, waterlevel is "zero"
	let elev = (translate::get_abs(
		lp.topography.read(lp.topography.TERRAIN, index) as f32,
		255.0,
		lp.wi.abs_elev_min as f32,
		lp.wi.abs_elev_max as f32,
	) as usize)
		.saturating_sub(lp.wi.waterlevel);

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

	#[allow(overlapping_patterns, clippy::match_overlapping_arm)]
	match temp {
		//▒▒▒▒▒▒▒▒▒▒ PERM ICE ▒▒▒▒▒▒▒▒▒▒
		TEMP_MIN..=TEMP_PERM_ICE => {
			if lp.topography.read(lp.topography.TERRAIN, index)
				< waterlevel_rel as u16
			{
				//fix elevation
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
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_POLAR_ICE_DESERT,
						RAIN_DESERT..=RAIN_MAX => BIOME_POLAR_SNOWY_GLACIER,
						_ => panic!("Unexpected rainfall biome value for biome {}", rain),
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_POLAR_ICE_DESERT,
				RAIN_DESERT..=RAIN_MAX => BIOME_POLAR_SNOWY_GLACIER,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_POLAR_MOUNTAIN_TOP,
			_ => panic!("Unexpected elevation biome value for biome {}", elev),
		},
		//▒▒▒▒▒▒▒▒▒ TUNDRA ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_POLAR..=TEMP_TUNDRA => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_ICY_WATERS
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_MAX => BIOME_TUNDRA_GRASSLAND,
						_ => panic!("Unexpected rainfall biome value for biome {}", rain),
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_BARREN_TUNDRA,
				RAIN_DESERT..=RAIN_MAX => BIOME_TUNDRA_GRASSLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TUNDRA_MOUNTAIN_TOP,
			_ => panic!("Unexpected elevation biome value for biome {}", elev),
		},
		//▒▒▒▒▒▒▒▒▒▒ BOREAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TUNDRA..=TEMP_BOREAL => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_COLD_WATERS
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_BOREAL_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_SWAMP,
						_ => panic!("Unexpected rainfall biome value for biome {}", rain),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_SWAMP,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_BOREAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_BOREAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_BOREAL_RAINFOREST,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_COLD_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_BOREAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_BOREAL_SHRUBLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_WOODLAND => BIOME_COLD_DESERT,
				RAIN_WOODLAND..=RAIN_MAX => BIOME_BOREAL_ALPINE_GRASSLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_BOREAL_MOUNTAIN_TOP,
			_ => panic!("Unexpected elevation biome value for biome {}", elev),
		},
		//▒▒▒▒▒▒▒▒▒ TEMPERATE ▒▒▒▒▒▒▒▒▒▒
		TEMP_BOREAL..=TEMP_TEMPERATE => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_TEMPERATE_WATERS
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_SWAMP,
						_ => panic!("Unexpected rainfall biome value for biome {}", rain),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_SWAMP,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TEMPERATE_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TEMPERATE_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TEMPERATE_RAINFOREST,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TEMPERATE_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TEMPERATE_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TEMPERATE_SHRUBLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_WOODLAND => BIOME_TEMPERATE_DESERT,
				RAIN_WOODLAND..=RAIN_MAX => BIOME_TEMPERATE_ALPINE_GRASSLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TEMPERATE_MOUNTAIN_TOP,
			_ => panic!("Unexpected elevation biome value for biome {}", elev),
		},
		//▒▒▒▒▒▒▒▒▒ TROPICAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TEMPERATE..=TEMP_MAX => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					BIOME_TROPICAL_WATERS
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_GRASSLAND,
						RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_WOODLAND,
						RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_FOREST,
						RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
						RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_SWAMP,
						_ => panic!("Unexpected rainfall biome value for biome {}", rain),
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_SWAMP,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_WOODLAND => BIOME_TROPICAL_WOODLAND,
				RAIN_WOODLAND..=RAIN_FOREST => BIOME_TROPICAL_FOREST,
				RAIN_FOREST..=RAIN_MAX => BIOME_TROPICAL_RAINFOREST,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => match rain {
				RAIN_MIN..=RAIN_DESERT => BIOME_TROPICAL_DESERT,
				RAIN_DESERT..=RAIN_GRASSLAND => BIOME_TROPICAL_GRASSLAND,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TROPICAL_SHRUBLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_HIGHLANDS..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_GRASSLAND => BIOME_TROPICAL_DESERT,
				RAIN_GRASSLAND..=RAIN_MAX => BIOME_TROPICAL_ALPINE_GRASSLAND,
				_ => panic!("Unexpected rainfall biome value for biome {}", rain),
			},
			ELEV_ALPINE..=ELEV_MAX => BIOME_TROPICAL_MOUNTAIN_TOP,
			_ => panic!("Unexpected elevation biome value for biome {}", elev),
		}, //elev
		_ => panic!("Unexpected temperature biome value for biome {}", temp),
	} //temp
}
