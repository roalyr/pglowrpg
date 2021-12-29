use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

//▒▒▒▒▒▒▒▒▒ WORLDGEN PRESET ▒▒▒▒▒▒▒▒▒▒▒
pub const DEFAULT_SEED: u32 = 0;
pub const DEFAULT_WORLDS_NUM: u32 = 1;

pub const ABS_ELEV_MIN: u32 = 0;
pub const ABS_ELEV_MAX: u32 = 10000;
pub const ABS_RAIN_MIN: u32 = 0;
pub const ABS_RAIN_MAX: u32 = 4000;
pub const ABS_TEMP_MIN: i32 = -50;
pub const ABS_TEMP_MAX: i32 = 50;

pub const TOPOG_SCOPE_MIN: f32 = 0.0;
pub const TOPOG_SCOPE_MAX: f32 = 100.0;
pub const TOPOG_CONTINUITY_MIN: f32 = 0.0;
pub const TOPOG_CONTINUITY_MAX: f32 = 100.0;
pub const TOPOG_CONCENTRATOR_MIN: f32 = 0.0;
pub const TOPOG_CONCENTRATOR_MAX: f32 = 100.0;
pub const TOPOG_FILTER_MIN: u32 = 0;
pub const TOPOG_FILTER_MAX: u32 = 255;

pub const TOPOG_EROSION_FACTOR_MIN: f32 = 0.0;
pub const TOPOG_EROSION_FACTOR_MAX: f32 = 100.0;
pub const TOPOG_NOISE_SIZE_MIN: f32 = 1.0;
pub const TOPOG_NOISE_SIZE_MAX: f32 = 100.0;
pub const TOPOG_NOISE_WEIGHT_MIN: f32 = 0.0;
pub const TOPOG_NOISE_WEIGHT_MAX: f32 = 100.0;

#[derive(Copy, Clone, Serialize, Deserialize, Debug)]
pub enum TempGrad {
	South,
	Neither,
	North,
	Both,
}

pub const TEMP_NOISE_SIZE_MIN: f32 = 1.0;
pub const TEMP_NOISE_SIZE_MAX: f32 = 100.0;
pub const TEMP_NOISE_WEIGHT_MIN: f32 = 0.0;
pub const TEMP_NOISE_WEIGHT_MAX: f32 = 100.0;

pub const RAIN_NOISE_SIZE_MIN: f32 = 1.0;
pub const RAIN_NOISE_SIZE_MAX: f32 = 100.0;
pub const RAIN_NOISE_WEIGHT_MIN: f32 = 0.0;
pub const RAIN_NOISE_WEIGHT_MAX: f32 = 100.0;

pub const RIVER_SOURCE_DENSITY_MIN: f32 = 0.0;
pub const RIVER_SOURCE_DENSITY_MAX: f32 = 100.0;
pub const RIVER_HEURISTIC_FACTOR_MIN: f32 = 0.0;
pub const RIVER_HEURISTIC_FACTOR_MAX: f32 = 200.0;
pub const RIVER_NOISE_SIZE1_MIN: f32 = 1.0;
pub const RIVER_NOISE_SIZE1_MAX: f32 = 100.0;
pub const RIVER_NOISE_SIZE2_MIN: f32 = 1.0;
pub const RIVER_NOISE_SIZE2_MAX: f32 = 100.0;
pub const RIVER_NOISE_BLEND_MIN: f32 = 0.0;
pub const RIVER_NOISE_BLEND_MAX: f32 = 100.0;
pub const RIVER_NOISE_WEIGHT_MIN: f32 = 0.0;
pub const RIVER_NOISE_WEIGHT_MAX: f32 = 100.0;
pub const RIVER_SEGMENT_LENGTH_MIN: u32 = 1;
pub const RIVER_SEGMENT_DISPLACEMENT_MIN: f32 = 0.0;
pub const RIVER_SEGMENT_DISPLACEMENT_MAX: f32 = 100.0;
pub const RIVER_MIN_LENGTH: u32 = 10;
pub const RIVER_ATTR_POOL_SIZE_POW_MIN: u16 = 1;
pub const RIVER_ATTR_POOL_SIZE_POW_MAX: u16 = 28;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MIN: u16 = 1;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MAX: u16 = 28;
pub const RIVER_EROSION_WIDTH_MIN: u32 = 0;
pub const RIVER_EROSION_WIDTH_MAX: u32 = 255;
pub const RIVER_EROSION_SMOOTH_MIN: f32 = 0.0;
pub const RIVER_EROSION_SMOOTH_MAX: f32 = 100.0;
pub const RIVER_RAND_VECTORS_MIN: f32 = 0.0;
pub const RIVER_RAND_VECTORS_MAX: f32 = 1.0;
pub const RIVER_VECT_ANGLE_MIN: f32 = 0.0;
pub const RIVER_VECT_ANGLE_MAX: f32 = 2.0 * std::f32::consts::PI;
pub const RIVER_VECT_ANGLE_NOISE_MIN: f32 = 0.0;
pub const RIVER_VECT_ANGLE_NOISE_MAX: f32 = 100.0;
pub const RIVER_VECT_ANGLE_MAX_DEVIATION_MIN: f32 = 0.0;
pub const RIVER_VECT_ANGLE_MAX_DEVIATION_MAX: f32 = 2.0 * std::f32::consts::PI;

