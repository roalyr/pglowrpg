use crate::*;

//▒▒▒▒▒▒ ADD NEW COMMANDS, STEP 5/5 ▒▒▒▒▒▒▒▒
pub fn process_input(
	gd: &mut GameData,
	gs: &GameStrings,
) -> bool {
	//This will determine if we should stop the main loop
	//which is the caller to this function
	let mut continue_loop = true;

	//For now just a single reply, thus "pop()"
	match parse_input(gd, gs).pop().unwrap() {
		//Default
		Reply::NoReply => {}

		//Movement
		Reply::MoveNorth => {
			gd.x = gd.x.saturating_add(1);
			if gd.x >= gd.lp.wi.map_size {
				gd.x = gd.lp.wi.map_size - 1;
			}
		}
		Reply::MoveEast => {
			gd.y = gd.y.saturating_add(1);
			if gd.y >= gd.lp.wi.map_size {
				gd.y = gd.lp.wi.map_size - 1;
			}
		}
		Reply::MoveSouth => {
			gd.x = gd.x.saturating_sub(1);
		}
		Reply::MoveWest => {
			gd.y = gd.y.saturating_sub(1);
		}
		//Teleport
		Reply::TeleportX => {
			gd.x = prompts::new_line_io(
				&gs.gm_str.gm7,
				&gs.ui_el.prompt2,
			)
			.trim()
			.parse::<usize>()
			.unwrap_or(gd.x);
			if gd.x >= gd.lp.wi.map_size {
				gd.x = gd.lp.wi.map_size - 1;
			}
		}
		Reply::TeleportY => {
			gd.y = prompts::new_line_io(
				&gs.gm_str.gm7,
				&gs.ui_el.prompt2,
			)
			.trim()
			.parse::<usize>()
			.unwrap_or(gd.y);
			if gd.y >= gd.lp.wi.map_size {
				gd.y = gd.lp.wi.map_size - 1;
			}
		}
		//General
		Reply::MapRenderLand => {
			gd.map_render_size = prompts::new_line_io(
				&gs.gm_str.gm25,
				&gs.ui_el.prompt2,
			)
			.trim()
			.parse::<usize>()
			.unwrap_or(gd.map_render_size);
			println!("{}", &gs.ui_el.separator2);
			let cx = gd.x;
			let cy = gd.y;
			map_render_land(gd, cx, cy);
			println!("{}", &gs.ui_el.separator2);
		}
		Reply::PrintHelp => {
			println!("{}", &gs.gm_str.gm2);
			//Make this better
			println!(
				"Registered commands are:\n{:?}",
				&gd.commands_vec
			);
			println!("{}", &gs.ui_el.separator2);
			//to hold the loop or browse topics (planned feature)
			prompts::new_line_io("", &gs.ui_el.prompt2);
		}
		Reply::Quit => {
			continue_loop = false;
		}
		Reply::Test => {}
	}

	continue_loop
}
