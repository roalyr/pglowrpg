use lib_constants::app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

// Moved to game module instead of lib_io_ops for convenience.
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct CommandsStrings {
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

pub fn get(input: &str) -> CommandsStrings {
	let path = Path::new(PATH_LOCALES)
		.join(&input)
		.join(NAME_STRINGS_COMMANDS)
		.with_extension(EXTENSION_LOCALE);
	let data = lib_io_ops::file_to_string(&vec![path.clone()]);
	let stuff: CommandsStrings = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap_or(""));
			std::process::exit(0);
		}
	};
	stuff
}
