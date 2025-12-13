// [Revision: v0.1] [Path: libs/lib_text_ops/src/game_str.rs] [Date: 2025-12-13]
// Description: Game-related string output functions

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
