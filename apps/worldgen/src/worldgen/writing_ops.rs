use crate::worldgen::*;

pub fn write_images(
	lp: &mut LayerPack,
	wg_str: &strings::worldgen_strings::Stuff,
	options_worldgen: &options::options_worldgen::Stuff,
	_options_debug: &options::options_debug::Stuff,
) {
	let save_dir = "/save/";

	//▒▒▒▒▒▒▒▒▒▒▒▒ WRITE ▒▒▒▒▒▒▒▒▒▒▒▒▒
	//elevation
	if options_worldgen.render_topography {
		println!("{}", wg_str.wg8);
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.to.array_map),
			//adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, "terrain.png"].concat(),
			GradMode::Raw,
			GradMode::BlackBlueBin,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//temperature
	if options_worldgen.render_temperature {
		println!("{}", wg_str.wg10);
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.te.array_map),
			//translate::decode(&lp.te.array_map),
			&[save_dir, "temperature.png"].concat(),
			GradMode::Temperature,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//rainfall
	if options_worldgen.render_rainfall {
		println!("{}", wg_str.wg12);
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.ra.array_map),
			//adapt_png(translate::decode16(&lp.ri_mask.array_map)),
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

		//watermask w rivers
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.wm.array_map),
			//adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, "watermask.png"].concat(),
			GradMode::RegSize,
			GradMode::RiverMask,
			Mode::Add,
			lp.wi.map_size,
		);

		//watermask over elev
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.to.array_map),
			//translate::decode(&lp.wm.array_map),
			&[save_dir, "watermask_over_elev.png"].concat(),
			GradMode::Topography,
			GradMode::BlackBlueBin,
			Mode::Screen,
			lp.wi.map_size,
		);

		//rivers
		println!("{}", wg_str.wg18);

		//ids
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.wm.array_map),
			//adapt_png(translate::decode16(&lp.ri_id.array_map)),
			&[save_dir, "rivers_id.png"].concat(),
			GradMode::BlackWhitePow,
			GradMode::Random,
			Mode::Add,
			lp.wi.map_size,
		);
		//widths
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.wm.array_map),
			//translate::decode(&lp.ri_width.array_map),
			&[save_dir, "rivers_width.png"].concat(),
			GradMode::BlackWhitePow,
			GradMode::RiverSize,
			Mode::Add,
			lp.wi.map_size,
		);
	}

	//biomes
	if options_worldgen.render_biomes {
		println!("{}", wg_str.wg20);
		combined_png(
			array_bg,
			array_fg,
			//translate::decode(&lp.bi.array_map),
			//adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, "biomes.png"].concat(),
			GradMode::Biomes,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}

	//georegion
	if options_worldgen.render_georegions {
		println!("{}", wg_str.wg22);
		combined_png(
			array_bg,
			array_fg,
			//adapt_png(translate::decode16(&lp.ge.array_map)),
			//translate::decode(&lp.to.array_map),
			&[save_dir, "georegions.png"].concat(),
			GradMode::Random,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}
}