pub const MAP_SIZES: [u32; 7] = [256, 512, 1024, 2048, 4096, 8192, 16384];

pub const NOISEMAP_SIZES: [u32; 6] = [256, 512, 1024, 2048, 4096, 8192];

//▒▒▒▒▒▒▒▒▒▒ TEMPERATURE ▒▒▒▒▒▒▒▒▒▒▒▒
pub const TEMP_MIN: i32 = ABS_TEMP_MIN;
pub const TEMP_PERM_ICE: i32 = -20;
pub const TEMP_POLAR: i32 = -10;
pub const TEMP_TUNDRA: i32 = 0;
pub const TEMP_BOREAL: i32 = 15;
pub const TEMP_TEMPERATE: i32 = 30;
pub const TEMP_MAX: i32 = ABS_TEMP_MAX;

//▒▒▒▒▒▒▒▒▒▒▒ ELEVATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const ELEV_MIN: u32 = ABS_ELEV_MIN;
pub const ELEV_WATERHOLD: u32 = 100;
pub const ELEV_LOWLANDS: u32 = 900;
pub const ELEV_HIGHLANDS: u32 = 2000;
pub const ELEV_ALPINE: u32 = 4000;
pub const ELEV_MAX: u32 = ABS_ELEV_MAX;

//▒▒▒▒▒▒▒▒▒▒▒▒ RAINFALL ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const RAIN_MIN: u32 = ABS_RAIN_MIN;
pub const RAIN_DESERT: u32 = 160;
pub const RAIN_GRASSLAND: u32 = 640;
pub const RAIN_WOODLAND: u32 = 1400;
pub const RAIN_FOREST: u32 = 2200;
pub const RAIN_MAX: u32 = ABS_RAIN_MAX;

//▒▒▒▒▒▒▒▒▒▒▒ WATERMASK ▒▒▒▒▒▒▒▒▒▒▒▒
pub const NO_WATER: u16 = 0;

//▒▒▒▒▒▒▒▒▒▒▒▒ RIVERS ▒▒▒▒▒▒▒▒▒▒▒▒▒
// This values just has to be large enough, it will be used for 1st
// iteration and later on it will be adjusted on the 2nd run.
pub const RIVER_HEUR_INIT: u32 = 1_000_000;

// This value should be around 5...50-ish in order for pathfinfing to be
// fast. It is the distance between the nodes between which A* will
// perform pathfinding, basically a key points to lead long paths.
pub const RIVER_PATHFINDING_SEGMENT_LENGTH: u32 = 15;
pub const RIVER_MIN_WIDTH: u16 = 1;
pub const RIVER_MAX_WIDTH: u16 = 12;

// This is the weight factor for temperature to affect river spawning.
// From 0.0 to 1.0, higher walues will make river spawning in hot
// regions, reducing chances for cold regions.
pub const RIVER_SPAWN_TEMPERATURE_INFLUENCE: f32 = 0.25;

