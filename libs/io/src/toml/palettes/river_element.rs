use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH: &str = "presets/palettes/river_element.toml";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub color_0: String,

	pub color_1: String,
	pub color_2: String,
	pub color_3: String,
	pub color_4: String,
	pub color_5: String,

	pub color_100: String,
}

pub fn get() -> Stuff {
	let path = Path::new(&PATH);
	let mut file = File::open(&path)
		.expect("no RIVER ELEMENT COLORS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read RIVER ELEMENT COLORS file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize RIVER ELEMENT COLORS");
	stuff
}
