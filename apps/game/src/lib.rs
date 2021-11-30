pub mod action_ops;
pub mod data_ops;
pub mod formatting_ops;
pub mod input_ops;
pub mod printing_ops;
pub mod struct_ops;

use lib_game_options::OPTIONS;
use lib_io_ops::readron::commands;
use lib_text_ops::game_strings as GS;

pub fn start() {
	let input_locale = &OPTIONS.locale;
	let commands: commands::Stuff = commands::get(&input_locale);
	//Init game structs
	let mut gs = struct_ops::init_gs();
	let mut gd = match struct_ops::init_gd(&gs, commands) {
		//Selecting preset may return None
		Some(gd) => gd,
		_ => return,
	};

	// Copy all the commands to the vector for autocomplete.
	// Must use toml here in order for the struct fields to be parsed.
	// Should be reworked when switching to hashmap.
	// Leaving unwraps as is for now.
	let temp_str = toml::to_string(&gd.commands).unwrap();
	let parsed = temp_str.parse::<toml::Value>().unwrap();
	for (_, v) in parsed.as_table().unwrap().iter() {
		gd.commands_vec.push((v.as_str().unwrap()).to_string());
	}

	//Welcoming message
	GS.print_banner();
	//Intro message
	GS.print_menu();

	//Main loop
	loop {
		//Game mechanics
		data_ops::get_world_current(&mut gd);

		//UI printing and rendering
		formatting_ops::get_strings_basic(&gd, &mut gs);
		printing_ops::print_strings_basic(&gs);

		//temporary here, for debug
		let cx = gd.x;
		let cy = gd.y;
		printing_ops::map_render_land(&mut gd, cy, cx);
		println!("Registered commands are:\n{:?}", &gd.commands_vec);

		//Input and actions
		match action_ops::process_input(&mut gd, &gs) {
			true => continue,
			false => return,
		}
	}
}
