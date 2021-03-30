pub mod data_ops;
pub mod image_color_ops;
pub mod image_raw_ops;

use crate::file_ops::data_ops::write_data;
use crate::file_ops::image_color_ops::write_images_color;
use crate::file_ops::image_raw_ops::write_images_raw;
use str_ops::worldgen_str::LOC;
use crate::*;
use io_ops::create_dir;
use std::path::Path;

pub fn write_save(
	lp: &mut LayerPack,
	options: &options::Stuff,
	preset_name: &str,
) {
	let world_name = [preset_name, "_", &lp.wi.seed.to_string()].concat();

	//Add custom save names, check for existing, etc.

	//Make a save directory if none exists
	let save_dir = Path::new(PATH_SAVE).to_path_buf();
	let world_dir = Path::new(PATH_SAVE).to_path_buf().join(world_name);
	create_dir(&save_dir);
	create_dir(&world_dir);

	//Write the data
	if options.write_data_files {
		LOC.print_write_data();
		write_data(lp, options, &world_dir);
	} else {
		//But still save a copy of preset into a save folder
		LOC.print_write_no_data();
	}
	LOC.print_sep2();

	//Optionally render colorful images
	if options.render_colorized_maps {
		LOC.print_write_color();
		write_images_color(lp, options, &world_dir);
	}
	LOC.print_sep2();

	//Optionally render raw images
	if options.render_raw_maps {
		LOC.print_write_raw();
		write_images_raw(lp, options, &world_dir);
	}
}
