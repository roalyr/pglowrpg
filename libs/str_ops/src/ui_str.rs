use crate::print_banner;
use crate::print_paragraph;
use crate::return_banner;
use crate::return_string;
use crate::UiStrings;
use textwrap::{fill, termwidth, Options};
use io_ops::readron::palettes;

//make a proper macro for this
print_banner! {
	palettes::text_colors::get().banner;
	UiStrings(
	print_newline, "newline", String;
	print_banner1, "banner1", String;
	print_banner2, "banner2", String;
	print_banner3, "banner3", String;
	print_banner1_col, "banner1", String;
	print_banner2_col, "banner2", String;
	print_banner3_col, "banner3", String;
	print_sep1, "separator1", String;
	print_sep2,"separator2", String;
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
