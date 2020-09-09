use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH: &str = "presets/palettes/river_width.toml";

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

	pub color_100: String,
}

pub fn get() -> Stuff {
	let path = Path::new(&PATH);
	let mut file =
		File::open(&path).expect("no RIVER WIDTH COLORS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read RIVER WIDTH COLORS file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize RIVER WIDTH COLORS");
	stuff
}
