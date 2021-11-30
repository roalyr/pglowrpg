//use slots::consts::*;
//use slots::slots::Slots;
//use constants::app::*;
//use game_data_codec::LayerPack;
//use game_options::OPTIONS;
//use io_ops::decompress_to_memory;
//use io_ops::readron::palettes::biomes;
//use io_ops::readron::strings;
//use io_ops::writepng::from_hex;
//use std::path::Path;
//use text_ops::{prompt_input, GS};
//use unit_systems::translate;

use colored::*;

pub mod entity_system;

pub fn start() {
	println!("{}", "DEV TESTING CORNER".red());
	println!("{}", "START\n".blue());

	entity_system::start();

	println!("{}", "\nEND".blue());
}
