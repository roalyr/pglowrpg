use crate::{
	printable_strings, printable_strings_with_arg, returnable_strings,
};
use crate::{Locale, WS};
use textwrap::{fill, termwidth, Options};

printable_strings! {
	Locale(
	print_banner, "wg0";
	print_intro, "wg1";
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

printable_strings_with_arg! {
	Locale(
	print_world_num, "wg6", usize;
	print_seed_used, "wg5", usize;
	)
}

returnable_strings! {
	Locale(
	str_sel_preset, "wg3";
	str_seed_rand, "wg2";
	str_world_num, "wg24";
	)
}
