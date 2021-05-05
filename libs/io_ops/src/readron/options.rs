use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "options";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	//General options
	pub locale: String,

	//Worldgen options
	pub worlds_to_generate: usize,
	pub default_preset: String,

	pub render_topography: bool,
	pub render_temperature: bool,
	pub render_rainfall: bool,
	pub render_rivers: bool,
	pub render_biomes: bool,
	pub render_georegions: bool,

	pub render_colorized_maps: bool,
	pub render_raw_maps: bool,
	pub write_data_files: bool,

	pub use_textwrap: bool,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_OPTIONS)
		.join(FILENAME)
		.with_extension(EXTENSION_OPTION);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
