use str_ops::{MS, UI};
use ui::prompts;

pub fn start() {
	UI.print_newline();
	UI.print_sep1();
	UI.print_banner1(String::new());
	UI.print_banner1_col(MS.str_game_title());
	UI.print_banner1_col(MS.str_game_name());
	UI.print_banner2_col(MS.str_developer());
	UI.print_banner1(String::new());
	UI.print_sep1();
	UI.print_newline();

	loop {
		UI.print_banner3_col(MS.str_menu_title());
		MS.print_menu();
		let input = prompts::new_line_io("", &UI.str_prompt1());
		if input.is_empty() {
			continue;
		}
		match input.as_str() {
			"w" => worldgen::start(),
			"g" => game::start(),
			"q" => return,
			_ => {}
		}
	}
}
