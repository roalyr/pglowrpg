use crate::*;

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 3/4 ▒▒▒▒▒▒▒▒
#[derive(Clone)]
pub enum Command {
	NoInput,
	MapRenderLand,
	PrintHelp,
	Quit,
	Test,
	North,
	East,
	South,
	West,
	CoordX,
	CoordY,
}

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 4/4 ▒▒▒▒▒▒▒▒
pub fn process_input(
	gd: &mut GameData,
	gs: &GameStrings,
) -> bool {
	//This will determine if we should stop the main loop
	//which is the caller to this function
	let mut continue_loop = true;

	//let commands sequences translate into a state/action vector
	//which is FIRST filled up, and then flushed into actions in order
	//to not to parse commands one by one and execute sequences
	//of actions prematurely

	//Another way to do this is to parse an iterator is a specific way:
	//Action, Object, Direction, Option. This could be done by spawning
	//Different sub-processes, so that Action could wait for Object,
	//Direction and Option and then take them as arguments.
	//Or for concurrent actions.
	//A copy of an iter could be sent to sub-processes for them to seek
	//And yield all the necessary information.
	//A pattern matching fuction then could be called recursively depending
	//on the action, thus is must have a well-defined interface.

	//Commands could then have an embedded attribute (Action, Object,
	// Direction or Option) for extra matching.

	//Vec of tuple of enum
	let input = parse_input(gd, gs);
	for t in input.iter() {
		match t {
			//Default
			Command::NoInput => {}
			Command::MapRenderLand => {
				gd.map_render_size = prompt_input!({})
					//&gs.gm_str.gm25, &gs.ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.unwrap_or(gd.map_render_size);
				//println!("{}", &gs.ui_el.separator2);
				let cx = gd.x;
				let cy = gd.y;
				map_render_land(gd, cx, cy);
				//println!("{}", &gs.ui_el.separator2);
			}
			Command::PrintHelp => {
				//println!("{}", &gs.gm_str.gm2);
				//Make this better
				println!("Registered commands are:\n{:?}", &gd.commands_vec);
				//println!("{}", &gs.ui_el.separator2);
				//to hold the loop or browse topics (planned feature)
				prompt_input!({});
			}
			Command::Quit => {
				continue_loop = false;
			}
			Command::Test => {}

			Command::North => {
				gd.x = gd.x.saturating_add(1);
				if gd.x >= gd.lp.wi.map_size {
					gd.x = gd.lp.wi.map_size - 1;
				}
			}
			Command::East => {
				gd.y = gd.y.saturating_add(1);
				if gd.y >= gd.lp.wi.map_size {
					gd.y = gd.lp.wi.map_size - 1;
				}
			}
			Command::South => {
				gd.x = gd.x.saturating_sub(1);
			}
			Command::West => {
				gd.y = gd.y.saturating_sub(1);
			}
			Command::CoordX => {
				gd.x = input_coord(gs).unwrap_or(gd.x);
				if gd.x >= gd.lp.wi.map_size {
					gd.x = gd.lp.wi.map_size - 1;
				};
			}
			Command::CoordY => {
				gd.y = input_coord(gs).unwrap_or(gd.y);
				if gd.y >= gd.lp.wi.map_size {
					gd.y = gd.lp.wi.map_size - 1;
				};
			}
		} //match
	} //for

	continue_loop
}
