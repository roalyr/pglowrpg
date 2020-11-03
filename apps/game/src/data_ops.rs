use crate::*;

pub fn get_layerpack(
	gm_str: &strings::game_strings::Stuff,
	ui_el: &strings::ui_elements::Stuff,
) -> LayerPack {
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
	bincode::deserialize(&data_read[..]).unwrap()
}