//MASK
//The values themselves are within u8 range.
pub const NO_RIVER: u16 = 0;
pub const RIVER_SOURCE: u16 = 1;
pub const RIVER_BODY: u16 = 2;
pub const RIVER_END: u16 = 3;
pub const RIVER_WATERFALL: u16 = 4;
pub const RIVER_WATERFALLS_MUL: u16 = 5;

//▒▒▒▒▒▒▒▒▒▒▒▒ BIOMES ▒▒▒▒▒▒▒▒▒▒▒▒▒
// ID values for each biome (type ID).
pub const BIOME_WATER_ICECAP: u8 = 0;
pub const BIOME_ICY_WATERS: u8 = 1;
pub const BIOME_COLD_WATERS: u8 = 2;
pub const BIOME_TEMPERATE_WATERS: u8 = 3;
pub const BIOME_TROPICAL_WATERS: u8 = 4;

pub const BIOME_POLAR_ICE_DESERT: u8 = 5;
pub const BIOME_COLD_DESERT: u8 = 6;
pub const BIOME_TEMPERATE_DESERT: u8 = 7;
pub const BIOME_TROPICAL_DESERT: u8 = 8;

pub const BIOME_POLAR_SNOWY_GLACIER: u8 = 9;
pub const BIOME_BARREN_TUNDRA: u8 = 10;

pub const BIOME_POLAR_MOUNTAIN_TOP: u8 = 11;
pub const BIOME_TUNDRA_MOUNTAIN_TOP: u8 = 12;
pub const BIOME_BOREAL_MOUNTAIN_TOP: u8 = 13;
pub const BIOME_TEMPERATE_MOUNTAIN_TOP: u8 = 14;
pub const BIOME_TROPICAL_MOUNTAIN_TOP: u8 = 15;

pub const BIOME_TUNDRA_GRASSLAND: u8 = 16;
pub const BIOME_BOREAL_GRASSLAND: u8 = 17;
pub const BIOME_TEMPERATE_GRASSLAND: u8 = 18;
pub const BIOME_TROPICAL_GRASSLAND: u8 = 19;

pub const BIOME_BOREAL_WOODLAND: u8 = 20;
pub const BIOME_TEMPERATE_WOODLAND: u8 = 21;
pub const BIOME_TROPICAL_WOODLAND: u8 = 22;

pub const BIOME_BOREAL_FOREST: u8 = 23;
pub const BIOME_TEMPERATE_FOREST: u8 = 24;
pub const BIOME_TROPICAL_FOREST: u8 = 25;

pub const BIOME_BOREAL_SWAMP: u8 = 26;
pub const BIOME_TEMPERATE_SWAMP: u8 = 27;
pub const BIOME_TROPICAL_SWAMP: u8 = 28;

pub const BIOME_BOREAL_RAINFOREST: u8 = 29;
pub const BIOME_TEMPERATE_RAINFOREST: u8 = 30;
pub const BIOME_TROPICAL_RAINFOREST: u8 = 31;

pub const BIOME_BOREAL_SHRUBLAND: u8 = 32;
pub const BIOME_TEMPERATE_SHRUBLAND: u8 = 33;
pub const BIOME_TROPICAL_SHRUBLAND: u8 = 34;

pub const BIOME_BOREAL_ALPINE_GRASSLAND: u8 = 35;
pub const BIOME_TEMPERATE_ALPINE_GRASSLAND: u8 = 36;
pub const BIOME_TROPICAL_ALPINE_GRASSLAND: u8 = 37;

// Biomes codenames for presets parsing. From lazy static doc.
// This will be used when parsing flora / fauna preset ron files.
// Each entity will contain biome(s) it is native to as codenames.
// Match codenames against this hashmap to get IDs when doing
// worldgen.
lazy_static! {
	pub static ref BIOMES_CODENAMES: HashMap<String, u8> = {
		let mut m = HashMap::new();
		m.insert("biome_water_icecap".to_string(), BIOME_WATER_ICECAP);
		m.insert("biome_icy_waters".to_string(), BIOME_ICY_WATERS);
		m.insert("biome_cold_waters".to_string(), BIOME_COLD_WATERS);
		m.insert("biome_temperate_waters".to_string(), BIOME_TEMPERATE_WATERS);
		m.insert("biome_tropical_waters".to_string(), BIOME_TROPICAL_WATERS);
		m
	};
}
