use lib_game_options::OPTIONS;
use lib_text_ops::prompt_input;
use lib_text_ops::INTERFACE_STRINGS as UI;
use lib_text_ops::MENU_STRINGS as MS;

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
		let input = prompt_input!(
			"opt";
		{
			UI.print_newline();
			UI.print_banner_dash(MS.menu_title());
			UI.print_newline();
			MS.print_menu();
		});

		match input.as_str() {
			"1" => game::start(),
			"2" => worldgen::start(),
			"3" => {} // Settings.
			"4" => dev_test_corner::start(),
			"5" => return,
			_ => continue,
		}
	}
}
