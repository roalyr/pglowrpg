use serde::Deserialize;
use std::fs::File;
use std::io::Read;
use std::path::Path;

use constants_world::*;

const PATH: &str = "presets/world/";

#[derive(Deserialize)]
pub struct Stuff {
	pub seed: usize,
	pub abs_elev_min: usize,
	pub abs_elev_max: usize,
	pub abs_rain_min: usize,
	pub abs_rain_max: usize,
	pub abs_temp_min: isize,
	pub abs_temp_max: isize,
	pub waterlevel: usize,
	pub topog_scope: f32,
	pub topog_continuity: f32,
	pub topog_concentrator: f32,
	pub topog_filter: usize,
	pub topog_erosion_factor: f32,
	pub topog_noise_size: f32,
	pub topog_noise_weight: f32,
	pub temp_mode: TempGrad,
	pub temp_noise_size: f32,
	pub temp_noise_weight: f32,
	pub rain_noise_size: f32,
	pub rain_noise_weight: f32,
	pub river_source_density: f32,
	pub river_segment_displacement: f32,
	pub river_heuristic_factor: f32,
	pub river_noise_size1: f32,
	pub river_noise_size2: f32,
	pub river_noise_blend: f32,
	pub river_noise_weight: f32,
	pub river_segment_length: usize,
	pub river_min_length: usize,
	pub river_attr_pool_size_pow: u16,
	pub river_sink_min_pool_size_pow: u16,
	pub river_erosion_width: usize,
	pub river_erosion_smooth: f32,
	pub river_rand_vectors: f32,
	pub river_vect_angle: f32,
	pub river_vect_angle_noise: f32,
	pub river_vect_angle_max_deviation: f32,
	pub map_size: usize,
	pub noisemap_size: usize,

	pub magic: f32,
	pub magic1: f32,
	pub magic2: f32,
	pub magic3: f32,
}

pub fn get(input: &str) -> Stuff {
	//takes only filename as arg
	let p_str = [PATH, &input, ".toml"].concat();

	let path = Path::new(&p_str);
	let mut file =
		File::open(&path).expect("no WORLDGEN PRESET file");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read WORLDGEN PRESET file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize WORLDGEN PRESET file");

	stuff
}
