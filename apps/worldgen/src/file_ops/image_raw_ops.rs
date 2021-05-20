use crate::LayerPack;
use constants::app as ca;
use constants::generic as cg;
use game_options::OPTIONS;
use io_ops::create_dir;
use io_ops::writepng::{simple_png, GradMode};

pub fn write_images_raw(
	lp: &mut LayerPack,
	world_dir: &std::path::PathBuf,
) {
	//Make a directory if none exists
	let raw_img_dir = world_dir.join(ca::PATH_SAVE_IMAGES_RAW);
	create_dir(&raw_img_dir);

	//Clean proxy maps for rendering images
	let map_size = lp.wi.map_size;
	let mut array_bg = vec![cg::ZERO_U8; lp.layer_vec_len];

	//Watermask and terraim
	if OPTIONS.render_topography {
		//println!("{}", wg_str.wg14);
		let file_path = raw_img_dir
			.join("terrain")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.TERRAIN, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);

		let file_path = raw_img_dir
			.join("watermask")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);
	}

	//Temperature
	if OPTIONS.render_temperature {
		//println!("{}", wg_str.wg10);
		let file_path = raw_img_dir
			.join("temperature")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.climate.read(lp.climate.TEMPERATURE, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);
	}

	//Rainfall
	if OPTIONS.render_rainfall {
		//println!("{}", wg_str.wg12);
		let file_path = raw_img_dir
			.join("rainfall")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.climate.read(lp.climate.RAINFALL, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);
	}

	//Rivers
	if OPTIONS.render_rivers {
		//println!("{}", wg_str.wg18);
		let file_path = raw_img_dir
			.join("rivers_elements")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.rivers.read(lp.rivers.ELEMENT, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);

		//Rivers widths
		let file_path = raw_img_dir
			.join("rivers_width")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.rivers.read(lp.rivers.WIDTH, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);
	}

	//Biomes
	if OPTIONS.render_biomes {
		//println!("{}", wg_str.wg20);
		let file_path = raw_img_dir
			.join("biomes")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.biomes.read(index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Raw, lp.wi.map_size);
	}
}
