use crate::print_paragraph;
use crate::print_paragraph_with_color;
use crate::print_paragraph_with_escape;
use crate::print_paragraph_with_var;
use crate::return_paragraph;
use crate::WgStrings;
use textwrap::{fill, termwidth, Options};

print_paragraph! {
	WgStrings(
	print_no_input_preset, "wg28";
	print_seed_rand, "wg4";
	print_prep_topog, "wg7";
	print_prep_climate, "wg9";
	print_prep_wmask, "wg13";
	print_prep_rmap, "wg17";
	print_prep_biome, "wg19";
	print_prep_georeg, "wg21";
	print_write_data, "wg25";
	print_write_no_data, "wg26";
	print_write_color, "wg15";
	print_write_raw, "wg16";
	print_done_worldgen, "wg23";
	)
}

print_paragraph_with_color! {
	WgStrings(
	print_intro, "wg1", (0, 100, 0);
	)
}

print_paragraph_with_var! {
	WgStrings(
	print_world_num, "wg6", usize;
	print_seed_used, "wg5", usize;
	)
}

return_paragraph! {
	WgStrings(
	str_banner_title, "wg0";
	str_sel_preset, "wg3";
	str_seed_rand, "wg2";
	str_world_num, "wg24";
	)
}
