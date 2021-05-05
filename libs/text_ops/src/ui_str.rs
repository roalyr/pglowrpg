use crate::print_banner;
use crate::print_paragraph;
use crate::return_string;
use crate::UiStrings;
use game_options::OPTIONS;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

// Special characters
print_paragraph! {
	[]; // Tabs and newlines allowed.
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
