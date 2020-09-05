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
			translate::decode(&lp.to.array_map),
			//translate::decode(&lp.ri_mask.array_map),
			adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, &lp.to.layer_name, ".png"].concat(),
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
			translate::decode(&lp.te.array_map),
			translate::decode(&lp.te.array_map),
			&[save_dir, &lp.te.layer_name, ".png"].concat(),
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
			translate::decode(&lp.ra.array_map),
			//translate::decode(&lp.ri_mask.array_map),
			adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, &lp.ra.layer_name, ".png"].concat(),
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
			translate::decode(&lp.wm.array_map),
			//translate::decode(&lp.ri_mask.array_map),
			adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, &lp.wm.layer_name, ".png"].concat(),
			GradMode::RegSize,
			GradMode::RiverMask,
			Mode::Add,
			lp.wi.map_size,
		);

		//watermask over elev
		combined_png(
			translate::decode(&lp.to.array_map),
			translate::decode(&lp.wm.array_map),
			&[save_dir, &lp.wm.layer_name, "_over_elev", ".png"].concat(),
			GradMode::Topography,
			GradMode::BlackBlueBin,
			Mode::Screen,
			lp.wi.map_size,
		);

		//river noise
		combined_png(
			translate::decode(&lp.rto.array_map),
			//translate::decode(&lp.ri_mask.array_map),
			adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, &lp.rto.layer_name, ".png"].concat(),
			GradMode::Topography,
			GradMode::Raw,
			Mode::FgIgnoreZero,
			lp.wi.map_size,
		);

		//rivers
		println!("{}", wg_str.wg18);

		//ids
		combined_png(
			translate::decode(&lp.wm.array_map),
			//make generic type here
			adapt_png(translate::decode16(&lp.ri_id.array_map)),
			&[save_dir, &lp.ri_id.layer_name, ".png"].concat(),
			GradMode::BlackWhitePow,
			GradMode::Random,
			Mode::Add,
			lp.wi.map_size,
		);
		//widths
		combined_png(
			translate::decode(&lp.wm.array_map),
			translate::decode(&lp.ri_width.array_map),
			&[save_dir, &lp.ri_width.layer_name, ".png"].concat(),
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
			translate::decode(&lp.bi.array_map),
			adapt_png(translate::decode16(&lp.ri_mask.array_map)),
			&[save_dir, &lp.bi.layer_name, ".png"].concat(),
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
			adapt_png(translate::decode16(&lp.ge.array_map)),
			translate::decode(&lp.to.array_map),
			&[save_dir, &lp.ge.layer_name, ".png"].concat(),
			GradMode::Random,
			GradMode::Raw,
			Mode::OnlyBg,
			lp.wi.map_size,
		);
	}
}

pub fn get_data_size(lp: &LayerPack) {
	let data_size = (lp.te.deep_size_of()
		+ lp.ra.deep_size_of()
		+ lp.to.deep_size_of()
		+ lp.rto.deep_size_of()
		+ lp.wm.deep_size_of()
		+ lp.ri_mask.deep_size_of()
		+ lp.ri_id.deep_size_of()
		+ lp.ri_width.deep_size_of()
		+ lp.ge.deep_size_of()
		+ lp.bi.deep_size_of()) as f32
		/ 1_000_000.0;

	println!("Summary layer size (MB): {:?}", data_size);

	let data_size_un = (translate::decode(&lp.te.array_map)
		.deep_size_of()
		+ translate::decode(&lp.ra.array_map).deep_size_of()
		+ translate::decode(&lp.to.array_map).deep_size_of()
		+ translate::decode(&lp.rto.array_map).deep_size_of()
		+ translate::decode(&lp.wm.array_map).deep_size_of()
		+ translate::decode(&lp.ri_mask.array_map).deep_size_of()
		+ translate::decode16(&lp.ri_id.array_map).deep_size_of()
		+ translate::decode(&lp.ri_width.array_map).deep_size_of()
		+ translate::decode(&lp.ge.array_map).deep_size_of()
		+ translate::decode(&lp.bi.array_map).deep_size_of())
		as f32 / 1_000_000.0;

	println!(
		"Summary uncompressed layer size (MB): {:?}",
		data_size_un
	);
}
