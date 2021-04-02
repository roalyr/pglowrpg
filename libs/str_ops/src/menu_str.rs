use crate::print_paragraph;
use crate::print_paragraph_with_color;
use crate::print_paragraph_with_escape;
use crate::print_paragraph_with_var;
use crate::return_paragraph;
use crate::MnStrings;
use textwrap::{fill, termwidth, Options};

print_paragraph_with_escape! {
	MnStrings(
	print_menu, "menu";
	)
}

return_paragraph! {
	MnStrings(
	str_game_title, "game_title";
	str_game_name, "game_name";
	str_developer, "developer";
	str_menu_title, "menu_title";
	)
}
