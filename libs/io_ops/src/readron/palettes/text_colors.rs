use constants::app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub normal: String,
	pub menu: String,
	pub announcement: String,
	pub prompt: String,
	pub banner: String,
	pub number: String,
	pub name: String,
	pub list: String,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_PRESETS_PALETTES)
		.join(NAME_PALETTE_TEXT_COLORS)
		.with_extension(EXTENSION_PRESET_PALETTE);

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
