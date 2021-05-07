pub mod action_ops;
pub mod data_ops;
pub mod formatting_ops;
pub mod input_ops;
pub mod printing_ops;
pub mod struct_ops;

use action_ops::*;
use data_ops::*;
use formatting_ops::*;
use input_ops::*;
use printing_ops::*;
use struct_ops::*;

use codec::LayerPack;
use colored::*;
use constants_app::*;
use game_options::OPTIONS;
use io_ops::decompress_to_memory;
use io_ops::readron::palettes::biomes;
use io_ops::readron::strings;
use io_ops::writepng::from_hex;
use std::path::Path;
use text_ops::{prompt_input, GS};
use units::translate;

pub fn start() {
	let input_locale = &OPTIONS.locale;
	let commands: strings::commands::Stuff =
		strings::commands::get(&input_locale);
	//Init game structs
	let mut gs = init_gs();
	let mut gd = match init_gd(&gs, commands) {
		//Selecting preset may return None
		Some(gd) => gd,
		_ => return,
	};

	//Welcoming message
	GS.print_banner();
	//Intro message
	GS.print_menu();

	//Main loop
	loop {
		//Game mechanics
		get_world_current(&mut gd);

		//UI printing and rendering
		get_strings_basic(&gd, &mut gs);
		print_strings_basic(&gs);

		//temporary here, for debug
		let cx = gd.x;
		let cy = gd.y;
		map_render_land(&mut gd, cx, cy);

		//Input and actions
		match process_input(&mut gd, &gs) {
			true => continue,
			false => return,
		}
	}
}
