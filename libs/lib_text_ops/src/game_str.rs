// [Revision: v0.1] [Path: libs/lib_text_ops/src/game_str.rs] [Date: 2025-12-13]
// Description: Game-related string output functions

use crate::print_paragraph;
use crate::print_menu;
use crate::return_string;
use crate::GmStrings;
use crate::INTERFACE_STRINGS as UI;
use dep::textwrap::{fill, termwidth, Options};
use lib_game_options::OPTIONS;
use lib_io_ops::readron::palettes;

// Simple paragraphs without variables
print_paragraph! {
	palettes::text_colors::get().normal;
	GmStrings(
	// Character creation
	print_char_stats_prompt, "gm_char_stats_prompt";
	print_char_stats_default, "gm_char_stats_default";
	print_char_use_default, "gm_char_use_default";
	print_char_using_default, "gm_char_using_default";
	
	// Headers
	print_stats_header, "gm_stats_header";
	
	// Exploration
	print_exploration_prompt, "gm_exploration_prompt";
	
	// Combat
	print_combat_encounter, "gm_combat_encounter";
	print_combat_prompt, "gm_combat_prompt";
	print_combat_enemy_strikes, "gm_combat_enemy_strikes";
	print_combat_continue, "gm_combat_continue";
	print_combat_escape_success, "gm_combat_escape_success";
	
	// Camp
	print_camp_setup, "gm_camp_setup";
	
	// Vows
	print_vows_active, "gm_vows_active";
	print_vows_none, "gm_vows_none";
	print_vows_menu_prompt, "gm_vows_menu_prompt";
	print_vows_choose_quest, "gm_vows_choose_quest";
	print_vows_details_stub, "gm_vows_details_stub";
	
	// Journey
	print_journey_destinations, "gm_journey_destinations";
	print_journey_choose, "gm_journey_choose";
	print_journey_no_region, "gm_journey_no_region";
	
	// Exploration actions
	print_explore_discover, "gm_explore_discover";
	print_explore_creature, "gm_explore_creature";
	
	// Game over
	print_over_fallen, "gm_over_fallen";
	print_over_broken, "gm_over_broken";
	print_over_journey_ends, "gm_over_journey_ends";
	print_over_statistics, "gm_over_statistics";
	
	// Oracle
	print_oracle_likelihood, "gm_oracle_likelihood";
	
	// General
	print_invalid_choice, "gm_invalid_choice";
	print_press_continue, "gm_press_continue";
	)
}

// Paragraphs with a value (name/number)
print_paragraph! {
	palettes::text_colors::get().normal;
	palettes::text_colors::get().name;
	GmStrings(
	print_char_name, "gm_char_name";
	print_journey_started, "gm_journey_started";
	print_journey_rank, "gm_journey_rank";
	)
}

print_paragraph! {
	palettes::text_colors::get().normal;
	palettes::text_colors::get().number;
	GmStrings(
	print_turn, "gm_turn";
	print_journey_progress, "gm_journey_progress";
	print_over_turns, "gm_over_turns";
	print_over_vows_completed, "gm_over_vows_completed";
	print_over_vows_active, "gm_over_vows_active";
	)
}

// Menus
print_menu! {
	palettes::text_colors::get().menu;
	UI.s["menu_left_bracket"]; UI.s["menu_right_bracket"];
	GmStrings(
	print_exploration_menu, "gm_exploration_prompt",
		[
			"gm_menu_view_character",
			"gm_menu_manage_vows",
			"gm_menu_start_journey",
			"gm_menu_make_camp",
			"gm_menu_explore",
			"gm_menu_gather_info",
			"gm_menu_ask_oracle",
			"gm_menu_quit",
		];
	print_combat_menu, "gm_combat_prompt",
		[
			"gm_combat_strike",
			"gm_combat_clash",
			"gm_combat_escape",
		];
	print_vows_menu, "gm_vows_menu_prompt",
		[
			"gm_vows_menu_swear",
			"gm_vows_menu_view",
			"gm_vows_menu_back",
		];
	print_swear_vow_menu, "gm_vows_choose_quest",
		[
			"gm_vow_temple",
			"gm_vow_protect",
			"gm_vow_artifact",
		];
	print_oracle_menu, "gm_oracle_prompt",
		[
			"gm_oracle_yes_no",
			"gm_oracle_action_theme",
			"gm_oracle_complication",
			"gm_oracle_opportunity",
			"gm_oracle_npc",
			"gm_oracle_location",
			"gm_oracle_pay_price",
			"gm_oracle_back",
		];
	)
}

// Return strings
return_string! {
	GmStrings(
	str_banner_ironsworn, "gm_banner_ironsworn";
	str_banner_char_creation, "gm_banner_character_creation";
	str_banner_char_sheet, "gm_banner_character_sheet";
	str_banner_exploration, "gm_banner_exploration";
	str_banner_combat, "gm_banner_combat";
	str_banner_camp, "gm_banner_camp";
	str_banner_vows, "gm_banner_vows";
	str_banner_journey, "gm_banner_journey";
	str_banner_undertake_journey, "gm_banner_undertake_journey";
	str_banner_explore, "gm_banner_explore";
	str_banner_gather_info, "gm_banner_gather_info";
	str_banner_swear_vow, "gm_banner_swear_vow";
	str_banner_game_over, "gm_banner_game_over";
	str_banner_oracle, "gm_banner_oracle";
	str_roll_strong_hit, "gm_roll_strong_hit";
	str_roll_weak_hit, "gm_roll_weak_hit";
	str_roll_miss, "gm_roll_miss";
	str_roll_match, "gm_roll_match";
	str_oracle_rolling, "gm_oracle_rolling";
	str_oracle_answer, "gm_oracle_answer";
	str_oracle_action, "gm_oracle_action";
	str_oracle_theme, "gm_oracle_theme";
	str_oracle_npc_disposition, "gm_oracle_npc_disposition";
	str_oracle_npc_goal, "gm_oracle_npc_goal";
	str_oracle_feature, "gm_oracle_feature";
	str_oracle_price, "gm_oracle_price";
	)
}
