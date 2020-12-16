use crate::*;

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 3/4 ▒▒▒▒▒▒▒▒
#[derive(Clone)]
pub enum Command {
	NoInput,
	MapRenderLand,
	PrintHelp,
	Quit,
	Test,
}

#[derive(Clone)]
pub enum Direction {
	NoInput,
	North,
	East,
	South,
	West,
	CoordX,
	CoordY,
}

#[derive(Clone)]
pub enum Action {
	NoInput,
	Move,
}

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 4/4 ▒▒▒▒▒▒▒▒
pub fn process_input(gd: &mut GameData, gs: &GameStrings) -> bool {
	//This will determine if we should stop the main loop
	//which is the caller to this function
	let mut continue_loop = true;

	//Vec of tuple of enum
	let t = parse_input(gd, gs).pop().unwrap();
	match t.0 {
		//Default
		Command::NoInput => {}
		Command::MapRenderLand => {
			gd.map_render_size =
				prompts::new_line_io(&gs.gm_str.gm25, &gs.ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.unwrap_or(gd.map_render_size);
			println!("{}", &gs.ui_el.separator2);
			let cx = gd.x;
			let cy = gd.y;
			map_render_land(gd, cx, cy);
			println!("{}", &gs.ui_el.separator2);
		}
		Command::PrintHelp => {
			println!("{}", &gs.gm_str.gm2);
			//Make this better
			println!("Registered commands are:\n{:?}", &gd.commands_vec);
			println!("{}", &gs.ui_el.separator2);
			//to hold the loop or browse topics (planned feature)
			prompts::new_line_io("", &gs.ui_el.prompt2);
		}
		Command::Quit => {
			continue_loop = false;
		}
		Command::Test => {}
	} //match

	match t.1 {
		Direction::NoInput => {}
		Direction::North => {
			gd.x = gd.x.saturating_add(1);
			if gd.x >= gd.lp.wi.map_size {
				gd.x = gd.lp.wi.map_size - 1;
			}
		}
		Direction::East => {
			gd.y = gd.y.saturating_add(1);
			if gd.y >= gd.lp.wi.map_size {
				gd.y = gd.lp.wi.map_size - 1;
			}
		}
		Direction::South => {
			gd.x = gd.x.saturating_sub(1);
		}
		Direction::West => {
			gd.y = gd.y.saturating_sub(1);
		}
		Direction::CoordX => {
			gd.x = input_coord(gs).unwrap_or(gd.x);
			if gd.x >= gd.lp.wi.map_size {
				gd.x = gd.lp.wi.map_size - 1;
			};
		}
		Direction::CoordY => {
			gd.y = input_coord(gs).unwrap_or(gd.y);
			if gd.y >= gd.lp.wi.map_size {
				gd.y = gd.lp.wi.map_size - 1;
			};
		}
	} //match

	match t.2 {
		Action::NoInput => {}
		Action::Move => {}
	} //match

	continue_loop
}
