use crate::*;

pub fn get_layerpack(gs: &GameStrings) -> Option<LayerPack> {
	//Select a world to load
	let save_dir_tuple = io_ops::dir_dir_contents(PATH_SAVE, "", "");

	//Get the contents of save dir
	let save_dir_paths = save_dir_tuple.1;
	let save_dir_formatted = [save_dir_tuple.0, "\n".to_string()].concat();
	//println!("{}", &gs.gm_str.gm3);

	//Read input to pick a specific save

	//TODO
	println!("{}", &save_dir_formatted);

	let input_save = prompt_input!(&save_dir_paths, {});

	if !input_save.is_empty() {
		//println!("{}", &gs.ui_el.separator2);
		//Show selected world
		//prompts::selected(&gs.gm_str.gm4, &input_save);
		//println!("{}", &gs.ui_el.separator2);
		let save_data = Path::new(PATH_SAVE)
			.to_path_buf()
			.join(input_save)
			.join(PATH_SAVE_DATA)
			.join(NAME_DATA_WORLD)
			.with_extension(EXTENSION_SAVE_DATA);
		let data_read = decompress_to_memory(&save_data);
		Some(bincode::deserialize(&data_read[..]).unwrap())
	} else {
		//Warning about no such folder
		//println!("{}", &gs.gm_str.gm5);
		None
	}
}

pub fn get_world_current(gd: &mut GameData) {
	//Coordinates 1D,  2D, height
	//+x is north, +y is east due to how worldgen was made
	gd.index = gd.lp.xy.ind(gd.x, gd.y);
	//Into data ops
	gd.temp = translate::get_abs(
		gd.lp.climate.read(gd.lp.climate.TEMPERATURE, gd.index) as f32,
		255.0,
		gd.lp.wi.abs_temp_min as f32,
		gd.lp.wi.abs_temp_max as f32,
	) as isize;
	gd.rain = translate::get_abs(
		gd.lp.climate.read(gd.lp.climate.RAINFALL, gd.index) as f32,
		255.0,
		gd.lp.wi.abs_rain_min as f32,
		gd.lp.wi.abs_rain_max as f32,
	) as usize;
	gd.elev = translate::get_abs(
		gd.lp.topography.read(gd.lp.topography.TERRAIN, gd.index) as f32,
		255.0,
		gd.lp.wi.abs_elev_min as f32,
		gd.lp.wi.abs_elev_max as f32,
	) as usize;

	gd.water = gd.lp.topography.read(gd.lp.topography.WATERMASK, gd.index);
	gd.biome = gd.lp.biomes.read(gd.index);
	gd.georeg_id = gd.lp.georeg_id.read(gd.index);
	gd.river_id = gd.lp.rivers_id.read(gd.index);
	gd.river_width = gd.lp.rivers.read(gd.lp.rivers.WIDTH, gd.index);
	gd.river_element = gd.lp.rivers.read(gd.lp.rivers.ELEMENT, gd.index);
	gd.river_upstream = gd.lp.rivers.read(gd.lp.rivers.UPSTREAM, gd.index);
	gd.river_downstream = gd.lp.rivers.read(gd.lp.rivers.DOWNSTREAM, gd.index);
}
