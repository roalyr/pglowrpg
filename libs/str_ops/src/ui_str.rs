use crate::print_banner;
use crate::print_paragraph;
use crate::return_banner;
use crate::return_string;
use crate::UiStrings;
use textwrap::{fill, termwidth, Options};
use io_ops::readron::palettes;

// Special characters
print_paragraph! {
	[]; // Tabs and newlines allowed.
	palettes::text_colors::get().normal;
	UiStrings(
	print_newline, "newline";
	)
}

// Banners and separators (with and without titles in them)
print_banner! {
	palettes::text_colors::get().banner;
	UiStrings(
	print_banner1, "banner1";
	print_banner2, "banner2";
	print_banner3, "banner3";
	print_banner1_col, "banner1";
	print_banner2_col, "banner2";
	print_banner3_col, "banner3";
	print_sep1, "separator1";
	print_sep2,"separator2";
	)
}

return_string! {
	UiStrings(
	str_newline, "newline";
	str_bul1, "bullet1";
	str_prompt1, "prompt1";
	str_prompt2, "prompt2";
	)
}

return_banner! {
	UiStrings(
	str_sep1, "separator1";
	str_sep2, "separator2";
	)
}
