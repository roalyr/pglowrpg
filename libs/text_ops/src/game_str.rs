use crate::print_paragraph;
use crate::GmStrings;
use textwrap::{fill, termwidth, Options};

print_paragraph! {
	[]; "color";
	GmStrings(
	print_banner, "gm1";
	print_menu, "gm2";
	)
}
