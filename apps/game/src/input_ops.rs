use crate::*;

//▒▒▒▒▒▒▒▒▒▒ INPUT HANDLING ▒▒▒▒▒▒▒▒▒▒▒
pub fn parse_input(
	gd: &mut GameData, gs: &GameStrings,
) -> Vec<(Command, Direction, Action)> {
	//Init the input reply sequence which will be returned to the game
	let mut seq = Vec::new();

	//Copy all the commands to the vector for autocomplete
	let temp_str = toml::to_string(&gd.commands).unwrap();
	let parsed = temp_str.parse::<toml::Value>().unwrap();

	for (_, v) in parsed.as_table().unwrap().iter() {
		gd.commands_vec.push((v.as_str().unwrap()).to_string());
	}

	//User input handling
	let mut input = prompts::new_line_io("", &gs.ui_el.prompt2);
	input = prompts::autocomplete(&input, &gd.commands_vec);
	println!("{}", &gs.ui_el.separator2);

	//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 2/4 ▒▒▒▒▒▒▒▒
	match input.as_str() {
		//Move
		i if i == gd.commands.move_west => {
			seq.push((Command::NoInput, Direction::West, Action::Move));
		}
		i if i == gd.commands.move_north => {
			seq.push((Command::NoInput, Direction::North, Action::Move));
		}
		i if i == gd.commands.move_east => {
			seq.push((Command::NoInput, Direction::East, Action::Move));
		}
		i if i == gd.commands.move_south => {
			seq.push((Command::NoInput, Direction::South, Action::Move));
		}
		//Teleport
		i if i == gd.commands.teleport_x => {
			seq.push((
				Command::NoInput,
				Direction::CoordX,
				Action::Move,
			));
		}
		i if i == gd.commands.teleport_y => {
			seq.push((
				Command::NoInput,
				Direction::CoordY,
				Action::Move,
			));
		}
		//General
		i if i == gd.commands.quit => {
			seq.push((
				Command::Quit,
				Direction::NoInput,
				Action::NoInput,
			));
		}
		i if i == gd.commands.map_render_land => {
			seq.push((
				Command::MapRenderLand,
				Direction::NoInput,
				Action::NoInput,
			));
		}
		i if i == gd.commands.print_help => {
			seq.push((
				Command::PrintHelp,
				Direction::NoInput,
				Action::NoInput,
			));
		}
		i if i == gd.commands.test => {
			seq.push((
				Command::Test,
				Direction::NoInput,
				Action::NoInput,
			));
		}

		&_ => {
			seq.push((
				Command::NoInput,
				Direction::NoInput,
				Action::NoInput,
			));
		}
	}
	seq
}

pub fn input_coord(
	gs: &GameStrings,
) -> Result<usize, std::num::ParseIntError> {
	prompts::new_line_io(&gs.gm_str.gm7, &gs.ui_el.prompt2)
		.trim()
		.parse::<usize>()
}
