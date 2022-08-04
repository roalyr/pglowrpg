use crate::print_paragraph;
use crate::GmStrings;
use dep::textwrap::{fill, termwidth, Options};
use lib_game_options::OPTIONS;

print_paragraph! {
	"color";
	GmStrings(
	print_banner, "gm1";
	print_menu, "gm2";
	)
}
