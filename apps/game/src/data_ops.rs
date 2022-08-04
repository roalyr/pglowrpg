use crate::struct_ops::{GameData, GameStrings, WorldData};

use dep::bincode;
use lib_constants::app as ca;
use lib_constants::generic as cg;
use lib_game_data_codec::LayerPack;
use lib_game_options::OPTIONS;
use lib_io_ops::decompress_to_memory;
use lib_text_ops::prompt_input;
use lib_unit_systems::translate;
use std::path::Path;

pub fn get_layerpack(_gs: &GameStrings) -> Option<LayerPack> {
	//Select a world to load
	let save_dir = lib_io_ops::dir_dir_contents(ca::PATH_SAVE);
	let input_save =
		prompt_input!("word"; &save_dir; {println!("{:?}", save_dir);});

	if !input_save.is_empty() {
		//println!("{}", &gs.ui_el.separator2);
		//Show selected world
		//prompts::selected(&gs.gm_str.gm4, &input_save);
		//println!("{}", &gs.ui_el.separator2);
		let path = Path::new(ca::PATH_SAVE)
			.to_path_buf()
			.join(input_save)
			.join(ca::PATH_SAVE_DATA)
			.join(ca::NAME_DATA_WORLD)
			.with_extension(ca::EXTENSION_SAVE_DATA);
		let data_read = decompress_to_memory(&path);
		let lp: LayerPack = match bincode::deserialize(&data_read[..]) {
			Ok(lp) => lp,
			Err(e) => {
				println!(
					"ERROR: unable to read game data save file: {}, {}",
					e.to_string(),
					path.to_str().unwrap_or("")
				);
				panic!();
			}
		};
		Some(lp)
	} else {
		//Warning about no such folder
		//println!("{}", &gs.gm_str.gm5);
		None
	}
}

// Probes and returns world data at x, y.
pub fn get_world_data_at(
	gd: &mut GameData,
	x: u32,
	y: u32,
) -> WorldData {
	//Swapping x and y, in reverse to worldgen?
	let index = gd.lp.index.get(x, y);
	let temp = translate::get_abs(
		gd.lp.climate.read(gd.lp.climate.TEMPERATURE, index) as f32,
		cg::VAL_255_F32,
		gd.lp.wi.abs_temp_min as f32,
		gd.lp.wi.abs_temp_max as f32,
	) as i32;
	let rain = translate::get_abs(
		gd.lp.climate.read(gd.lp.climate.RAINFALL, index) as f32,
		cg::VAL_255_F32,
		gd.lp.wi.abs_rain_min as f32,
		gd.lp.wi.abs_rain_max as f32,
	) as u32;
	let elev = translate::get_abs(
		gd.lp.topography.read(gd.lp.topography.TERRAIN, index) as f32,
		cg::VAL_255_F32,
		gd.lp.wi.abs_elev_min as f32,
		gd.lp.wi.abs_elev_max as f32,
	) as u32;

	WorldData {
		x,
		y,
		index,
		temp,
		rain,
		elev,
		water: gd.lp.topography.read(gd.lp.topography.WATERMASK, index),
		biome: gd.lp.biomes.read(index),
		bioreg_id: gd.lp.bioreg_id.read(index),
		georeg_id: gd.lp.georeg_id.read(index),
		river_id: gd.lp.rivers_id.read(index),
		river_width: gd.lp.rivers.read(gd.lp.rivers.WIDTH, index),
		river_element: gd.lp.rivers.read(gd.lp.rivers.ELEMENT, index),
		river_upstream: gd.lp.rivers.read(gd.lp.rivers.UPSTREAM, index),
		river_downstream: gd.lp.rivers.read(gd.lp.rivers.DOWNSTREAM, index),
		flora: gd
			.lp
			.flora
			.data
			.get(&(index as u32))
			.unwrap_or(&Vec::new())
			.clone(),
	}
}
