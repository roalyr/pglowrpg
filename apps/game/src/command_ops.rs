use crate::*;

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 1/5 ▒▒▒▒▒▒▒▒
#[derive(Clone)]
pub enum Reply {
	//Default reply, no command associated
	NoReply,
	//Movement
	MoveNorth,
	MoveEast,
	MoveSouth,
	MoveWest,
	//Teleport
	TeleportX,
	TeleportY,
	//Game actions / menus
	MapRenderLand,
	PrintHelp,
	Quit,
	Test,
}

macro_rules! clone_to_command_vec {
    ($game_data: ident; $($field_name: ident), *) => {
		$($game_data.commands_vec.push($game_data.commands.$field_name.clone());)*
   	}
}

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 2/5 ▒▒▒▒▒▒▒▒
pub fn commands_autocomplete(gd: &mut GameData) {
	//Copy all the commands to the vector for autocomplete
	clone_to_command_vec!(gd;
		move_west,
		move_north,
		move_east,
		move_south,
		teleport_x,
		teleport_y,
		map_render_land,
		print_help,
		quit,
		test
	);
}
