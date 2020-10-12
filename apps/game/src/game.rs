use codec::*;
use constants_app::*;
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

	let save_dir_formatted = save_dir_tuple.0;
	let save_dir_paths = save_dir_tuple.1;

	println!("{}", &gm_str.gm3);
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

	let save_data = Path::new(PATH_SAVE)
		.to_path_buf()
		.join(input_save)
		.join(PATH_SAVE_DATA)
		.join(NAME_DATA_WORLD)
		.with_extension(EXTENSION_SAVE_DATA);

	let data_read = decompress_to_memory(&save_data);
	let _data_decoded: LayerPack =
		bincode::deserialize(&data_read[..]).unwrap();

	//main loop
	loop {
		let input = prompts::new_line_io("", &ui_el.prompt1);

		if input.is_empty() {
			continue;
		}

		if (input == "q") || (input == "Q") {
			return;
		}

		if input == "?" {
			//Intro message
			println!("{}", &gm_str.gm2);
		}
	}
}
