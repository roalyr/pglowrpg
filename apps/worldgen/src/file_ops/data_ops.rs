use crate::LayerPack;
use constants::app as ca;
use io_ops::{compress_to_storage, create_dir};
use std::path::Path;

pub fn write_data(
	lp: &mut LayerPack,
	world_dir: &Path,
) {
	//Make a directory if none exists
	let data_dir = world_dir.join(ca::PATH_SAVE_DATA);
	create_dir(&data_dir);

	let file_path = data_dir
		.join(ca::NAME_DATA_WORLD)
		.with_extension(ca::EXTENSION_SAVE_DATA);

	//println!("{}", wg_str.wg27);
	let encoded: Vec<u8> = bincode::serialize(&lp).unwrap();
	compress_to_storage(encoded, &file_path);
}
