use crate::worldgen::*;
use io_ops::create_dir;
use io_ops::writepng::*;
use std::path::Path;

pub fn write_images_raw(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	save_dir: &std::path::PathBuf,
) {
	//Make a directory if none exists
	let raw_img_dir = save_dir.join(PATH_SAVE_IMAGES_RAW);
	create_dir(&raw_img_dir);

	//Clean proxy maps for rendering images
	let map_size = lp.wi.map_size;
	let mut array_bg = vec![0u8; lp.layer_vec_len];
	let mut array_fg = vec![0u8; lp.layer_vec_len];

	let xy = Index { map_size };

	//Watermask and terraim
	if options_worldgen.render_topography {
		println!("{}", wg_str.wg14);

		let file_path = raw_img_dir
			.join("terrain")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg =
					lp.topography.read(lp.topography.TERRAIN, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);

		let file_path = raw_img_dir
			.join("watermask")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp
					.topography
					.read(lp.topography.WATERMASK, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);
	}

	//Temperature
	if options_worldgen.render_temperature {
		println!("{}", wg_str.wg10);

		let file_path = raw_img_dir
			.join("temperature")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg =
					lp.climate.read(lp.climate.TEMPERATURE, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);
	}

	//Rainfall
	if options_worldgen.render_rainfall {
		println!("{}", wg_str.wg12);

		let file_path = raw_img_dir
			.join("rainfall")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.climate.read(lp.climate.RAINFALL, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);
	}

	//Rivers
	if options_worldgen.render_rivers {
		println!("{}", wg_str.wg18);

		let file_path = raw_img_dir
			.join("rivers_elements")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.rivers.read(lp.rivers.ELEMENT, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);

		//Rivers widths

		let file_path = raw_img_dir
			.join("rivers_width")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.rivers.read(lp.rivers.WIDTH, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);
	}

	//Biomes
	if options_worldgen.render_biomes {
		println!("{}", wg_str.wg20);

		let file_path = raw_img_dir
			.join("biomes")
			.with_extension(EXTENSION_SAVE_IMAGE);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.biomes.read(index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::Raw,
			lp.wi.map_size,
		);
	}
}
