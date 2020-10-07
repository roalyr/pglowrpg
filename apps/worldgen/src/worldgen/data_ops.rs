use crate::worldgen::*;
use bincode;
use io_ops::{compress_to_storage, create_dir};

pub fn write_data(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	_options_worldgen: &options::options_worldgen::Stuff,
	world_dir: &std::path::PathBuf,
) {
	//Make a directory if none exists
	let data_dir = world_dir.join(PATH_SAVE_DATA);
	create_dir(&data_dir);

	let file_path = data_dir
		.join("world_data")
		.with_extension(EXTENSION_SAVE_DATA);

	println!("{}", wg_str.wg27);
	let encoded: Vec<u8> = bincode::serialize(&lp).unwrap();
	compress_to_storage(encoded, &file_path);

	//Just a test and for future use
	//use io_ops::decompress_to_memory;
	//let data_read = decompress_to_memory(&file_path);
	//let data_decoded: LayerPack = bincode::deserialize(&data_read[..]).unwrap();
	//println!("{:?}", data_decoded);
}
