pub mod array_ops;
pub mod file_ops;
pub mod layer_ops;
pub mod misc_ops;

use crate::file_ops::write_save;
use crate::misc_ops::convert_preset_values;
use crate::misc_ops::seed_generating;
use constants::app as ca;
use constants::world as cw;
use game_data_codec::*;
use game_options::OPTIONS;
use io_ops::readron::presets;
use text_ops::{prompt_input, UI, WS};
use unit_systems::coords::Index;

#[rustfmt::skip]
pub fn start() {
	// Intro block.
	UI.print_banner_dash(WS.str_banner_title());
	UI.print_newline();
	WS.print_intro();
	
	// Preset selection.
	let preset_def = io_ops::dir_file_contents(
		ca::PATH_PRESETS_WORLD,
		ca::EXTENSION_PRESET_WORLD,
	);
	let preset_user = io_ops::dir_file_contents(
		ca::PATH_PRESETS_WORLD_USER,
		ca::EXTENSION_PRESET_WORLD,
	);
	let presets = [preset_def, preset_user].concat();
	let input_preset = prompt_input!( 
		"word"; &presets;
		{
			UI.print_separator_thin("");
			WS.print_list_preset(&presets);
		}
	);
	// Decide how to treat no input.
	if input_preset.is_empty() {
		WS.print_no_input_preset();
		return;
	}
	let mut wi: presets::presets_worldgen::Stuff = presets::presets_worldgen::get(&input_preset);
	WS.print_preset_selected(&input_preset);
	// Converts from user-friendly scales to mechanics-friendly.
	// Also checks validity.
	convert_preset_values::all(&mut wi);

	// Seed selection.
	let input_seed = prompt_input!(
		"opt";
		{
			UI.print_separator_thin(""); 
			WS.print_seed_default(cw::DEFAULT_SEED); 
			WS.print_seed_menu(); 
		} 
	);
	let mut world_seed = match input_seed.as_str(){
		"1" => {
			let seed_man = prompt_input!( "num"; { WS.print_seed_man(); });
			seed_man.trim().parse::<u32>().unwrap_or(cw::DEFAULT_SEED)
		},
		"2" => {
			WS.print_seed_rand();
			seed_generating::get()
		},
		"3" => {
			WS.print_seed_pres();
			wi.seed
		},
		_ => {cw::DEFAULT_SEED}
	};
	WS.print_seed_base(world_seed);
	
	// Decide how many worlds to generate.
	let input_world_num = prompt_input!( 
		"num";
		{
			UI.print_separator_thin(""); 
			WS.print_prompt_world_num();
			WS.print_world_num_default(cw::DEFAULT_WORLDS_NUM);
		}
	);
	let world_num = input_world_num.trim().parse::<usize>().unwrap_or(1);
	UI.print_separator_thin("");
	WS.print_world_num(world_num);
	
	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;
	let map_size = wi.map_size;

	for n in 0..world_num {
		// Text block.
		UI.print_separator_thick("");
		WS.print_making_world(n+1);
		WS.print_seed_used(world_seed);
		UI.print_newline();
		// After this will be the print-outs from the layer generation
		// modules, as well as other progress reports.
		
		// Create a "WorldInit" struct that holds all the preset data.
		// Re-call this every loop iteration due to new seed.
		let mut wi: presets::presets_worldgen::Stuff = 
			presets::presets_worldgen::get(&input_preset);
		convert_preset_values::all(&mut wi);
		wi.seed = world_seed;
		
		// Create a "LayerPack" struct which holds all the world data.
		// Re-call this every loop iteration due to new seed.
		let mut lp = LayerPack {
			index: Index { map_size },
			wi,
			noisemap_vec_len,
			layer_vec_len,
			
			// Defining the "flags" for each layer
			// here insures there will be no mistake
			// in using wrong offsets for them later on.
			biomes: BitLayerBiomes {
				data: vec![0; layer_vec_len as usize],
			},
			topography: BitLayerTopography {
				data: vec![0; layer_vec_len as usize],
				TERRAIN: 0b_0000_0000_1111_1111,
				WATERMASK: 0b_0001_1111_0000_0000,
				_placeholder: 0b_1110_0000_0000_0000,
			},
			climate: BitLayerClimate {
				data: vec![0; layer_vec_len as usize],
				TEMPERATURE: 0b_0000_0000_1111_1111,
				RAINFALL: 0b_1111_1111_0000_0000,
			},
			rivers: BitLayerRivers {
				data: vec![0; layer_vec_len as usize],
				ELEMENT: 0b_0000_0000_0000_0111,
				WIDTH: 0b_0000_0000_0111_1000,
				UPSTREAM: 0b_0000_0111_1000_0000,
				DOWNSTREAM: 0b_0111_1000_0000_0000,
				_placeholder: 0b_1000_0000_0000_0000,
			},
			rivers_id: BitLayerRiversID {
				data: vec![0; layer_vec_len as usize],
			},
			georeg_id: BitLayerGeoregID {
				data: vec![0; layer_vec_len as usize],
			},
		};
		
		// Perform generation.
		// Keep the order as is, because the data is incrfmentally
		// generated and used for later generation process.
		WS.print_prep_topog(); layer_ops::terrain_mapping::get(&mut lp);
		WS.print_prep_climate(); layer_ops::climate_mapping::get(&mut lp);
		WS.print_prep_wmask(); layer_ops::watermask_mapping::get(&mut lp);
		WS.print_prep_rmap(); layer_ops::river_mapping::get(&mut lp);
		WS.print_prep_biome(); layer_ops::biome_mapping::get(&mut lp);
		WS.print_prep_georeg(); layer_ops::georegion_mapping::get(&mut lp);

		// Write data.
		UI.print_newline();
		write_save(&mut lp, &input_preset);
		UI.print_newline();
		WS.print_done_worldgen();
		
		// Increment seed for multiple worlds
		world_seed += 1;
	}
// TODO: Add an empty prompt "continue..."
}
