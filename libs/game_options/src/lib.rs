use constants::app::*;
use io_ops::file_to_string;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
use std::path::Path;

// This is to initialize the options once and for all.
lazy_static! {
	pub static ref OPTIONS: Stuff = get_options();
}

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	//General options
	pub locale: String,
	pub use_textwrap: bool,
	pub use_text_colors: bool,
	pub repeat_text_if_no_input: bool,

	//Worldgen options
	pub worlds_to_generate: usize,

	pub render_topography: bool,
	pub render_temperature: bool,
	pub render_rainfall: bool,
	pub render_rivers: bool,
	pub render_biomes: bool,
	pub render_bioregions: bool,
	pub render_georegions: bool,

	pub render_colorized_maps: bool,
	pub render_raw_maps: bool,
	pub write_data_files: bool,
}

pub fn get_options() -> Stuff {
	let path = Path::new(PATH_OPTIONS)
		.join(NAME_OPTIONS)
		.with_extension(EXTENSION_OPTION);

	let data = file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap_or(""));
			std::process::exit(0);
		}
	};
	stuff
}
