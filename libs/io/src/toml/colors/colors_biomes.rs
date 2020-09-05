use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH: &str = "presets/colorcodes/biomes.toml";

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
	pub color_26: String,
	pub color_27: String,
	pub color_28: String,
	pub color_29: String,
	pub color_30: String,
	pub color_31: String,
	pub color_32: String,
	pub color_33: String,
	pub color_34: String,
	pub color_35: String,
	pub color_36: String,
	pub color_37: String,
	pub color_100: String,
}

pub fn get() -> Stuff {
	let path = Path::new(&PATH);
	let mut file =
		File::open(&path).expect("no BIOME COLORS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read BIOME COLORS file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize BIOME COLORS");
	stuff
}
