use crate::{
	print_list, print_menu, print_paragraph, print_progress, return_string,
};
use crate::{WgStrings, UI};
use game_options::OPTIONS;
use io_ops::readron::palettes;
use textwrap::{fill, termwidth, Options};

// Normal text.
print_paragraph! {
	palettes::text_colors::get().normal;
	WgStrings(
	print_intro, "worldgen_intro";

	print_no_input_preset, "preset_not_found";

	print_seed_rand, "seed_rand";
	print_seed_man, "seed_man";
	print_seed_pres, "seed_pres";

	print_prompt_world_num, "world_num_prompt";

	print_prep_topog, "prep_topog";
	print_prep_climate, "prep_climate";
	print_prep_wmask, "prep_wmask";
	print_prep_rmap, "prep_rmap";
	print_prep_biome, "prep_biome";
	print_prep_georeg, "prep_georeg";

	print_write_data, "write_data";
	print_write_no_data, "write_no_data";
	print_write_color, "write_img_col";
	print_write_raw, "write_img_raw";
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

	print_world_num_default, "world_num_default";
	print_world_num, "world_num";
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
	print_done_worldgen, "worldgen_done";
	)
}

// Numeric progress.
print_progress! {
	palettes::text_colors::get().normal; //Main color
	palettes::text_colors::get().number; //Number color
	WgStrings(
	print_progress_rivers, "progress_rivers";
	)
}

// Returns a string on demand where needed.
return_string! {
	WgStrings(
	str_banner_title, "worldgen_banner";
	)
}
