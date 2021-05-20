use constants::app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "temperature";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub color_0: String,
	pub color_1: String,
	pub color_2: String,
	pub color_3: String,
	pub color_4: String,
	pub color_5: String,
	pub color_6: String,
	pub color_7: String,
	pub color_8: String,
	pub color_9: String,
	pub color_10: String,
	pub color_11: String,
	pub color_12: String,
	pub color_13: String,
	pub color_14: String,
	pub color_15: String,
	pub color_16: String,
	pub color_17: String,
	pub color_18: String,
	pub color_19: String,
	pub color_20: String,
	pub color_21: String,
	pub color_22: String,
	pub color_23: String,
	pub color_24: String,
	pub color_25: String,

	pub color_100: String,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_PRESETS_PALETTES)
		.join(FILENAME)
		.with_extension(EXTENSION_PRESET_PALETTE);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap_or(""));
			std::process::exit(0);
		}
	};
	stuff
}
