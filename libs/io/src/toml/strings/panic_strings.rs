use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH_PART1: &str = "locales/";

const PATH_PART2: &str = "/worldgen/panic_strings.toml";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub pm1: String,
	pub pm2: String,
	pub pm3: String,
	pub pm4: String,
	pub line: String,
}

pub fn get(input: &str) -> Stuff {
	//takes only locale code as arg
	let p_str = [PATH_PART1, &input, PATH_PART2].concat();

	let path = Path::new(&p_str);

	let mut file =
		File::open(&path).expect("no PANIC STRINGS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read PANIC STRINGS");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize PANIC STRINGS");
	stuff
}
