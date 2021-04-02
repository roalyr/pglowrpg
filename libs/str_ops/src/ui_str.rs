use crate::print_banner_with_title;
use crate::print_banner_with_title_color;
use crate::print_paragraph;
use crate::print_paragraph_with_color;
use crate::print_paragraph_with_escape;
use crate::print_paragraph_with_var;
use crate::print_separator;
use crate::return_banner;
use crate::return_paragraph;
use crate::UiStrings;
use textwrap::{fill, termwidth, Options};

print_paragraph_with_escape! {
	UiStrings(
	print_newline, "newline";
	)
}

print_separator! {
	UiStrings(
	print_sep1, "separator1";
	print_sep2,"separator2";
	)
}

print_banner_with_title! {
	UiStrings(
	print_banner1, "banner1", String;
	print_banner2, "banner2", String;
	print_banner3, "banner3", String;
	)
}

print_banner_with_title_color! {
	UiStrings(
	print_banner1_col, "banner1", String, (20, 30, 200);
	print_banner2_col, "banner2", String, (200, 30, 20);
	print_banner3_col, "banner3", String, (20, 200, 20);
	)
}

return_paragraph! {
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
