use crate::print_paragraph;
use crate::return_string;
use crate::MnStrings;
use game_options::OPTIONS;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

// Menu
print_paragraph! {
	[]; // Tabs and newlines allowed.
	palettes::text_colors::get().menu;
	MnStrings(
	print_menu, "menu";
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
