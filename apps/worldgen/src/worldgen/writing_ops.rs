use crate::worldgen::*;
use io_ops::create_dir;
use std::path::Path;

pub fn write_save(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	preset_name: &str,
) {
	let world_name =
		[preset_name, "_", &lp.wi.seed.to_string()].concat();

	//Make a save directory if none exists
	let save_dir = Path::new(PATH_SAVE).to_path_buf();

	let world_dir =
		Path::new(PATH_SAVE).to_path_buf().join(world_name);

	create_dir(&save_dir);
	create_dir(&world_dir);

	//Write the data
	if options_worldgen.write_data_files {
		println!("{}", wg_str.wg25);
		write_data(lp, wg_str, options_worldgen, &world_dir);
	} else {
		println!("{}", wg_str.wg26);
	}

	//Optionally render colorful images
	if options_worldgen.render_colorized_maps {
		println!("{}", wg_str.wg15);
		write_images_color(lp, wg_str, options_worldgen, &world_dir);
	}

	//Optionally render raw images
	if options_worldgen.render_raw_maps {
		println!("{}", wg_str.wg16);
		write_images_raw(lp, wg_str, options_worldgen, &world_dir);
	}
}
