use serde::Deserialize;

//▒▒▒▒▒▒▒▒▒▒▒▒ GENERIC ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub const ONE_USIZE: usize = 1;
pub const ONE_U16: u16 = 1;
pub const ZERO_USIZE: usize = 0;

pub const NO_VAL_U8: u8 = 0;
pub const NO_VAL_U16: u16 = 0;

//Zero is reserved for none
pub const INIT_ID_U16: u16 = 1;
pub const NONE_ID_U16: u16 = 0;

//▒▒▒▒▒▒▒▒▒ WORLDGEN PRESET ▒▒▒▒▒▒▒▒▒▒▒
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

#[derive(Copy, Clone, Deserialize)]
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
pub const RIVER_WIDTH_ORDER_MAX: u8 = 12;
pub const RIVER_ATTR_POOL_SIZE_POW_MIN: u8 = 1;
pub const RIVER_ATTR_POOL_SIZE_POW_MAX: u8 = 28;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MIN: u8 = 1;
pub const RIVER_SINK_MIN_POOL_SIZE_POW_MAX: u8 = 28;
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
pub const RIVER_VECT_ANGLE_MAX_DEVIATION_MAX: f32 =
	2.0 * std::f32::consts::PI;

pub const MAP_SIZES: [usize; 7] =
	[256, 512, 1024, 2048, 4096, 8192, 16384];

pub const NOISEMAP_SIZES: [usize; 3] = [256, 512, 1024];

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
pub const RIVER_MIN_WIDTH: u8 = 1;

//MASK
//The values themselves are within u8 range.
pub const NO_RIVER: u16 = 0;
pub const RIVER_SOURCE: u16 = 1;
pub const RIVER_BODY: u16 = 2;
pub const RIVER_END: u16 = 3;
pub const RIVER_WATERFALL: u16 = 4;
pub const RIVER_WATERFALLS_MUL: u16 = 5;
