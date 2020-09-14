use crate::worldgen::*;
use io_ops::create_dir;
use std::path::Path;

pub fn write_data(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	save_dir: &std::path::PathBuf,
) {
	//Make a directory if none exists
	let data_dir = save_dir.join(PATH_SAVE_DATA);
	create_dir(&data_dir);

	//Code will be further
}
