use lib_constants::app::*;
use lib_game_data_codec as gdc;
use std::path::PathBuf;

pub fn get() -> Vec<gdc::entities::EntityData> {
	// Get paths to .ron files.
	let mut presets: Vec<PathBuf> =
		crate::dir_file_contents_full_paths(PATH_PRESETS_FLORA, EXTENSION_PRESET);

	let mut presets_usr: Vec<PathBuf> = crate::dir_file_contents_full_paths(
		PATH_PRESETS_FLORA_USER,
		EXTENSION_PRESET,
	);

	// Merge paths to presets and feed them to the loop to get data.
	presets.append(&mut presets_usr);
	let mut all_entities_from_presets: Vec<gdc::entities::EntityData> =
		Vec::new();

	for preset_path in presets.iter() {
		let data = crate::single_file_to_string(preset_path.clone());

		let mut entries_from_preset: Vec<Vec<gdc::entities::EntityData>> =
			Vec::new();
		match ron::from_str(&data) {
			Ok(f) => entries_from_preset.push(f),
			Err(e) => {
				println!("ERROR when parsing flora preset: {}, {}", preset_path.as_path().display().to_string(), e.to_string());
				println!("Check missing commas in preset.");
				println!("Check if all option names are valid.");
				println!(
					"Check if all values are within their limits (u8, u16, etc.)."
				);
				println!("Check if structure ends with 'End' element, if the number is set.");
				std::process::exit(0);
			}
		};
		// Drain entries from one preset (can be multiple) into general vec.
		for entry in entries_from_preset.iter() {
			for entity in entry.iter() {
				all_entities_from_presets.push(entity.clone());
			}
		}
	}
	// Return global vector of entries.
	//println!("{:?}", all_entities_from_presets);
	all_entities_from_presets
}
