use crate::LayerPack;
use constants::app as ca;
use game_options::OPTIONS;
use io_ops::create_dir;
use io_ops::writepng::{combined_png, simple_png, GradMode, Mode};

pub fn write_images_color(
	lp: &mut LayerPack,
	world_dir: &std::path::PathBuf,
) {
	//Make a directory if none exists
	let color_img_dir = world_dir.join(ca::PATH_SAVE_IMAGES_COLOR);
	create_dir(&color_img_dir);

	//Clean proxy maps for rendering images
	let map_size = lp.wi.map_size;
	let mut array_bg = vec![0u8; lp.layer_vec_len];
	let mut array_fg = vec![0u8; lp.layer_vec_len];

	//Watermask over terraim
	if OPTIONS.render_topography {
		//println!("{}", wg_str.wg14);
		let file_path = color_img_dir
			.join("watermask_over_terrain")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.topography.read(lp.topography.TERRAIN, index);
				let fg = lp.topography.read(lp.topography.WATERMASK, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&file_path,
			GradMode::PaletteTerrain,
			GradMode::BlueBinary,
			Mode::Screen,
			lp.wi.map_size,
		);
	}

	//Temperature
	if OPTIONS.render_temperature {
		//println!("{}", wg_str.wg10);
		let file_path = color_img_dir
			.join("temperature")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.climate.read(lp.climate.TEMPERATURE, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::PaletteTemperature,
			lp.wi.map_size,
		);
	}

	//Rainfall
	if OPTIONS.render_rainfall {
		//println!("{}", wg_str.wg12);
		let file_path = color_img_dir
			.join("rainfall")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.climate.read(lp.climate.RAINFALL, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&file_path,
			GradMode::PaletteRainfall,
			lp.wi.map_size,
		);
	}

	//Georegion
	if OPTIONS.render_georegions {
		//println!("{}", wg_str.wg22);
		let file_path = color_img_dir
			.join("georegions")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.georeg_id.read(index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Random, lp.wi.map_size);
	}

	//Rivers
	if OPTIONS.render_rivers {
		//Rivers and watermasks
		//println!("{}", wg_str.wg18);
		let file_path = color_img_dir
			.join("rivers_elements")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.ELEMENT, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&file_path,
			GradMode::PaletteRegionSize,
			GradMode::PaletteRiverElement,
			Mode::Add,
			lp.wi.map_size,
		);

		//Rivers ids
		let file_path = color_img_dir
			.join("rivers_id")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers_id.read(index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&file_path,
			GradMode::RawCurved,
			GradMode::Random,
			Mode::Add,
			lp.wi.map_size,
		);

		//Rivers widths
		let file_path = color_img_dir
			.join("rivers_width")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&file_path,
			GradMode::RawCurved,
			GradMode::PaletteRiverWidth,
			Mode::Add,
			lp.wi.map_size,
		);
	}

	//Biomes
	if OPTIONS.render_biomes {
		//println!("{}", wg_str.wg20);
		let file_path = color_img_dir
			.join("biomes")
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = lp.xy.ind(i, j);
				let bg = lp.biomes.read(index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&file_path,
			GradMode::PaletteBiomes,
			GradMode::RawCurved,
			Mode::Subtract,
			lp.wi.map_size,
		);
	}
}
