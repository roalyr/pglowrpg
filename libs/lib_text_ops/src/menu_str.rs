use crate::MnStrings;
use crate::INTERFACE_STRINGS as UI;
use crate::{print_menu, return_string};
use dep::textwrap::{fill, termwidth, Options};
use lib_game_options::OPTIONS;
use lib_io_ops::readron::palettes;

// Menu
print_menu! {
	palettes::text_colors::get().menu;
	UI.s["menu_left_bracket"]; UI.s["menu_right_bracket"];
	MnStrings(
	print_menu, "menu_prompt",
		[
			"menu_entry_game",
			"menu_entry_worldgen",
			"menu_entry_settings",
			"menu_entry_dev_test_corner",
			"menu_entry_quit",
		];
	)
}

// Banner strings
return_string! {
	MnStrings(
	game_title, "game_title";
	game_name, "game_name";
	developer, "developer";
	menu_title, "menu_title";
	)
}
