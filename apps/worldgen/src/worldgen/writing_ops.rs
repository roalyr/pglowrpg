use crate::worldgen::*;

pub fn write_images(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	_options_debug: &options::options_debug::Stuff,
) {
	let save_dir = "save/";
	match std::fs::create_dir(save_dir){
		Err(e) => {
			match e.kind() {
				std::io::ErrorKind::AlreadyExists => println!("Using folder {}", save_dir),
				_ => println!("UNACCOUNTED ERROR when making ./save dir: {:?}", e.kind()),
			}
		},
		Ok(()) => {},
	}
		
		
		
	let map_size = lp.wi.map_size;

	//Clean proxy maps for rendering images
	let mut array_bg = vec![0u8; lp.layer_vec_len];
	let mut array_fg = vec![0u8; lp.layer_vec_len];

	let xy = Index { map_size };

	//Watermask over terraim
	if options_worldgen.render_topography {
		println!("{}", wg_str.wg14);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg =
					lp.topography.read(lp.topography.TERRAIN, index);
				let fg = lp
					.topography
					.read(lp.topography.WATERMASK, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "watermask_over_terrain.png"].concat(),
			GradMode::PaletteTerrain,
			GradMode::BlueBinary,
			Mode::Screen,
			lp.wi.map_size,
		);
	}

	//Temperature
	if options_worldgen.render_temperature {
		println!("{}", wg_str.wg10);
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
			&[save_dir, "temperature.png"].concat(),
			GradMode::PaletteTemperature,
			lp.wi.map_size,
		);
	}

	//Rainfall
	if options_worldgen.render_rainfall {
		println!("{}", wg_str.wg12);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.climate.read(lp.climate.RAINFALL, index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&[save_dir, "rainfall.png"].concat(),
			GradMode::PaletteRainfall,
			lp.wi.map_size,
		);
	}

	//Georegion
	if options_worldgen.render_georegions {
		println!("{}", wg_str.wg22);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.georeg_id.read(index);
				array_bg[index] = bg as u8;
			}
		}
		simple_png(
			&array_bg,
			&[save_dir, "georegions.png"].concat(),
			GradMode::Random,
			lp.wi.map_size,
		);
	}

	//Rivers
	if options_worldgen.render_rivers {
		println!("{}", wg_str.wg18);

		//Rivers and watermasks
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp
					.topography
					.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.ELEMENT, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rivers_elements.png"].concat(),
			GradMode::PaletteRegionSize,
			GradMode::PaletteRiverElement,
			Mode::Add,
			lp.wi.map_size,
		);

		//Rivers ids
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp
					.topography
					.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers_id.read(index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rivers_id.png"].concat(),
			GradMode::RawCurved,
			GradMode::Random,
			Mode::Add,
			lp.wi.map_size,
		);

		//Rivers widths
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp
					.topography
					.read(lp.topography.WATERMASK, index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rivers_width.png"].concat(),
			GradMode::RawCurved,
			GradMode::PaletteRiverWidth,
			Mode::Add,
			lp.wi.map_size,
		);
	}

	//Biomes
	if options_worldgen.render_biomes {
		println!("{}", wg_str.wg20);
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.biomes.read(index);
				let fg = lp.rivers.read(lp.rivers.WIDTH, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "biomes.png"].concat(),
			GradMode::PaletteBiomes,
			GradMode::RawCurved,
			Mode::Subtract,
			lp.wi.map_size,
		);
	}
}
