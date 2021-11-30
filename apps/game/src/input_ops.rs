use crate::action_ops::Command;
use crate::struct_ops::{GameData, GameStrings};

use lib_game_options::OPTIONS;
use lib_text_ops::prompt_input;

//▒▒▒▒▒▒▒▒▒▒ INPUT HANDLING ▒▒▒▒▒▒▒▒▒▒▒
pub fn parse_input(
	gd: &mut GameData,
	_gs: &GameStrings,
) -> Vec<Command> {
	//Init the input reply sequence which will be returned to the game
	let mut seq = Vec::new();
	let input = prompt_input!("word"; &gd.commands_vec; {});

	//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 2/4 ▒▒▒▒▒▒▒▒
	match input.as_str() {
		//Move
		i if i == gd.commands.move_west => {
			seq.push(Command::West);
		}
		i if i == gd.commands.move_north => {
			seq.push(Command::North);
		}
		i if i == gd.commands.move_east => {
			seq.push(Command::East);
		}
		i if i == gd.commands.move_south => {
			seq.push(Command::South);
		}
		//Teleport
		i if i == gd.commands.teleport_x => {
			seq.push(Command::CoordX);
		}
		i if i == gd.commands.teleport_y => {
			seq.push(Command::CoordY);
		}
		//General
		i if i == gd.commands.quit => {
			seq.push(Command::Quit);
		}
		i if i == gd.commands.map_render_land => {
			seq.push(Command::MapRenderLand);
		}
		i if i == gd.commands.print_help => {
			seq.push(Command::PrintHelp);
		}
		i if i == gd.commands.test => {
			seq.push(Command::Test);
		}

		&_ => {
			seq.push(Command::NoInput);
		}
	}
	seq
}

pub fn input_coord(_gs: &GameStrings) -> Result<u32, std::num::ParseIntError> {
	prompt_input!("num"; {}).trim().parse::<u32>()
}
