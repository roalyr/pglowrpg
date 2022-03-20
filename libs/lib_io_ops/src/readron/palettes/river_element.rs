use lib_constants::app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub color_0: String,

	pub color_1: String,
	pub color_2: String,
	pub color_3: String,
	pub color_4: String,
	pub color_5: String,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_PRESETS_PALETTES)
		.join(NAME_PALETTE_RIVER_ELEMENTS)
		.with_extension(EXTENSION_PRESET);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap_or(""));
			std::process::exit(0);
		}
	};
	stuff
}
