use serde::{Deserialize, Serialize};

//▒▒▒▒▒▒▒▒▒▒▒▒ GENERIC ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const ONE_USIZE: usize = 1;
pub const ONE_U16: u16 = 1;
pub const ONE_U32: u32 = 1;
pub const ONE_F32: f32 = 1.0;

pub const ZERO_USIZE: usize = 0;
pub const ZERO_U8: u8 = 0;
pub const ZERO_U16: u16 = 0;
pub const ZERO_F32: f32 = 0.0;

pub const VAL_255_F32: f32 = 255.0;
pub const VAL_127_F32: f32 = 127.0;

//▒▒▒▒▒▒▒▒▒▒▒▒ IDs ▒▒▒▒▒▒▒▒▒▒▒▒▒
// IDs that are written into BitLayer map.
pub const ID_MAP_NO_U32: u32 = 0;
pub const ID_MAP_MIN_U32: u32 = 1;

// IDs that are written into tables.
pub const UID_MIN_U32: u32 = 0;

//▒▒▒▒▒▒▒▒▒ WORLDGEN PRESET ▒▒▒▒▒▒▒▒▒▒▒
pub const DEFAULT_SEED: usize = 0;
pub const DEFAULT_WORLDS_NUM: usize = 1;

pub const ABS_ELEV_MIN: usize = 0;
pub const ABS_ELEV_MAX: usize = 10000;
pub const ABS_RAIN_MIN: usize = 0;
pub const ABS_RAIN_MAX: usize = 4000;
pub const ABS_TEMP_MIN: isize = -50;
pub const ABS_TEMP_MAX: isize = 50;

pub const TOPOG_SCOPE_MIN: f32 = 0.0;
pub const TOPOG_SCOPE_MAX: f32 = 100.0;
pub const TOPOG_CONTINUITY_MIN: f32 = 0.0;
pub const TOPOG_CONTINUITY_MAX: f32 = 100.0;
pub const TOPOG_CONCENTRATOR_MIN: f32 = 0.0;
pub const TOPOG_CONCENTRATOR_MAX: f32 = 100.0;
pub const TOPOG_FILTER_MIN: usize = 0;
pub const TOPOG_FILTER_MAX: usize = 255;

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
pub const RIVER_SEGMENT_LENGTH_MIN: usize = 1;
pub const RIVER_SEGMENT_DISPLACEMENT_MIN: f32 = 0.0;
pub const RIVER_SEGMENT_DISPLACEMENT_MAX: f32 = 100.0;
pub const RIVER_MIN_LENGTH: usize = 10;
pub const RIVER_ATTR_POOL_SIZE_POW_MIN: u16 = 1;
pub const RIVER_ATTR_POOL_SIZE_POW_MAX: u16 = 28;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MIN: u16 = 1;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MAX: u16 = 28;
pub const RIVER_EROSION_WIDTH_MIN: usize = 0;
pub const RIVER_EROSION_WIDTH_MAX: usize = 255;
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

pub const MAP_SIZES: [usize; 7] = [256, 512, 1024, 2048, 4096, 8192, 16384];

pub const NOISEMAP_SIZES: [usize; 6] = [256, 512, 1024, 2048, 4096, 8192];

//▒▒▒▒▒▒▒▒▒▒ TEMPERATURE ▒▒▒▒▒▒▒▒▒▒▒▒
pub const TEMP_MIN: isize = ABS_TEMP_MIN;
pub const TEMP_PERM_ICE: isize = -20;
pub const TEMP_POLAR: isize = -10;
pub const TEMP_TUNDRA: isize = 0;
pub const TEMP_BOREAL: isize = 15;
pub const TEMP_TEMPERATE: isize = 30;
pub const TEMP_MAX: isize = ABS_TEMP_MAX;

//▒▒▒▒▒▒▒▒▒▒▒ ELEVATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const ELEV_MIN: usize = ABS_ELEV_MIN;
pub const ELEV_WATERHOLD: usize = 100;
pub const ELEV_LOWLANDS: usize = 900;
pub const ELEV_HIGHLANDS: usize = 2000;
pub const ELEV_ALPINE: usize = 4000;
pub const ELEV_MAX: usize = ABS_ELEV_MAX;

//▒▒▒▒▒▒▒▒▒▒▒▒ RAINFALL ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const RAIN_MIN: usize = ABS_RAIN_MIN;
pub const RAIN_DESERT: usize = 160;
pub const RAIN_GRASSLAND: usize = 640;
pub const RAIN_WOODLAND: usize = 1400;
pub const RAIN_FOREST: usize = 2200;
pub const RAIN_MAX: usize = ABS_RAIN_MAX;

//▒▒▒▒▒▒▒▒▒▒▒ WATERMASK ▒▒▒▒▒▒▒▒▒▒▒▒
pub const NO_WATER: u16 = 0;

//▒▒▒▒▒▒▒▒▒▒▒▒ RIVERS ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const RIVER_HEUR_INIT: usize = 1_000_000;
pub const RIVER_MIN_WIDTH: u16 = 1;
pub const RIVER_MAX_WIDTH: u16 = 12;
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
