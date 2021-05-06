pub mod array_ops;
pub mod file_ops;
pub mod layer_ops;
pub mod misc_ops;

use crate::file_ops::write_save;
use crate::misc_ops::preset_validate;
use crate::misc_ops::seed_generating;
use codec::*;
use constants_app::*;
use coords::Index;
use game_options::OPTIONS;
use io_ops::readron::presets;
use text_ops::{prompt_input, selected, UI, WS};

#[rustfmt::skip]
pub fn start() {
	
	UI.print_newline();
	UI.print_banner_dash(WS.str_banner_title());
	UI.print_newline();
	
	//Preset selection
	let preset_def = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD,
		EXTENSION_PRESET_WORLD,
	);
	let preset_user = io_ops::dir_file_contents(
		PATH_PRESETS_WORLD_USER,
		EXTENSION_PRESET_WORLD,
	);
	let presets = [preset_def, preset_user].concat();
	let input_preset = prompt_input!( 
		&presets;
		{
			WS.print_preset_select();
			WS.print_list_preset(&presets);
		}
	);
	//Decide how to treat no input
	if input_preset.is_empty() {
		WS.print_no_input_preset();
		return;
	}
	
	UI.print_separator_thin("");

	//Load a preset
	let mut wi: presets::presets_worldgen::Stuff = presets::presets_worldgen::get(&input_preset);
	selected(&WS.str_sel_preset(), &input_preset);
	preset_validate::all(&mut wi);

	//Seed selection
	let input_seed = prompt_input!( 
		{ WS.print_prompt_seed_rand(); }
	);
	
	UI.print_separator_thin("");
	
	let mut temp_seed = if (input_seed == "r") || (input_seed == "R") {
		WS.print_seed_rand();
		seed_generating::get()
	} else if (input_seed == "p") || (input_seed == "P") {
		wi.seed
	} else {
		input_seed.trim().parse::<usize>().unwrap_or(wi.seed)
	} ;

	//Decide how many worlds to generate
	let input_world_num = prompt_input!( 
		{ WS.print_prompt_world_num(); }
	);
	let world_num = input_world_num.trim().parse::<usize>().unwrap_or(1);
	
	UI.print_separator_thin("");
	WS.print_world_num(world_num);

	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;
	let map_size = wi.map_size;

	for iter in 0..world_num {
		//Increment seed for multiple worlds, must be before wi
		temp_seed += iter;
		
		//Create a "WorldInit" struct that holds all the WS OPTIONS.
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
		UI.print_separator_thick("");
		WS.print_seed_used(lp.wi.seed);
		UI.print_newline();
		
		//Perform generation
		//Keep the order as is, because the data is incrfmentally
		//generated and used for later generation process
		WS.print_prep_topog(); layer_ops::terrain_mapping::get(&mut lp);
		WS.print_prep_climate(); layer_ops::climate_mapping::get(&mut lp);
		WS.print_prep_wmask(); layer_ops::watermask_mapping::get(&mut lp);
		WS.print_prep_rmap(); layer_ops::river_mapping::get(&mut lp);
		WS.print_prep_biome(); layer_ops::biome_mapping::get(&mut lp);
		WS.print_prep_georeg(); layer_ops::georegion_mapping::get(&mut lp);

		//WRITING DATA
		write_save(&mut lp, &input_preset);
		UI.print_separator_thin("");
		WS.print_done_worldgen();
		// Add an empty prompt "continue..."
	}
}
