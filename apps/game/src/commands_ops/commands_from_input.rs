use crate::commands_ops::commands_registry::Commands;
use crate::struct_ops::{GameData, GameStrings};

use lib_game_options::OPTIONS;
use lib_text_ops::prompt_input;

pub fn parse_input(
	gd: &mut GameData,
	_gs: &GameStrings,
) -> Vec<Commands> {
	//Init the input reply sequence which will be returned to the game
	let mut seq = Vec::new();
	let input = prompt_input!("word"; &gd.commands_list; {});

	match input.as_str() {
		//Move
		i if i == gd.commands_struct.move_west => {
			seq.push(Commands::West);
		}
		i if i == gd.commands_struct.move_north => {
			seq.push(Commands::North);
		}
		i if i == gd.commands_struct.move_east => {
			seq.push(Commands::East);
		}
		i if i == gd.commands_struct.move_south => {
			seq.push(Commands::South);
		}
		//Teleport
		i if i == gd.commands_struct.teleport_x => {
			seq.push(Commands::CoordX);
		}
		i if i == gd.commands_struct.teleport_y => {
			seq.push(Commands::CoordY);
		}
		//General
		i if i == gd.commands_struct.quit => {
			seq.push(Commands::Quit);
		}
		i if i == gd.commands_struct.map_render_land => {
			seq.push(Commands::MapRenderLand);
		}
		i if i == gd.commands_struct.print_help => {
			seq.push(Commands::PrintHelp);
		}
		i if i == gd.commands_struct.test => {
			seq.push(Commands::Test);
		}

		&_ => {
			seq.push(Commands::NoInput);
		}
	}
	seq
}

pub fn input_coord(_gs: &GameStrings) -> Result<u32, std::num::ParseIntError> {
	prompt_input!("num"; {}).trim().parse::<u32>()
}
