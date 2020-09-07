use crate::worldgen::*;

pub fn write_images(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	_options_debug: &options::options_debug::Stuff,
) {
	let save_dir = "save/";
	let map_size = lp.wi.map_size;

	let mut array_bg = vec![0; lp.layer_vec_len];
	let mut array_fg = vec![0; lp.layer_vec_len];

	let m_terrain = lp.topography.masks.terrain;
	let m_wmask = lp.topography.masks.watermask;

	let m_temp = lp.climate.masks.temperature;
	let m_rain = lp.climate.masks.rainfall;

	let m_elem = lp.rivers.masks.element;
	let m_width = lp.rivers.masks.width;

	let xy = Index { map_size };

	//▒▒▒▒▒▒▒▒▒▒▒▒ WRITE ▒▒▒▒▒▒▒▒▒▒▒▒▒
	//elevation
	if options_worldgen.render_topography {
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.topography.read(m_terrain, index);
				let fg = lp.rivers.read(m_elem, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}

		println!("{}", wg_str.wg8);
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "terrain.png"].concat(),
			GradMode::Raw,
			GradMode::BlackBlueBin,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//temperature
	if options_worldgen.render_temperature {
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.climate.read(m_temp, index);
				//let fg = lp.rivers.read(m_elem, index);
				array_bg[index] = bg as u8;
				//array_fg[index] = fg as u8;
			}
		}

		println!("{}", wg_str.wg10);
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "temperature.png"].concat(),
			GradMode::Temperature,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//rainfall
	if options_worldgen.render_rainfall {
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.climate.read(m_rain, index);
				//let fg = lp.rivers.read(m_elem, index);
				array_bg[index] = bg as u8;
				//array_fg[index] = fg as u8;
			}
		}

		println!("{}", wg_str.wg12);
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rainfall.png"].concat(),
			GradMode::Rainfall,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//rivers and watermasks
	if options_worldgen.render_rivers {
		println!("{}", wg_str.wg14);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.topography.read(m_wmask, index);
				let fg = lp.rivers.read(m_elem, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}

		//watermask w rivers
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "watermask.png"].concat(),
			GradMode::RegSize,
			GradMode::RiverMask,
			Mode::Add,
			lp.wi.map_size,
		);

		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.topography.read(m_terrain, index);
				let fg = lp.topography.read(m_wmask, index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}

		//watermask over elev
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "watermask_over_elev.png"].concat(),
			GradMode::Topography,
			GradMode::BlackBlueBin,
			Mode::Screen,
			lp.wi.map_size,
		);

		//rivers
		println!("{}", wg_str.wg18);

		//ids
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.topography.read(m_wmask, index);
				let fg = lp.rivers_id.read(index);
				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}

		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rivers_id.png"].concat(),
			GradMode::BlackWhitePow,
			GradMode::Random,
			Mode::Add,
			lp.wi.map_size,
		);

		//widths
		for i in 0..map_size {
			for j in 0..map_size {
				let index = xy.ind(i, j);
				let bg = lp.topography.read(m_wmask, index);
				let fg = lp.rivers.read(m_width, index);

				array_bg[index] = bg as u8;
				array_fg[index] = fg as u8;
			}
		}

		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "rivers_width.png"].concat(),
			GradMode::BlackWhitePow,
			GradMode::RiverSize,
			Mode::Add,
			lp.wi.map_size,
		);
	}

	//biomes

	for i in 0..map_size {
		for j in 0..map_size {
			let index = xy.ind(i, j);
			let bg = lp.biomes.read(index);
			//let fg = lp.rivers.read(m_elem, index);
			array_bg[index] = bg as u8;
			//array_fg[index] = fg as u8;
		}
	}

	if options_worldgen.render_biomes {
		println!("{}", wg_str.wg20);
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "biomes.png"].concat(),
			GradMode::Biomes,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//georegion

	for i in 0..map_size {
		for j in 0..map_size {
			let index = xy.ind(i, j);
			let bg = lp.georeg_id.read(index);
			//let fg = lp.rivers.read(m_elem, index);
			array_bg[index] = bg as u8;
			//array_fg[index] = fg as u8;
		}
	}

	if options_worldgen.render_georegions {
		println!("{}", wg_str.wg22);
		combined_png(
			&array_bg,
			&array_fg,
			&[save_dir, "georegions.png"].concat(),
			GradMode::Random,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}
}
