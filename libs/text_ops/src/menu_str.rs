use crate::{print_menu, print_paragraph, return_string};
use crate::{MnStrings, UI};
use game_options::OPTIONS;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

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
