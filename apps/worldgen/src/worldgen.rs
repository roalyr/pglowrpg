pub mod data_ops;
pub mod image_color_ops;
pub mod image_raw_ops;
pub mod writing_ops;

use data_ops::*;
use image_color_ops::*;
use image_raw_ops::*;
use writing_ops::*;

use crate::layers::*;
use crate::preset_validate;
use crate::seed_generating;

use codec::*;
use constants_app::*;
use coords::Index;
use io_ops::toml::{options, presets, strings};
use ui::prompt;

use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug)]
pub struct LayerPack {
	pub xy: Index,
	pub wi: presets::presets_worldgen::Stuff,

	pub layer_vec_len: usize,
	pub noisemap_vec_len: usize,

	pub biomes: BitLayerBiomes,
	pub rivers_id: BitLayerRiversID,
	pub georeg_id: BitLayerGeoregID,
	pub topography: BitLayerTopography,
	pub climate: BitLayerClimate,
	pub rivers: BitLayerRivers,
}

pub fn start(
	options: &options::Stuff,
	wg_str: &strings::worldgen_strings::Stuff,
	panic_str: &strings::panic_strings::Stuff,
	ui_el: &strings::ui_elements::Stuff,
) {
	//Intro message
	println!("{}", &wg_str.wg1);
	println!("{}", &ui_el.separator2);

	//Preset selection
	//List files in default and user dirs
	let p_str_def = io_ops::dir_contents(
		PATH_PRESETS_WORLD,
		EXTENSION_PRESET_WORLD,
		&ui_el.bullet1,
		&ui_el.separator1,
	);

	let p_str_usr = io_ops::dir_contents(
		PATH_PRESETS_WORLD_USER,
		EXTENSION_PRESET_WORLD,
		&ui_el.bullet1,
		&ui_el.separator1,
	);

	let p_str = [p_str_def, p_str_usr].concat();

	let mut input_preset = prompt::new_line_io(&p_str, &ui_el);
	println!("{}", &ui_el.separator2);

	if input_preset.is_empty() {
		input_preset = options.default_preset.clone();
	}

	let mut wi: presets::presets_worldgen::Stuff =
		presets::presets_worldgen::get(&input_preset);

	//Show selected preset
	prompt::selected(&wg_str.wg3, &input_preset);

	//Check the preset values
	preset_validate::all(&mut wi, &panic_str);

	//Seed selection
	let input_seed = prompt::new_line_io(&wg_str.wg2, &ui_el);
	println!("{}", &ui_el.separator2);

	if (input_seed == "r") || (input_seed == "R") {
		wi.seed = seed_generating::get();
		println!("{}", wg_str.wg4);
	}

	//Decide how many worlds to generate
	let input_world_num = prompt::new_line_io(&wg_str.wg24, &ui_el);
	println!("{}", &ui_el.separator2);

	#[allow(unused_assignments)]
	let mut world_num = 0;

	if input_world_num.is_empty() {
		world_num = options.worlds_to_generate;
	} else {
		//proper panic str later
		world_num = input_world_num
			.trim()
			.parse::<usize>()
			.expect("Expected an integer");
	}
	prompt::selected(&wg_str.wg6, &world_num.to_string());
	println!("{}", &ui_el.separator2);

	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;
	let map_size = wi.map_size;

	for iter in 0..world_num {
		//Re-call this every loop iteration
		let mut wi: presets::presets_worldgen::Stuff =
			presets::presets_worldgen::get(&input_preset);
			
		preset_validate::all(&mut wi, &panic_str);
		
		//Re-call this every loop iteration
		let mut lp = LayerPack {
			//coordinate system
			xy: Index { map_size },

			//header stuff
			wi,
			noisemap_vec_len,
			layer_vec_len,

			//layers
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
				UPSTREAM: 0b_0000_0011_1000_0000,
				DOWNSTREAM: 0b_0001_1100_0000_0000,
				_placeholder: 0b_1110_0000_0000_0000,
			},

			rivers_id: BitLayerRiversID {
				data: vec![0; layer_vec_len],
			},

			georeg_id: BitLayerGeoregID {
				data: vec![0; layer_vec_len],
			},
		};

		//Increment seed for multiple worlds
		lp.wi.seed += iter;

		//Show selected seed
		prompt::selected(&wg_str.wg5, &(lp.wi.seed.to_string()));

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
