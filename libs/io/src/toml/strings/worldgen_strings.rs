use serde::{Deserialize, Serialize};
use std::fs::File;
use std::io::Read;
use std::path::Path;

const PATH_PART1: &str = "locales/";

const PATH_PART2: &str = "/worldgen/worldgen_strings.toml";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub wg1: String,
	pub wg2: String,
	pub wg3: String,

	pub wg5: String,
	pub wg6: String,
	pub wg7: String,
	pub wg8: String,
	pub wg9: String,
	pub wg10: String,
	pub wg11: String,
	pub wg12: String,
	pub wg13: String,
	pub wg14: String,
	pub wg15: String,
	pub wg16: String,
	pub wg17: String,
	pub wg18: String,
	pub wg19: String,
	pub wg20: String,
	pub wg21: String,
	pub wg22: String,

	pub rg1: String,
	pub rg2: String,
	pub rg3: String,

	pub gr1: String,
}

pub fn get(input: &str) -> Stuff {
	//takes only locale code as arg
	let p_str = [PATH_PART1, &input, PATH_PART2].concat();

	let path = Path::new(&p_str);

	let mut file =
		File::open(&path).expect("no WORLDGEN STRINGS file/folder");

	let mut data = String::new();
	file.read_to_string(&mut data)
		.expect("unable to read WORLDGEN STRINGS");

	let stuff: Stuff = toml::from_str(&data)
		.expect("unable to deserialize WORLDGEN STRINGS");
	stuff
}
