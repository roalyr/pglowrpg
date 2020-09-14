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

pub struct Layer {
	pub array_map: Vec<u8>,
	pub layer_name: String,
}

pub struct LayerPack<'a> {
	pub xy: Index,
	pub wi: &'a presets::presets_worldgen::Stuff,

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
	options_worldgen: options::options_worldgen::Stuff,
	_options_global: options::options_global::Stuff,
	options_debug: options::options_debug::Stuff,
	wg_str: strings::worldgen_strings::Stuff,
	panic_str: strings::panic_strings::Stuff,
) {
	//Intro message
	println!("{}", &wg_str.wg1);

	//List files in dir
	let p_str = prompt::dir_contents(
		PATH_PRESETS_WORLD,
		EXTENSION_PRESET_WORLD,
		", ",
		&panic_str,
	);

	//Input prompts
	let mut input_preset = prompt::new_line_io(&p_str);
	let input_seed = prompt::new_line_io(&wg_str.wg2);

	//Preset selection
	if input_preset.is_empty() {
		input_preset = options_worldgen.default_preset.clone();
	}

	let mut wi: presets::presets_worldgen::Stuff =
		presets::presets_worldgen::get(&input_preset);

	//Show selected preset
	prompt::selected(&wg_str.wg3, &input_preset);

	//Check the preset values
	preset_validate::all(&mut wi, &panic_str);

	if input_seed == "r" {
		wi.seed = seed_generating::get();
	}

	let w = options_worldgen.worlds_to_generate;
	prompt::selected(&wg_str.wg6, &w.to_string());

	for _ in 0..w {
		prompt::selected(&wg_str.wg5, &wi.seed.to_string());
		run(
			&wi,
			&wg_str,
			&options_worldgen,
			&options_debug,
			&input_preset,
		);
		wi.seed += 1;
	}
}

//GENERATE
fn run(
	wi: &presets::presets_worldgen::Stuff,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	_options_debug: &options::options_debug::Stuff,
	preset_name: &str,
) {
	let layer_vec_len = wi.map_size * wi.map_size;
	let noisemap_vec_len = wi.noisemap_size * wi.noisemap_size;

	//▒▒▒▒▒▒▒▒▒▒▒▒ INIT ▒▒▒▒▒▒▒▒▒▒▒▒
	let mut lp = LayerPack {
		//coordinate system
		xy: Index {
			map_size: wi.map_size,
		},

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

	//▒▒▒▒▒▒▒▒▒▒ GENERATION ▒▒▒▒▒▒▒▒▒▒▒
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
	write_save(&mut lp, wg_str, options_worldgen, &preset_name);
}
