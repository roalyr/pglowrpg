use crate::print_list;
use crate::print_menu;
use crate::print_paragraph;
use crate::return_string;
use crate::WgStrings;
use crate::UI;
use game_options::OPTIONS;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

// Normal text.
print_paragraph! {
	palettes::text_colors::get().normal;
	WgStrings(
	print_intro, "intro";

	print_no_input_preset, "wg28";

	print_seed_rand, "seed_rand";
	print_seed_man, "seed_man";
	print_seed_pres, "seed_pres";

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

// Normal text with NUMBER value.
print_paragraph! {
	palettes::text_colors::get().normal; //Main color
	palettes::text_colors::get().number; //Number color
	WgStrings(
	print_seed_default, "seed_default_value";
	print_seed_base, "seed_base";
	print_seed_used, "seed_used";

	print_world_num, "wg6";
	)
}

// Normal text with NAME value.
print_paragraph! {
	palettes::text_colors::get().normal; //Main color
	palettes::text_colors::get().name; //Name color
	WgStrings(
	print_preset_selected, "preset_selected";
	)
}

// List options / entries for selection from a vector.
print_list! {
	UI.s["bullet1"];
	palettes::text_colors::get().list;
	WgStrings(
	print_list_preset, "preset_prompt";
	)
}

// List menus.
print_menu! {
	palettes::text_colors::get().menu;
	UI.s["menu_left_bracket"]; UI.s["menu_right_bracket"];
	WgStrings(
	print_seed_menu, "seed_menu_prompt",
		[
			"seed_entry_man",
			"seed_entry_rand",
			"seed_entry_pres",
		];
	)
}

// Announcement without variables
print_paragraph! {
	palettes::text_colors::get().announcement;
	WgStrings(
	print_done_worldgen, "wg23";
	)
}

return_string! {
	WgStrings(
	str_banner_title, "wg0";
	)
}
