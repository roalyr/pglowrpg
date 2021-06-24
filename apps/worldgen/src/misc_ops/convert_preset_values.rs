use constants::world::*;
use io_ops::readron::presets;

struct Cap2<T> {
	min: T,
	max: T,
}

pub fn all(wi: &mut presets::presets_worldgen::Stuff) {
	map_size(wi);
	noisemap_size(wi);
	params(wi);
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CHECK SIZES ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn map_size(wi: &presets::presets_worldgen::Stuff) {
	let size = MAP_SIZES.iter().find(|&&s| s == wi.map_size);
	if size == None {
		let msg = "ERROR: map size value is wrong: ";
		let msg2 = "Allowed values are: ";
		println!("{}{}", msg, wi.map_size,);
		println!("{}{:?}", msg2, MAP_SIZES);
		std::process::exit(0);
	}
}

fn noisemap_size(wi: &presets::presets_worldgen::Stuff) {
	let size = NOISEMAP_SIZES.iter().find(|&&s| s == wi.noisemap_size);
	if size == None {
		let msg = "ERROR: noisemap size value is wrong: ";
		let msg2 = "Allowed values are: ";
		println!("{}{}", msg, wi.noisemap_size,);
		println!("{}{:?}", msg2, NOISEMAP_SIZES);
		std::process::exit(0);
	}
	if wi.noisemap_size > wi.map_size {
		println!("ERROR: noisemap should be less or equal to map size");
		println!("Map size: {}", wi.map_size);
		println!("Noisemap size: {}", wi.noisemap_size);
		std::process::exit(0);
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CHECK VALUES ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn params(wi: &mut presets::presets_worldgen::Stuff) {
	wi.waterlevel = cap1("Waterlevel", wi.waterlevel, ABS_ELEV_MIN, ABS_ELEV_MAX);

	let abs_elev: Cap2<u32> = cap2(
		"Elevation",
		wi.abs_elev_min,
		wi.abs_elev_max,
		ABS_ELEV_MIN,
		ABS_ELEV_MAX,
	);

	wi.abs_elev_min = abs_elev.min;
	wi.abs_elev_max = abs_elev.max;

	let abs_rain: Cap2<u32> = cap2(
		"Rainfall",
		wi.abs_rain_min,
		wi.abs_rain_max,
		ABS_RAIN_MIN,
		ABS_RAIN_MAX,
	);

	wi.abs_rain_min = abs_rain.min;
	wi.abs_rain_max = abs_rain.max;

	let abs_temp: Cap2<i32> = cap2(
		"Temperature",
		wi.abs_temp_min,
		wi.abs_temp_max,
		ABS_TEMP_MIN,
		ABS_TEMP_MAX,
	);

	wi.abs_temp_min = abs_temp.min;
	wi.abs_temp_max = abs_temp.max;

	wi.topog_scope = cap1(
		"Topography scope",
		wi.topog_scope,
		TOPOG_SCOPE_MIN,
		TOPOG_SCOPE_MAX,
	);

	wi.topog_continuity = cap1(
		"Topography continuity",
		wi.topog_continuity,
		TOPOG_CONTINUITY_MIN,
		TOPOG_CONTINUITY_MAX,
	);

	wi.topog_concentrator = cap1(
		"Topography concentrator",
		wi.topog_concentrator,
		TOPOG_CONCENTRATOR_MIN,
		TOPOG_CONCENTRATOR_MAX,
	);

	wi.topog_filter = cap1(
		"Topography filter",
		wi.topog_filter,
		TOPOG_FILTER_MIN,
		TOPOG_FILTER_MAX,
	);

	wi.topog_erosion_factor = cap1(
		"Topography erosion factor",
		wi.topog_erosion_factor,
		TOPOG_EROSION_FACTOR_MIN,
		TOPOG_EROSION_FACTOR_MAX,
	);

	wi.topog_noise_size = cap1(
		"Topography noise size",
		wi.topog_noise_size,
		TOPOG_NOISE_SIZE_MIN,
		TOPOG_NOISE_SIZE_MAX,
	);

	wi.topog_noise_weight = cap1(
		"Topography noise weight",
		wi.topog_noise_weight,
		TOPOG_NOISE_WEIGHT_MIN,
		TOPOG_NOISE_WEIGHT_MAX,
	);

	wi.temp_noise_size = cap1(
		"Temperature noise size",
		wi.temp_noise_size,
		TEMP_NOISE_SIZE_MIN,
		TEMP_NOISE_SIZE_MAX,
	);

	wi.temp_noise_weight = cap1(
		"Temperature noise weight",
		wi.temp_noise_weight,
		TEMP_NOISE_WEIGHT_MIN,
		TEMP_NOISE_WEIGHT_MAX,
	);

	wi.rain_noise_size = cap1(
		"Rainfall noise size",
		wi.rain_noise_size,
		RAIN_NOISE_SIZE_MIN,
		RAIN_NOISE_SIZE_MAX,
	);

	wi.rain_noise_weight = cap1(
		"Rainfall noise weight",
		wi.rain_noise_weight,
		RAIN_NOISE_WEIGHT_MIN,
		RAIN_NOISE_WEIGHT_MAX,
	);

	wi.river_source_density = cap1(
		"River source density",
		wi.river_source_density,
		RIVER_SOURCE_DENSITY_MIN,
		RIVER_SOURCE_DENSITY_MAX,
	);

	wi.river_heuristic_factor = cap1(
		"River heuristic factor",
		wi.river_heuristic_factor,
		RIVER_HEURISTIC_FACTOR_MIN,
		RIVER_HEURISTIC_FACTOR_MAX,
	);

	wi.river_noise_size1 = cap1(
		"River noise size 1",
		wi.river_noise_size1,
		RIVER_NOISE_SIZE1_MIN,
		RIVER_NOISE_SIZE1_MAX,
	);

	wi.river_noise_size2 = cap1(
		"River noise size 2",
		wi.river_noise_size2,
		RIVER_NOISE_SIZE2_MIN,
		RIVER_NOISE_SIZE2_MAX,
	);

	wi.river_noise_blend = cap1(
		"River noise blend",
		wi.river_noise_blend,
		RIVER_NOISE_BLEND_MIN,
		RIVER_NOISE_BLEND_MAX,
	);

	wi.river_noise_weight = cap1(
		"River noise weight",
		wi.river_noise_weight,
		RIVER_NOISE_WEIGHT_MIN,
		RIVER_NOISE_WEIGHT_MAX,
	);

	wi.river_min_length = cap1(
		"River minimum length",
		wi.river_min_length,
		RIVER_MIN_LENGTH,
		wi.map_size,
	);

	wi.river_attr_pool_size_pow = cap1(
		"River attractor pool size 2^pow",
		wi.river_attr_pool_size_pow,
		RIVER_ATTR_POOL_SIZE_POW_MIN,
		RIVER_ATTR_POOL_SIZE_POW_MAX,
	);

	wi.river_sink_min_pool_size_pow = cap1(
		"River attractor sink minimum pool size 2^pow",
		wi.river_sink_min_pool_size_pow,
		RIVER_SINK_MIN_POOL_SIZE_POW_MIN,
		RIVER_SINK_MIN_POOL_SIZE_POW_MAX,
	);

	wi.river_erosion_width = cap1(
		"River erosion width",
		wi.river_erosion_width,
		RIVER_EROSION_WIDTH_MIN,
		RIVER_EROSION_WIDTH_MAX,
	);

	wi.river_erosion_smooth = cap1(
		"River erosion smoothness",
		wi.river_erosion_smooth,
		RIVER_EROSION_SMOOTH_MIN,
		RIVER_EROSION_SMOOTH_MAX,
	);

	//▒▒▒▒▒▒▒▒▒▒▒▒ CONVERSION ▒▒▒▒▒▒▒▒▒▒▒▒▒
	wi.topog_scope /= 72.5 * 100.0;
	wi.topog_continuity /= 1.3 * 100.0;
	wi.topog_concentrator /= 26.5 * 100.0;
	wi.topog_erosion_factor /= 100.0;
	wi.topog_noise_weight /= 100.0;
	wi.topog_noise_size =
		diminishing_scale(wi.noisemap_size, wi.topog_noise_size);

	wi.rain_noise_weight /= 100.0;
	wi.rain_noise_size =
		diminishing_scale(wi.noisemap_size, wi.rain_noise_size) / 1.5;

	wi.temp_noise_weight /= 100.0;
	wi.temp_noise_size =
		diminishing_scale(wi.noisemap_size, wi.temp_noise_size) / 2.0;

	//wi.river_max_length is ok
	//wi.river_attr_pool_size_pow is ok
	wi.river_noise_blend /= 100.0;
	wi.river_noise_weight /= 100.0;
	wi.river_source_density /= wi.map_size as f32 * 0.5;

	wi.river_heuristic_factor /= 100.0;
	wi.river_noise_size1 = 1.0 / wi.river_noise_size1.powf(1.2);

	wi.river_noise_size2 = 1.0 / wi.river_noise_size2.powf(1.2);

	wi.river_erosion_smooth /= 5.0;
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CAPPING FNS ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn cap1<T>(
	name: &str,
	val: T,
	cap_min: T,
	cap_max: T,
) -> T
where
	T: PartialOrd + std::fmt::Display,
{
	let warn = "WARNING:";
	let msg1 = "absolute value is capped, adjusting";
	let mut v = val;
	if v < cap_min {
		println!("{} {} {} {} => {}", warn, name, msg1, v, cap_min);
		v = cap_min;
	}
	if v > cap_max {
		println!("{} {} {} {} => {}", warn, name, msg1, v, cap_max);
		v = cap_max;
	}
	v
}

fn cap2<T>(
	name: &str,
	min: T,
	max: T,
	cap_min: T,
	cap_max: T,
) -> Cap2<T>
where
	T: PartialOrd + std::fmt::Display + Copy,
{
	let warn = "WARNING:";
	let msg1 = "absolute minimal value is capped, adjusting";
	let msg2 = "absolute maximum value is capped, adjusting";
	let msg3 = "minimal value is greater than maximum, adjusting maximum";
	let mut c = Cap2 { min, max };
	if c.min < cap_min {
		println!("{} {} {} {} => {}", warn, name, msg1, c.min, cap_min);
		c.min = cap_min;
	}
	if c.min > cap_max {
		println!("{} {} {} {} => {}", warn, name, msg1, c.min, cap_max);
		c.min = cap_max;
	}
	if c.max < c.min {
		println!("{} {} {} {} => {}", warn, name, msg3, c.max, c.min);
		c.max = c.min;
	}
	if c.max < cap_min {
		println!("{} {} {} {} => {}", warn, name, msg2, c.max, cap_min);
		c.max = cap_min;
	}
	if c.max > cap_max {
		println!("{} {} {} {} => {}", warn, name, msg2, c.max, cap_max);
		c.max = cap_max;
	}
	c
}

//▒▒▒▒▒▒▒▒▒▒▒ SCALES ▒▒▒▒▒▒▒▒▒▒▒▒▒
//0-100 scale
fn diminishing_scale(
	scale_max: u32,
	input: f32,
) -> f32 {
	100.0 / (input * scale_max as f32)
}
