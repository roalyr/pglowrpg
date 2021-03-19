pub mod array_ops;
pub mod file_ops;
pub mod layer_ops;
pub mod misc_ops;
pub mod print_ops;

use crate::file_ops::write_save;
use crate::misc_ops::preset_validate;
use crate::misc_ops::seed_generating;
use codec::*;
use constants_app::*;
use coords::Index;
use io_ops::toml::{options, presets};
use print_ops::locale_load;
use ui::prompts;

#[rustfmt::skip]
pub fn start() {
	//Load options and locale/style presets
	let options: options::Stuff = options::get();
	let l = locale_load();
	
	l.print_intro();
	
	//Preset selection
	//List files in both default and user dirs
	let preset_def_tuple = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD,
		EXTENSION_PRESET_WORLD,
		l.ui_bul1(), l.ui_newline(),
	);
	let mut preset_user_tuple = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD_USER,
		EXTENSION_PRESET_WORLD,
		l.ui_bul1(), l.ui_newline(),
	);
	
	//Get the contents of the presets folder
	let mut presets_paths = preset_def_tuple.1;
	presets_paths.append(&mut preset_user_tuple.1);
	let presets_formatted = [preset_def_tuple.0, preset_user_tuple.0, "\n".to_string()].concat();
	let mut input_preset = prompts::new_line_io(&presets_formatted, l.ui_ps2());
	input_preset = prompts::autocomplete(&input_preset, &presets_paths);
	
	//Decide how to treat no input
	if input_preset.is_empty() {
		l.print_no_input_preset();
	return;}
	//enable this later
	//if input_preset.is_empty() {input_preset = options.default_preset.clone();}
	l.print_sep2();

	//Load a preset
	let mut wi: presets::presets_worldgen::Stuff = presets::presets_worldgen::get(&input_preset);
	prompts::selected(l.str_sel_preset(), &input_preset);
	preset_validate::all(&mut wi);

	//Seed selection
	let input_seed = prompts::new_line_io(l.str_seed_rand(), l.ui_ps2());
	l.print_sep2();
	let mut temp_seed = if (input_seed == "r") || (input_seed == "R") {
		l.print_seed_rand();
		seed_generating::get()
	} else {
		wi.seed
	};

	//Decide how many worlds to generate
	let input_world_num = prompts::new_line_io(l.str_world_num(), l.ui_ps2());
	let world_num = if input_world_num.is_empty() {
		options.worlds_to_generate
	} else {
		//proper panic str later (uwrap_or)
		input_world_num.trim().parse::<usize>().expect("Expected an integer")
	};
	l.print_sep2();
	l.print_world_num(&world_num);

	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;
	let map_size = wi.map_size;

	for iter in 0..world_num {
		//Increment seed for multiple worlds, must be before wi
		temp_seed += iter;
		
		//Create a "WorldInit" struct that holds all the WG options.
		//Re-call this every loop iteration due to new seed
		let mut wi: presets::presets_worldgen::Stuff = presets::presets_worldgen::get(&input_preset);
		preset_validate::all(&mut wi);
		wi.seed = temp_seed;
		
		//Create a "LayerPack" struct which holds all the world data.
		//Re-call this every loop iteration due to new seed
		let mut lp = LayerPack {
			xy: Index { map_size },
			wi,
			noisemap_vec_len,
			layer_vec_len,
			
			//Defining the "flags" for each layer
			//here insures there will be no mistake
			//in using wrong offsets for them later on
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
		l.print_sep1();
		l.print_seed_used(&lp.wi.seed);
		l.print_newline();
		
		//Perform generation
		//Keep the order as is, because the data is incrfmentally
		//generated and used for later generation process
		l.print_prep_topog(); layer_ops::terrain_mapping::get(&mut lp);
		l.print_prep_climate(); layer_ops::climate_mapping::get(&mut lp);
		l.print_prep_wmask(); layer_ops::watermask_mapping::get(&mut lp);
		l.print_prep_rmap(); layer_ops::river_mapping::get(&mut lp);
		l.print_prep_biome(); layer_ops::biome_mapping::get(&mut lp);
		l.print_prep_georeg(); layer_ops::georegion_mapping::get(&mut lp);

		//WRITING DATA
		write_save(&mut lp, &options, &input_preset);
		l.print_sep2();
		l.print_done_worldgen();
	}
}
