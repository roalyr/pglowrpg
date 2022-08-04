use crate::UiStrings;
use crate::{print_banner, print_paragraph, return_string};
use dep::textwrap::{fill, termwidth, Options};
use lib_game_options::OPTIONS;
use lib_io_ops::readron::palettes;

// Special characters
print_paragraph! {
	palettes::text_colors::get().normal;
	UiStrings(
	print_newline, "newline";
	)
}

// Banners and separators (with and without titles in them).
// Those function may take a string as argument.
print_banner! {
	palettes::text_colors::get().banner;
	UiStrings(
	print_banner_empty, "banner_empty";
	print_banner_block, "banner_block";
	print_banner_dash, "banner_dash";
	print_separator_thin, "separator_thin";
	print_separator_thick,"separator_thick";
	)
}

return_string! {
	UiStrings(
	str_newline, "newline";
	str_bul1, "bullet1";
	)
}
