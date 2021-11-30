use lib_constants::app::*;
use lib_constants::world::TempGrad;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize, Debug)]
pub struct Stuff {
	pub seed: u32,
	pub abs_elev_min: u32,
	pub abs_elev_max: u32,
	pub abs_rain_min: u32,
	pub abs_rain_max: u32,
	pub abs_temp_min: i32,
	pub abs_temp_max: i32,
	pub waterlevel: u32,
	pub topog_scope: f32,
	pub topog_continuity: f32,
	pub topog_concentrator: f32,
	pub topog_filter: u32,
	pub topog_erosion_factor: f32,
	pub topog_noise_size: f32,
	pub topog_noise_weight: f32,
	pub temp_mode: TempGrad,
	pub temp_noise_size: f32,
	pub temp_noise_weight: f32,
	pub rain_noise_size: f32,
	pub rain_noise_weight: f32,
	pub river_source_density: f32,
	pub river_heuristic_factor: f32,
	pub river_noise_size1: f32,
	pub river_noise_size2: f32,
	pub river_noise_blend: f32,
	pub river_noise_weight: f32,
	pub river_min_length: u32,
	pub river_attr_pool_size_pow: u16,
	pub river_sink_min_pool_size_pow: u16,
	pub river_erosion_width: u32,
	pub river_erosion_smooth: f32,
	pub map_size: u32,
	pub noisemap_size: u32,

	pub magic: f32,
	pub magic1: f32,
	pub magic2: f32,
	pub magic3: f32,
}

pub fn get(input: &str) -> Stuff {
	//Check both default and user paths
	let path_def = Path::new(PATH_PRESETS_WORLD)
		.join(&input)
		.with_extension(EXTENSION_PRESET_PALETTE);

	let path_usr = Path::new(PATH_PRESETS_WORLD_USER)
		.join(&input)
		.with_extension(EXTENSION_PRESET_PALETTE);

	let path_vec = vec![path_def, path_usr];

	let data = crate::file_to_string(&path_vec);

	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}", e.to_string());
			std::process::exit(0);
		}
	};
	stuff
}
