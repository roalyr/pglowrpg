use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

#[derive(Copy, Clone, Serialize, Deserialize)]
pub enum InterpMode {
	Nearest,
	Mitchell,
}

const PATH: &str = "options/worldgen.toml";

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
}

pub fn get() -> Stuff {
	let path = Path::new(&PATH);
	let mut file =
		File::open(&path).expect("no WORLDGEN OPTIONS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read WORLDGEN OPTIONS file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize WORLDGEN OPTIONS");
	stuff
}
