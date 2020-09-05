use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH: &str = "options/global.toml";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub locale: String,
}

pub fn get() -> Stuff {
	let path = Path::new(&PATH);
	let mut file =
		File::open(&path).expect("no GLOBAL OPTIONS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read GLOBAL OPTIONS file");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize GLOBAL OPTIONS");
	stuff
}
