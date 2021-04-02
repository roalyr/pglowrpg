use crate::print_paragraph;
use crate::print_paragraph_with_color;
use crate::print_paragraph_with_escape;
use crate::print_paragraph_with_var;
use crate::return_paragraph;
use crate::GmStrings;
use textwrap::{fill, termwidth, Options};

print_paragraph_with_escape! {
	GmStrings(
	print_banner, "gm1";
	print_menu, "gm2";
	)
}
