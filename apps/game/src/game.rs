use codec::*;
use constants_app::*;
use coords::Index;
use io_ops::decompress_to_memory;
use io_ops::toml::{options, strings};
use std::path::Path;
use ui::prompts;
use units::translate;

pub fn start(
	_options: &options::Stuff,
	gm_str: &strings::game_strings::Stuff,
	_panic_str: &strings::panic_strings::Stuff,
	ui_el: &strings::ui_elements::Stuff,
) {
	//Banner
	println!("{}", &gm_str.gm1);
	//Intro message
	println!("{}", &gm_str.gm2);

	//Select a world to load
	let save_dir_tuple = io_ops::dir_dir_contents(
		PATH_SAVE,
		&ui_el.bullet1,
		&ui_el.separator1,
	);

	//Get the contents of save dir
	let save_dir_paths = save_dir_tuple.1;
	let save_dir_formatted =
		[save_dir_tuple.0, "\n".to_string()].concat();

	println!("{}", &gm_str.gm3);

	//Read input to pick a specific save
	let mut input_save =
		prompts::new_line_io(&save_dir_formatted, &ui_el.prompt2);

	input_save = prompts::autocomplete(&input_save, &save_dir_paths);

	if input_save.is_empty() {
		//Warning about no such folder
		println!("{}", &gm_str.gm5);
		return;
	}

	println!("{}", &ui_el.separator2);

	//Show selected world
	prompts::selected(&gm_str.gm4, &input_save);
	println!("{}", &ui_el.separator2);

	let save_data = Path::new(PATH_SAVE)
		.to_path_buf()
		.join(input_save)
		.join(PATH_SAVE_DATA)
		.join(NAME_DATA_WORLD)
		.with_extension(EXTENSION_SAVE_DATA);

	let data_read = decompress_to_memory(&save_data);
	let lp: LayerPack = bincode::deserialize(&data_read[..]).unwrap();

	//For predictive input, can be moved somewhere else later
	//All commands must be registered here in ordet to be able to
	//match to them
	let commands = [
		//Movement directions
		"north".to_string(),
		"east".to_string(),
		"south".to_string(),
		"west".to_string(),
		//teleport
		"x".to_string(),
		"y".to_string(),
		//Common actions
		"?".to_string(),
		"q".to_string(),
	]
	.to_vec();

	//Main loop init
	let map_size = lp.wi.map_size;
	let yx = Index { map_size };
	let mut x = 0;
	let mut y = 0;

	//A temporary string
	#[allow(unused_assignments)]
	let mut s = String::new();

	//Main loop
	loop {
		//Coordinates 1D,  2D, height
		//To match the +x: right, +y:up, it should be like this.
		let index = yx.ind(y, x);

		//Read data
		let temp = translate::get_abs(
			lp.climate.read(lp.climate.TEMPERATURE, index) as f32,
			255.0,
			lp.wi.abs_temp_min as f32,
			lp.wi.abs_temp_max as f32,
		) as isize;

		let rain = translate::get_abs(
			lp.climate.read(lp.climate.RAINFALL, index) as f32,
			255.0,
			lp.wi.abs_rain_min as f32,
			lp.wi.abs_rain_max as f32,
		) as usize;

		let elev = translate::get_abs(
			lp.topography.read(lp.topography.TERRAIN, index) as f32,
			255.0,
			lp.wi.abs_elev_min as f32,
			lp.wi.abs_elev_max as f32,
		) as usize;

		let water =
			lp.topography.read(lp.topography.WATERMASK, index);

		let biome = lp.biomes.read(index);
		let georeg_id = lp.georeg_id.read(index);

		let river_id = lp.rivers_id.read(index);
		let river_width = lp.rivers.read(lp.rivers.WIDTH, index);
		let river_element = lp.rivers.read(lp.rivers.ELEMENT, index);
		let river_upstream =
			lp.rivers.read(lp.rivers.UPSTREAM, index);
		let river_downstream =
			lp.rivers.read(lp.rivers.DOWNSTREAM, index);

		//Format strings
		let coord_str = [
			&gm_str.gm6,
			"x:",
			&(x.to_string()),
			" y:",
			&(y.to_string()),
			" index:",
			&(index.to_string()),
			" ",
		]
		.concat();

		let temp_str =
			[&gm_str.gm8, &(temp.to_string()), " ℃"].concat();

		let biome_str =
			[&gm_str.gm16, &(biome.to_string()), ""].concat();

		let georeg_id_str =
			[&gm_str.gm17, &(georeg_id.to_string()), ""].concat();

		let rain_str =
			[&gm_str.gm9, &(rain.to_string()), " mm"].concat();

		let elev_str = [
			&gm_str.gm10,
			{
				//Must be less or equal
				if elev <= lp.wi.waterlevel {
					s = [&(elev.to_string()), " m ", &gm_str.gm14]
						.concat();
					&s
				} else {
					s = [&(elev.to_string()), " m ", &gm_str.gm15]
						.concat();
					&s
				}
			},
			"",
		]
		.concat();

		let water_str = [
			&gm_str.gm11,
			{
				match water {
					0 => &gm_str.gm12,
					_ => {
						s = [&gm_str.gm13, &(water.to_string()), ""]
							.concat();
						&s
					}
				}
			},
			"",
		]
		.concat();

		let river_str = [
			&gm_str.gm18,
			{
				match river_id {
					0 => &gm_str.gm19,
					_ => {
						s = [
							//id
							&gm_str.gm20,
							&(river_id.to_string()),
							"\n",
							//width
							&gm_str.gm21,
							&(river_width.to_string()),
							"\n",
							//element
							&gm_str.gm22,
							&(river_element.to_string()),
							"\n",
							//upstream
							&gm_str.gm23,
							&(river_upstream.to_string()),
							"\n",
							//downstream
							&gm_str.gm24,
							&(river_downstream.to_string()),
							"",
						]
						.concat();
						&s
					}
				}
			},
			"",
		]
		.concat();

		//Output handling
		println!("{}", coord_str);
		println!("{}", elev_str);
		println!("{}", water_str);
		println!("{}", temp_str);
		println!("{}", rain_str);
		println!("{}", biome_str);
		println!("{}", georeg_id_str);
		println!("{}", river_str);

		//Input handling
		let mut input = prompts::new_line_io("", &ui_el.prompt2);
		input = prompts::autocomplete(&input, &commands);
		println!("{}", &ui_el.separator2);

		//Movement directions
		match input.as_str() {
			"west" => {
				x = x.saturating_sub(1);
			}
			"north" => {
				y = y.saturating_add(1);
				if y >= map_size {
					y = map_size - 1;
				}
			}
			"east" => {
				x = x.saturating_add(1);
				if x >= map_size {
					x = map_size - 1;
				}
			}
			"south" => {
				y = y.saturating_sub(1);
			}
			&_ => {}
		}

		//Teleport
		match input.as_str() {
			"x" => {
				x = prompts::new_line_io(&gm_str.gm7, &ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.unwrap_or(x);
				if x >= map_size {
					x = map_size - 1;
				}
			}
			"y" => {
				y = prompts::new_line_io(&gm_str.gm7, &ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.unwrap_or(y);
				if y >= map_size {
					y = map_size - 1;
				}
			}
			&_ => {}
		}

		//Common commands?
		match input.as_str() {
			"q" => return,
			"?" => {
				println!("{}", &gm_str.gm2);
				//Make this better
				println!("Registered commands are:\n{:?}", &commands);
				println!("{}", &ui_el.separator2);
			}
			&_ => {}
		}
	}
}
