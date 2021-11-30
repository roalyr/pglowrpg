pub mod data_ops;
pub mod image_color_ops;
pub mod image_raw_ops;

use crate::file_ops::data_ops::write_data;
use crate::file_ops::image_color_ops::write_images_color;
use crate::file_ops::image_raw_ops::write_images_raw;
use crate::LayerPack;
use lib_constants::app as ca;
use lib_game_options::OPTIONS;
use lib_io_ops::create_dir;
use lib_text_ops::WORLDGEN_STRINGS as WS;
use std::path::Path;

pub fn write_save(
	lp: &mut LayerPack,
	preset_name: &str,
) {
	let world_name = [preset_name, "_", &lp.wi.seed.to_string()].concat();

	// Add custom save names, check for existing, etc.

	// Make a save directory if none exists (unless it is a cold run).
	let mut make_dirs = false;

	#[allow(unused_assignments)]
	let mut save_dir = Path::new("").to_path_buf();
	let mut world_dir = Path::new("").to_path_buf();

	if OPTIONS.write_data_files
		|| OPTIONS.render_colorized_maps
		|| OPTIONS.render_raw_maps
	{
		make_dirs = true;
	}

	if make_dirs {
		save_dir = Path::new(ca::PATH_SAVE).to_path_buf();
		world_dir = Path::new(ca::PATH_SAVE).to_path_buf().join(world_name);
		create_dir(&save_dir);
		create_dir(&world_dir);
		// TODO: save a copy of preset into a save folder
	}

	// Write the data
	if OPTIONS.write_data_files {
		WS.print_write_data();
		write_data(lp, &world_dir);
	} else {
		WS.print_write_no_data();
	}

	// Optionally render colorful images
	if OPTIONS.render_colorized_maps {
		WS.print_write_color();
		write_images_color(lp, &world_dir);
	}

	// Optionally render raw images
	if OPTIONS.render_raw_maps {
		WS.print_write_raw();
		write_images_raw(lp, &world_dir);
	}
}
