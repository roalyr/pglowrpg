use crate::*;

//▒▒▒▒▒▒▒▒▒▒ INPUT HANDLING ▒▒▒▒▒▒▒▒▒▒▒
pub fn parse_input(
	gd: &mut GameData,
	gs: &GameStrings,
) -> Vec<Reply> {
	//Init the input reply sequence which will be returned to the game
	let mut reply = Vec::new();

	println!("{:?}", gd.commands_vec);

	//User input handling
	let mut input = prompts::new_line_io("", &gs.ui_el.prompt2);
	input = prompts::autocomplete(&input, &gd.commands_vec);
	println!("{}", &gs.ui_el.separator2);

	//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 4/5 ▒▒▒▒▒▒▒▒
	match input.as_str() {
		//Move
		i if i == gd.commands.move_west => {
			reply.push(Reply::MoveWest);
		}
		i if i == gd.commands.move_north => {
			reply.push(Reply::MoveNorth);
		}
		i if i == gd.commands.move_east => {
			reply.push(Reply::MoveEast);
		}
		i if i == gd.commands.move_south => {
			reply.push(Reply::MoveSouth);
		}
		//Teleport
		i if i == gd.commands.teleport_x => {
			reply.push(Reply::TeleportX);
		}
		i if i == gd.commands.teleport_y => {
			reply.push(Reply::TeleportY);
		}
		//General
		i if i == gd.commands.quit => {
			reply.push(Reply::Quit);
		}
		i if i == gd.commands.map_render_land => {
			reply.push(Reply::MapRenderLand);
		}
		i if i == gd.commands.print_help => {
			reply.push(Reply::PrintHelp);
		}
		i if i == gd.commands.test => {
			reply.push(Reply::Test);
		}

		&_ => {
			reply.push(Reply::NoReply);
		}
	}
	reply
}
