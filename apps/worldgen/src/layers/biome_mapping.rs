use crate::array_ops::translate;
use crate::worldgen;
use constants_world::*;

#[allow(clippy::cognitive_complexity)]
pub fn get(lp: &mut worldgen::LayerPack) {
	for index in 0..lp.layer_vec_len {
		let biome_type = match_biomes(lp, index);
		lp.biomes.write(biome_type, index);
	}
}

fn match_biomes(
	lp: &mut worldgen::LayerPack,
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

	let watermask =
		lp.topography.read(lp.topography.WATERMASK, index);

	#[allow(overlapping_patterns, clippy::match_overlapping_arm)]
	match temp {
		//▒▒▒▒▒▒▒▒▒▒ PERM ICE ▒▒▒▒▒▒▒▒▒▒
		TEMP_MIN..=TEMP_PERM_ICE => {
			if lp.topography.read(lp.topography.TERRAIN, index)
				< waterlevel_rel as u16
			{
				lp.topography.write(
					waterlevel_rel as u16,
					lp.topography.TERRAIN,
					index,
				) //fix elev
			}
			5 //polar ice desert
		}

		//▒▒▒▒▒▒▒▒▒▒ POLAR ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_PERM_ICE..=TEMP_POLAR => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					0 //water icecap
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => 5, //polar ice desert
						RAIN_DESERT..=RAIN_MAX => 9, //polar snowy glacier
						_ => 100,
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => 5, //polar ice desert
				RAIN_DESERT..=RAIN_MAX => 9, //polar snowy glacier
				_ => 100,
			},
			ELEV_ALPINE..=ELEV_MAX => 11, //polar mountain tops
			_ => 100,
		},

		//▒▒▒▒▒▒▒▒▒ TUNDRA ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_POLAR..=TEMP_TUNDRA => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					1 //icy waters
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_MAX => 16, //tundra grassland
						_ => 100,
					}
				}
			}
			ELEV_MIN..=ELEV_ALPINE => match rain {
				RAIN_MIN..=RAIN_DESERT => 10, //barren tundra
				RAIN_DESERT..=RAIN_MAX => 16, //tundra grassland
				_ => 100,
			},
			ELEV_ALPINE..=ELEV_MAX => 12, //tundra mountain tops
			_ => 100,
		},

		//▒▒▒▒▒▒▒▒▒▒ BOREAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TUNDRA..=TEMP_BOREAL => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					2 //cold waters
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => 17, //boreal grassland
						RAIN_DESERT..=RAIN_GRASSLAND => 20, //boreal woodland
						RAIN_GRASSLAND..=RAIN_WOODLAND => 23, //boreal forest
						RAIN_WOODLAND..=RAIN_FOREST => 23, //boreal forest
						RAIN_FOREST..=RAIN_MAX => 26, //boreal swamp
						_ => 100,
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => 6, //cold desert
				RAIN_DESERT..=RAIN_GRASSLAND => 17, //boreal grassland
				RAIN_GRASSLAND..=RAIN_WOODLAND => 20, //boreal woodland
				RAIN_WOODLAND..=RAIN_FOREST => 23, //boreal forest
				RAIN_FOREST..=RAIN_MAX => 26, //boreal swamp
				_ => 100,
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 6, //cold desert
					RAIN_DESERT..=RAIN_GRASSLAND => 17, //boreal grassland
					RAIN_GRASSLAND..=RAIN_WOODLAND => 20, //boreal woodland
					RAIN_WOODLAND..=RAIN_FOREST => 23, //boreal forest
					RAIN_FOREST..=RAIN_MAX => 29, //boreal rainforest
					_ => 100,
				}
			}
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 6, //cold desert
					RAIN_DESERT..=RAIN_GRASSLAND => 17, //boreal grassland
					RAIN_GRASSLAND..=RAIN_MAX => 32, //boreal shrubland
					_ => 100,
				}
			}
			ELEV_HIGHLANDS..=ELEV_ALPINE => {
				match rain {
					RAIN_MIN..=RAIN_WOODLAND => 6, //cold desert
					RAIN_WOODLAND..=RAIN_MAX => 35, //boreal alpine grassland
					_ => 100,
				}
			}
			ELEV_ALPINE..=ELEV_MAX => 13, //boreal mountain tops
			_ => 100,
		},

		//▒▒▒▒▒▒▒▒▒ TEMPERATE ▒▒▒▒▒▒▒▒▒▒
		TEMP_BOREAL..=TEMP_TEMPERATE => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					3 //temperate waters
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => 18, //temperate grassland
						RAIN_DESERT..=RAIN_GRASSLAND => 21, //temperate woodland
						RAIN_GRASSLAND..=RAIN_WOODLAND => 24, //temperate forest
						RAIN_WOODLAND..=RAIN_FOREST => 24, //temperate forest
						RAIN_FOREST..=RAIN_MAX => 27, //temperate swamp
						_ => 100,
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => 7, //temperate desert
				RAIN_DESERT..=RAIN_GRASSLAND => 18, //temperate grassland
				RAIN_GRASSLAND..=RAIN_WOODLAND => 21, //temperate woodland
				RAIN_WOODLAND..=RAIN_FOREST => 24, //temperate forest
				RAIN_FOREST..=RAIN_MAX => 27, //temperate swamp
				_ => 100,
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 7, //temperate desert
					RAIN_DESERT..=RAIN_GRASSLAND => 18, //temperate grassland
					RAIN_GRASSLAND..=RAIN_WOODLAND => 21, //temperate woodland
					RAIN_WOODLAND..=RAIN_FOREST => 24, //temperate forest
					RAIN_FOREST..=RAIN_MAX => 30, //temperate rainforest
					_ => 100,
				}
			}
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 7, //temperate desert
					RAIN_DESERT..=RAIN_GRASSLAND => 18, //temperate grassland
					RAIN_GRASSLAND..=RAIN_MAX => 33, //temperate shrubland
					_ => 100,
				}
			}
			ELEV_HIGHLANDS..=ELEV_ALPINE => {
				match rain {
					RAIN_MIN..=RAIN_WOODLAND => 7, //temperate desert
					RAIN_WOODLAND..=RAIN_MAX => 36, //temperate alpine grassland
					_ => 100,
				}
			}
			ELEV_ALPINE..=ELEV_MAX => 14, //temperate mountain tops
			_ => 100,
		},

		//▒▒▒▒▒▒▒▒▒ TROPICAL ▒▒▒▒▒▒▒▒▒▒▒
		TEMP_TEMPERATE..=TEMP_MAX => match elev {
			ELEV_MIN => {
				if watermask != NO_WATER {
					4 //tropical waters
				} else {
					//river basin biomes
					match rain {
						RAIN_MIN..=RAIN_DESERT => 19, //tropical grassland
						RAIN_DESERT..=RAIN_GRASSLAND => 22, //tropical woodland
						RAIN_GRASSLAND..=RAIN_WOODLAND => 25, //tropical forest
						RAIN_WOODLAND..=RAIN_FOREST => 25, //tropical forest
						RAIN_FOREST..=RAIN_MAX => 28, //tropical swamp
						_ => 100,
					}
				}
			}
			ELEV_MIN..=ELEV_WATERHOLD => match rain {
				RAIN_MIN..=RAIN_DESERT => 8, //tropical desert
				RAIN_DESERT..=RAIN_GRASSLAND => 19, //tropical grassland
				RAIN_GRASSLAND..=RAIN_WOODLAND => 22, //tropical woodland
				RAIN_WOODLAND..=RAIN_FOREST => 25, //tropical forest
				RAIN_FOREST..=RAIN_MAX => 28, //tropical swamp
				_ => 100,
			},
			ELEV_WATERHOLD..=ELEV_LOWLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 8, //tropical desert
					RAIN_DESERT..=RAIN_GRASSLAND => 19, //tropical grassland
					RAIN_GRASSLAND..=RAIN_WOODLAND => 22, //tropical woodland
					RAIN_WOODLAND..=RAIN_FOREST => 25, //tropical forest
					RAIN_FOREST..=RAIN_MAX => 31, //tropical rainforest
					_ => 100,
				}
			}
			ELEV_LOWLANDS..=ELEV_HIGHLANDS => {
				match rain {
					RAIN_MIN..=RAIN_DESERT => 8, //tropical desert
					RAIN_DESERT..=RAIN_GRASSLAND => 19, //tropical grassland
					RAIN_GRASSLAND..=RAIN_MAX => 34, //tropical shrubland
					_ => 100,
				}
			}
			ELEV_HIGHLANDS..=ELEV_ALPINE => {
				match rain {
					RAIN_MIN..=RAIN_GRASSLAND => 8, //tropical desert
					RAIN_GRASSLAND..=RAIN_MAX => 37, //tropical alpine grassland
					_ => 100,
				}
			}
			ELEV_ALPINE..=ELEV_MAX => 15, //tropical mountain tops
			_ => 100,
		}, //elev
		_ => 100,
	} //temp
}
