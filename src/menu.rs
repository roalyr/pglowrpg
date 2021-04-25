use text_ops::{prompt_option, MS, UI};

pub fn start() {
	UI.print_newline();
	UI.print_separator_thick("");
	UI.print_newline();
	UI.print_banner_block(MS.game_title());
	UI.print_banner_empty(MS.game_name());
	UI.print_banner_empty(MS.developer());
	UI.print_newline();
	UI.print_separator_thick("");

	loop {
		UI.print_newline();
		UI.print_banner_dash(MS.menu_title());
		UI.print_newline();
		MS.print_menu();

		let input = prompt_option();
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
