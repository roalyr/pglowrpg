use crate::print_paragraph;
use crate::GmStrings;
use lib_game_options::OPTIONS;
use textwrap::{fill, termwidth, Options};

print_paragraph! {
	"color";
	GmStrings(
	print_banner, "gm1";
	print_menu, "gm2";
	)
}
