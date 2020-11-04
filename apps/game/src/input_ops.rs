use crate::*;

//▒▒▒▒▒▒▒▒▒ COMMAND REGISTRY ▒▒▒▒▒▒▒▒▒▒
#[derive(Clone)]
pub enum Reply {
	//General
	NoReply,
	Quit,
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
}

//Expected user inputs
//Move to .toml into locales
pub fn get_commands() -> Vec<String> {
	[
		//Movement directions
		"north".to_string(),
		"east".to_string(),
		"south".to_string(),
		"west".to_string(),
		//teleport
		"x".to_string(),
		"y".to_string(),
		//Common actions
		"?".to_string(),
		"q".to_string(),
		"render surrounding".to_string(),
	]
	.to_vec()
}

//▒▒▒▒▒▒▒▒▒▒ INPUT HANDLING ▒▒▒▒▒▒▒▒▒▒▒
pub fn parse_input(
	gd: &mut GameData,
	gs: &GameStrings,
) -> Vec<Reply> {
	//Init the input reply sequence
	let mut reply = Vec::new();

	//User input handling
	let mut input = prompts::new_line_io("", &gs.ui_el.prompt2);
	input = prompts::autocomplete(&input, &gd.commands);
	println!("{}", &gs.ui_el.separator2);

	//Movement directions
	match input.as_str() {
		//▒▒▒▒▒▒▒▒▒▒▒▒ MOVE ▒▒▒▒▒▒▒▒▒▒▒▒▒
		"west" => {
			reply.push(Reply::MoveWest);
		}
		"north" => {
			reply.push(Reply::MoveNorth);
		}
		"east" => {
			reply.push(Reply::MoveEast);
		}
		"south" => {
			reply.push(Reply::MoveSouth);
		}
		//▒▒▒▒▒▒▒▒▒▒▒▒ TELEPORT ▒▒▒▒▒▒▒▒▒▒▒▒▒
		"x" => {
			reply.push(Reply::TeleportX);
		}
		"y" => {
			reply.push(Reply::TeleportY);
		}
		//▒▒▒▒▒▒▒▒▒▒▒▒ GENERIC ▒▒▒▒▒▒▒▒▒▒▒▒▒
		"q" => {
			reply.push(Reply::Quit);
		}
		"render surrounding" => {
			reply.push(Reply::MapRenderLand);
		}
		"?" => {
			reply.push(Reply::PrintHelp);
		}

		&_ => {
			reply.push(Reply::NoReply);
		}
	}
	reply
}
