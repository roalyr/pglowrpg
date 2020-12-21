pub mod array_ops;
pub mod file_ops;
pub mod layer_ops;
pub mod misc_ops;

use crate::file_ops::data_ops::*;
use crate::file_ops::image_color_ops::*;
use crate::file_ops::image_raw_ops::*;
use crate::file_ops::writing_ops::*;
use crate::layer_ops::*;
use crate::misc_ops::preset_validate;
use crate::misc_ops::seed_generating;
use codec::*;
use constants_app::*;
use coords::Index;
use io_ops::toml::{options, presets, strings};
use ui::prompts;

pub fn start(
	options: &options::Stuff,
	wg_str: &strings::worldgen_strings::Stuff,
	panic_str: &strings::panic_strings::Stuff,
	ui_el: &strings::ui_elements::Stuff,
) {
	//Intro message
	println!("{}", &wg_str.wg1);

	//Preset selection
	//List files in default and user dirs
	let preset_def_tuple = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD,
		EXTENSION_PRESET_WORLD,
		&ui_el.bullet1,
		&ui_el.separator1,
	);
	let mut preset_user_tuple = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD_USER,
		EXTENSION_PRESET_WORLD,
		&ui_el.bullet1,
		&ui_el.separator1,
	);

	//Get the contents of the presets folder
	let mut presets_paths = preset_def_tuple.1;
	presets_paths.append(&mut preset_user_tuple.1);
	let presets_formatted =
		[preset_def_tuple.0, preset_user_tuple.0, "\n".to_string()].concat();
	let mut input_preset =
		prompts::new_line_io(&presets_formatted, &ui_el.prompt2);
	input_preset = prompts::autocomplete(&input_preset, &presets_paths);
	if input_preset.is_empty() {
		//Warning about no such preset
		println!("{}", &wg_str.wg28);
		return;
	}
	println!("{}", &ui_el.separator2);
	if input_preset.is_empty() {
		input_preset = options.default_preset.clone();
	}

	let mut wi: presets::presets_worldgen::Stuff =
		presets::presets_worldgen::get(&input_preset);
	//Show selected preset
	prompts::selected(&wg_str.wg3, &input_preset);
	preset_validate::all(&mut wi, &panic_str);

	//Seed selection
	let input_seed = prompts::new_line_io(&wg_str.wg2, &ui_el.prompt2);
	println!("{}", &ui_el.separator2);
	let mut temp_seed = if (input_seed == "r") || (input_seed == "R") {
		println!("{}", wg_str.wg4);
		seed_generating::get()
	} else {
		wi.seed
	};

	//Decide how many worlds to generate
	let input_world_num = prompts::new_line_io(&wg_str.wg24, &ui_el.prompt2);
	println!("{}", &ui_el.separator2);
	let world_num = if input_world_num.is_empty() {
		options.worlds_to_generate
	} else {
		//proper panic str later
		input_world_num
			.trim()
			.parse::<usize>()
			.expect("Expected an integer")
	};
	prompts::selected(&wg_str.wg6, &world_num.to_string());
	println!("{}", &ui_el.separator2);

	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;
	let map_size = wi.map_size;

	for iter in 0..world_num {
		//Increment seed for multiple worlds, must be before wi
		temp_seed += iter;

		//Re-call this every loop iteration
		let mut wi: presets::presets_worldgen::Stuff =
			presets::presets_worldgen::get(&input_preset);
		preset_validate::all(&mut wi, &panic_str);
		wi.seed = temp_seed;

		//Re-call this every loop iteration
		let mut lp = LayerPack {
			xy: Index { map_size },
			wi,
			noisemap_vec_len,
			layer_vec_len,
			biomes: BitLayerBiomes {
				data: vec![0; layer_vec_len],
			},
			topography: BitLayerTopography {
				data: vec![0; layer_vec_len],
				TERRAIN: 0b_0000_0000_1111_1111,
				WATERMASK: 0b_0001_1111_0000_0000,
				_placeholder: 0b_1110_0000_0000_0000,
			},
			climate: BitLayerClimate {
				data: vec![0; layer_vec_len],
				TEMPERATURE: 0b_0000_0000_1111_1111,
				RAINFALL: 0b_1111_1111_0000_0000,
			},
			rivers: BitLayerRivers {
				data: vec![0; layer_vec_len],
				ELEMENT: 0b_0000_0000_0000_0111,
				WIDTH: 0b_0000_0000_0111_1000,
				UPSTREAM: 0b_0000_0111_1000_0000,
				DOWNSTREAM: 0b_0111_1000_0000_0000,
				_placeholder: 0b_1000_0000_0000_0000,
			},
			rivers_id: BitLayerRiversID {
				data: vec![0; layer_vec_len],
			},
			georeg_id: BitLayerGeoregID {
				data: vec![0; layer_vec_len],
			},
		};

		//Show selected seed
		prompts::selected(&wg_str.wg5, &(lp.wi.seed.to_string()));
		//Perform generation
		println!("{}", wg_str.wg7);
		terrain_mapping::get(&mut lp);
		println!("{}", wg_str.wg9);
		climate_mapping::get(&mut lp);
		//Requires temperature
		println!("{}", wg_str.wg13);
		watermask_mapping::get(&mut lp);
		//Requires terrain, watermask, temperature, rainfall
		println!("{}", wg_str.wg17);
		river_mapping::get(&mut lp, &wg_str);
		//Requires the above, must be called after rivers (erosion)
		println!("{}", wg_str.wg19);
		biome_mapping::get(&mut lp);
		//Requires biomes
		println!("{}", wg_str.wg21);
		georegion_mapping::get(&mut lp, &wg_str);

		//WRITING DATA
		write_save(&mut lp, &wg_str, &ui_el, &options, &input_preset);
		println!("{}", &ui_el.separator2);
	}
}
