use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "worldgen";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub worlds_to_generate: usize,
	pub default_preset: String,

	pub render_topography: bool,
	pub render_temperature: bool,
	pub render_rainfall: bool,
	pub render_rivers: bool,
	pub render_biomes: bool,
	pub render_georegions: bool,

	pub render_raw_maps: bool,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_OPTIONS)
		.join(FILENAME)
		.with_extension(EXTENSION_OPTION);

	let data = crate::file_to_string(&path);

	let stuff: Stuff = match toml::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
