use crate::print_paragraph;
use crate::return_string;
use crate::WgStrings;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

// Ordinary text
print_paragraph! {
	['\t', '\n'];  // No tabs and newline characters.
	palettes::text_colors::get().normal;
	WgStrings(
	print_intro, "wg1";
	print_no_input_preset, "wg28";
	print_seed_rand, "wg4";
	print_prompt_seed_rand, "wg2";
	print_prompt_world_num, "wg24";
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
	)
}

// Announcement without variables
print_paragraph! {
	['\t', '\n'];  // No tabs and newline characters.
	palettes::text_colors::get().announcement;
	WgStrings(
	print_done_worldgen, "wg23";
	)
}

// Announcement with number
print_paragraph! {
	['\t', '\n'];  // No tabs and newline characters.
	palettes::text_colors::get().announcement; //Main color
	palettes::text_colors::get().number; //Number color
	WgStrings(
	print_world_num, "wg6";
	print_seed_used, "wg5";
	)
}

return_string! {
	WgStrings(
	str_banner_title, "wg0";
	str_sel_preset, "wg3";
	
	)
}
