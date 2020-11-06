use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "commands";

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 3/5 ▒▒▒▒▒▒▒▒
#[derive(Serialize, Deserialize)]
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
		.join(FILENAME)
		.with_extension(EXTENSION_LOCALE);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match toml::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
