use crate::LayerPack;
use constants::app as ca;
use constants::generic as cg;
use game_options::OPTIONS;
use image_ops::{combined_png, simple_png, GradMode, Mode};
use io_ops::create_dir;
use std::path::Path;

pub fn write_images_color(
	lp: &mut LayerPack,
	world_dir: &Path,
) {
	//Make a directory if none exists
	let color_img_dir = world_dir.join(ca::PATH_SAVE_IMAGES_COLOR);
	create_dir(&color_img_dir);

	//Clean proxy maps for rendering images
	let map_size = lp.wi.map_size;
	let mut array_bg = vec![cg::ZERO_U8; lp.layer_vec_len as usize];
	let mut array_fg = vec![cg::ZERO_U8; lp.layer_vec_len as usize];

	//Watermask over terraim
	if OPTIONS.render_topography {
		//println!("{}", wg_str.wg14);
		let file_path = color_img_dir
			.join(ca::NAME_IMAGE_COLOR_WATERMASK_OVER_TERRAIN)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.TERRAIN, index);
				let fg = lp.topography.read(lp.topography.WATERMASK, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
				array_fg[index_mirrored] = fg as u8;
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
			.join(ca::NAME_IMAGE_COLOR_TERRAIN)
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
			.join(ca::NAME_IMAGE_COLOR_RAINFALL)
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
		simple_png(
			&array_bg,
			&file_path,
			GradMode::PaletteRainfall,
			lp.wi.map_size,
		);
	}

	//Bioregion
	if OPTIONS.render_bioregions {
		let file_path = color_img_dir
			.join(ca::NAME_IMAGE_COLOR_BIOREGIONS)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.bioreg_id.read(index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Random, lp.wi.map_size);
	}

	//Georegion
	if OPTIONS.render_georegions {
		//println!("{}", wg_str.wg22);
		let file_path = color_img_dir
			.join(ca::NAME_IMAGE_COLOR_GEOREGIONS)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.georeg_id.read(index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
			}
		}
		simple_png(&array_bg, &file_path, GradMode::Random, lp.wi.map_size);
	}

	//Rivers
	if OPTIONS.render_rivers {
		//Rivers and watermasks
		//println!("{}", wg_str.wg18);
		let file_path = color_img_dir
			.join(ca::NAME_IMAGE_COLOR_RIVER_ELEMENTS)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.ELEMENT, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
				array_fg[index_mirrored] = fg as u8;
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
			.join(ca::NAME_IMAGE_COLOR_RIVER_IDS)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers_id.read(index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
				array_fg[index_mirrored] = fg as u8;
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
			.join(ca::NAME_IMAGE_COLOR_RIVER_WIDTH)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.topography.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
				array_fg[index_mirrored] = fg as u8;
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
			.join(ca::NAME_IMAGE_COLOR_BIOMES)
			.with_extension(ca::EXTENSION_SAVE_IMAGE);
		for y in 0..map_size {
			for x in 0..map_size {
				let index = lp.index.get(x, y);
				let bg = lp.biomes.read(index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				// Proper rendering requires mirroring the map.
				let index_mirrored = lp.index.get_mirrored_h(x, y);
				array_bg[index_mirrored] = bg as u8;
				array_fg[index_mirrored] = fg as u8;
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
