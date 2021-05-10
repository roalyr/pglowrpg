use constants::app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 1/4 ▒▒▒▒▒▒▒▒
// TODO: check if this can also be rep'aced with a hashmap after
// implementing the algorithm to pre-check stgings.

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct Stuff {
	pub move_west: String,
	pub move_north: String,
	pub move_east: String,
	pub move_south: String,
	pub teleport_x: String,
	pub teleport_y: String,
	pub print_help: String,
	pub map_render_land: String,
	pub quit: String,
	pub test: String,
}

pub fn get(input: &str) -> Stuff {
	let path = Path::new(PATH_LOCALES)
		.join(&input)
		.join(NAME_STRINGS_COMMANDS)
		.with_extension(EXTENSION_LOCALE);
	let data = crate::file_to_string(&vec![path.clone()]);
	let stuff: Stuff = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
