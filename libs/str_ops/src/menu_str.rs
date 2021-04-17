use crate::print_paragraph;
use crate::return_string;
use crate::MnStrings;
use textwrap::{fill, termwidth, Options};
use io_ops::readron::palettes;

// Menu
print_paragraph! {
	[]; // Tabs and newlines allowed.
	palettes::text_colors::get().menu;
	MnStrings(
	print_menu, "menu";
	)
}

return_string! {
	MnStrings(
	str_game_title, "game_title";
	str_game_name, "game_name";
	str_developer, "developer";
	str_menu_title, "menu_title";
	)
}
