use codec::*;
use constants_app::*;
use coords::Index;
use io_ops::decompress_to_memory;
use io_ops::toml::{options, strings};
use std::path::Path;
use ui::prompts;

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
	let mut input_save = String::new();
	input_save =
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
	let mut input = String::new();
	let map_size = lp.wi.map_size;
	let xy = Index { map_size };
	let mut x = 0;
	let mut y = 0;

	//Main loop
	loop {
		//Coordinates 1D,  2D, height
		let index = xy.ind(x, y);
		let coord_str = [
			"x:",
			&(x.to_string()),
			" y:",
			&(y.to_string()),
			" index:",
			&(index.to_string()),
			" ",
		]
		.concat();

		//Input handling
		input = prompts::new_line_io(&coord_str, &ui_el.prompt2);
		input = prompts::autocomplete(&input, &commands);
		prompts::selected(&gm_str.gm6, &input);
		println!("{}", &ui_el.separator2);

		//Movement directions
		match input.as_str() {
			"west" => {
				x = x.saturating_sub(1);
			}
			"north" => {
				y = y.saturating_add(1);
				if y > map_size {
					y = map_size;
				}
			}
			"east" => {
				x = x.saturating_add(1);
				if x > map_size {
					x = map_size;
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
				x = prompts::new_line_io("", &ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.expect("Expected an integer");
				if x > map_size {
					x = map_size;
				}
			}
			"y" => {
				y = prompts::new_line_io("", &ui_el.prompt2)
					.trim()
					.parse::<usize>()
					.expect("Expected an integer");
				if y > map_size {
					y = map_size;
				}
			}
			&_ => {}
		}

		//Common commands?
		match input.as_str() {
			"q" => return,
			"?" => {
				println!("{}", &gm_str.gm2);
				println!("Registered commands are:\n{:?}", &commands);
			}
			&_ => {}
		}
	}
}
