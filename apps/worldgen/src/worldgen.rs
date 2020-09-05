//pub mod writing_ops;

use crate::array_ops::translate;
//use crate::layers::*;
use crate::preset_validate;
use crate::seed_generating;

use codec::*;
use coords::Index;
use io::prompt;
use io::toml::{options, presets, strings};
use io::writepng::*;
//use writing_ops::*;

use deepsize::DeepSizeOf;

#[derive(DeepSizeOf)]
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

pub fn start() {
	//LOAD OPTIONS
	let options_worldgen: options::options_worldgen::Stuff =
		options::options_worldgen::get();

	let options_global: options::options_global::Stuff =
		options::options_global::get();

	let options_debug: options::options_debug::Stuff =
		options::options_debug::get();

	//LOAD UI STRINGS
	let input_locale = options_global.locale;
	let wg_str: strings::worldgen_strings::Stuff =
		strings::worldgen_strings::get(&input_locale);

	let panic_str: strings::panic_strings::Stuff =
		strings::panic_strings::get(&input_locale);

	//UI
	let mut input_preset = prompt::new_line_io(&wg_str.wg1);
	let input_seed = prompt::new_line_io(&wg_str.wg2);

	//PRESET
	if input_preset.is_empty() {
		input_preset = options_worldgen.default_preset.clone();
	}

	let mut wi: presets::presets_worldgen::Stuff =
		presets::presets_worldgen::get(&input_preset);
	prompt::selected(&wg_str.wg3, &input_preset);

	//CHECK AND SEED
	preset_validate::all(&mut wi, &panic_str);

	if input_seed == "r" {
		wi.seed = seed_generating::get();
	}

	let w = options_worldgen.worlds_to_generate;
	prompt::selected(&wg_str.wg6, &w.to_string());

	for _ in 0..w {
		prompt::selected(&wg_str.wg5, &wi.seed.to_string());
		run(&wi, &wg_str, &options_worldgen, &options_debug);
		wi.seed += 1;
	}
}

//GENERATE
fn run(
	wi: &presets::presets_worldgen::Stuff,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	options_debug: &options::options_debug::Stuff,
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
			masks: MasksTopography {
				terrain: 0b_0000_0000_1111_1111,
				watermask: 0b_0001_1111_0000_0000,
				_placeholder: 0b_1110_0000_0000_0000,
			},
		},

		climate: BitLayerClimate {
			data: vec![0; layer_vec_len],
			masks: MasksClimate {
				temperature: 0b_0000_0000_1111_1111,
				rainfall: 0b_1111_1111_0000_0000,
			},
		},

		rivers: BitLayerRivers {
			data: vec![0; layer_vec_len],
			masks: MasksRivers {
				element: 0b_0000_0000_0000_0111,
				width: 0b_0000_0000_0111_1000,
				upstream: 0b_0000_0011_1000_0000,
				downstream: 0b_0001_1100_0000_0000,
				_placeholder: 0b_1110_0000_0000_0000,
			},
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
	//terrain_mapping::get(&mut lp);

	println!("{}", wg_str.wg9);
	//climate_mapping::get(&mut lp);

	//need temperature
	println!("{}", wg_str.wg13);
	//watermask_mapping::get(&mut lp);

	//needs terrain, watermask, temperature, rainfall
	println!("{}", wg_str.wg17);
	//river_mapping::get(&mut lp, &wg_str);

	//needs the above, must be called after rivers (erosion)
	println!("{}", wg_str.wg19);
	//biome_mapping::get(&mut lp);

	//needs biomes
	println!("{}", wg_str.wg21);
	//georegion_mapping::get(&mut lp, &wg_str);

	//WRITING DATA
	//["topog_", &{ lp.wi.seed.to_string() }].concat();
	//get_data_size(&lp);
	//write_images(&mut lp, wg_str,
	//options_worldgen, options_debug);
	//write raws
	//write data files
}
