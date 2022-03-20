use lib_constants::app::*;
use lib_game_data_codec as gdc;
use std::path::Path;

pub fn get(input: &str) -> gdc::WorldgenPreset {
	//Check both default and user paths
	let path_def = Path::new(PATH_PRESETS_WORLD)
		.join(&input)
		.with_extension(EXTENSION_PRESET);

	let path_usr = Path::new(PATH_PRESETS_WORLD_USER)
		.join(&input)
		.with_extension(EXTENSION_PRESET);

	let path_vec = vec![path_def, path_usr];

	let data = crate::file_to_string(&path_vec);

	let stuff: gdc::WorldgenPreset = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}", e.to_string());
			std::process::exit(0);
		}
	};
	stuff
}
