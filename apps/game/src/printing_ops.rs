use crate::struct_ops::{GameData, GameStrings};

use colored::*;
use image_ops::from_hex;
use io_ops::readron::palettes::biomes;

pub fn print_strings_basic(gs: &GameStrings) {
	println!("{}", gs.coord_str);
	println!("{}", gs.elev_str);
	println!("{}", gs.water_str);
	println!("{}", gs.temp_str);
	println!("{}", gs.rain_str);
	println!("{}", gs.biome_str);
	println!("{}", gs.georeg_id_str);
	println!("{}", gs.river_str);
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MESSY CODE AHEAD ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn map_render_land(
	gd: &mut GameData,
	center_y: usize,
	center_x: usize,
) {
	let mut render_line = Vec::new();
	let bi: biomes::Stuff = biomes::get();

	for j in 0..gd.map_render_size * 2 {
		let shift_y: isize = j as isize - gd.map_render_size as isize;
		// The way it prints, this should be reversed.
		for i in (0..gd.map_render_size * 2).rev() {
			let shift_x: isize = i as isize - gd.map_render_size as isize;

			let xx = (center_x as isize - shift_x) as isize;
			let yy = (center_y as isize - shift_y) as isize;

			if (xx >= 0)
				&& (yy >= 0)
				&& ((xx as usize) < gd.lp.wi.map_size)
				&& ((yy as usize) < gd.lp.wi.map_size)
			{
				let render_index = gd.lp.index.get(xx as usize, yy as usize);
				let elev = gd
					.lp
					.topography
					.read(gd.lp.topography.TERRAIN, render_index);
				//let elev_rel = (elev as f32) / 255.0;

				let river_width = gd.lp.rivers.read(gd.lp.rivers.WIDTH, render_index);
				let biome = gd.lp.biomes.read(render_index);

				//Forgive me what comes below. I will fix it later.
				let mut element = match river_width {
					//⁓≃≅≈≊≋
					1 => ".⁓".truecolor(70, 100, 145).bold(),
					2 => "⁓⁓".truecolor(70, 100, 145).bold(),
					3 => "⁓⩪".truecolor(70, 100, 145).bold(),
					4 => "⩪⩪".truecolor(70, 130, 190).bold(),
					5 => "⩪≃".truecolor(70, 130, 190).bold(),
					6 => "≃≃".truecolor(70, 130, 190).bold(),
					7 => "≃≊".truecolor(70, 150, 230).bold(),
					8 => "≊≊".truecolor(70, 150, 230).bold(),
					9 => "≊≋".truecolor(70, 150, 230).bold(),
					10 => "≋≋".truecolor(70, 195, 230).bold(),
					11 => "≋⩭".truecolor(70, 195, 230).bold(),
					12 => "⩰⩰".truecolor(70, 195, 230).bold(),

					_ => {
						//put biome tiles where there is no river tile
						let argb: Vec<u8> = match biome {
							0 => from_hex(&bi.color_0),
							1 => from_hex(&bi.color_1),
							2 => from_hex(&bi.color_2),
							3 => from_hex(&bi.color_3),
							4 => from_hex(&bi.color_4),
							5 => from_hex(&bi.color_5),
							6 => from_hex(&bi.color_6),
							7 => from_hex(&bi.color_7),
							8 => from_hex(&bi.color_8),
							9 => from_hex(&bi.color_9),
							10 => from_hex(&bi.color_10),
							11 => from_hex(&bi.color_11),
							12 => from_hex(&bi.color_12),
							13 => from_hex(&bi.color_13),
							14 => from_hex(&bi.color_14),
							15 => from_hex(&bi.color_15),
							16 => from_hex(&bi.color_16),
							17 => from_hex(&bi.color_17),
							18 => from_hex(&bi.color_18),
							19 => from_hex(&bi.color_19),
							20 => from_hex(&bi.color_20),
							21 => from_hex(&bi.color_21),
							22 => from_hex(&bi.color_22),
							23 => from_hex(&bi.color_23),
							24 => from_hex(&bi.color_24),
							25 => from_hex(&bi.color_25),
							26 => from_hex(&bi.color_26),
							27 => from_hex(&bi.color_27),
							28 => from_hex(&bi.color_28),
							29 => from_hex(&bi.color_29),
							30 => from_hex(&bi.color_30),
							31 => from_hex(&bi.color_31),
							32 => from_hex(&bi.color_32),
							33 => from_hex(&bi.color_33),
							34 => from_hex(&bi.color_34),
							35 => from_hex(&bi.color_35),
							36 => from_hex(&bi.color_36),
							37 => from_hex(&bi.color_37),

							_ => {
								println!("ERROR: couldn't print BIOMES colored symbol, unexpected value.");
								panic!();
							}
						};

						match biome {
							0 => "▒▒" //water
								.truecolor(argb[1], argb[2], argb[3]),
							1 => "▒▒" //water
								.truecolor(argb[1], argb[2], argb[3]),
							2 => "▒▒" //water
								.truecolor(argb[1], argb[2], argb[3]),
							3 => "▒▒" //water
								.truecolor(argb[1], argb[2], argb[3]),
							4 => "▒▒" //water
								.truecolor(argb[1], argb[2], argb[3]),
							5 => "⁛⁚" //desert
								.truecolor(argb[1], argb[2], argb[3]),
							6 => "⁛⁚" //desert
								.truecolor(argb[1], argb[2], argb[3]),
							7 => "⁛⁚" //desert
								.truecolor(argb[1], argb[2], argb[3]),
							8 => "⁛⁚" //desert
								.truecolor(argb[1], argb[2], argb[3]),
							9 => "██" //glacier
								.truecolor(argb[1], argb[2], argb[3]),
							10 => "⁖⁖" //barren tundra
								.truecolor(argb[1], argb[2], argb[3]),
							11 => "██" //mountain top
								.truecolor(argb[1], argb[2], argb[3]),
							12 => "██" //mountain top
								.truecolor(argb[1], argb[2], argb[3]),
							13 => "██" //mountain top
								.truecolor(argb[1], argb[2], argb[3]),
							14 => "██" //mountain top
								.truecolor(argb[1], argb[2], argb[3]),
							15 => "██" //mountain top
								.truecolor(argb[1], argb[2], argb[3]),
							16 => "⚵⚶" //grassland
								.truecolor(argb[1], argb[2], argb[3]),
							17 => "⚵⚶" //grassland
								.truecolor(argb[1], argb[2], argb[3]),
							18 => "⚵⚶" //grassland
								.truecolor(argb[1], argb[2], argb[3]),
							19 => "⚵⚶" //grassland
								.truecolor(argb[1], argb[2], argb[3]),
							//⥾⨡⸶⸷⸙⸙☘⍋⍙⌇⌄⌄…‴⏃
							20 => "⥾⏃" //woodland
								.truecolor(argb[1], argb[2], argb[3]),
							21 => "⥾⏃" //woodland
								.truecolor(argb[1], argb[2], argb[3]),
							22 => "⥾⏃" //woodland
								.truecolor(argb[1], argb[2], argb[3]),
							23 => "⍋⏃" //forest
								.truecolor(argb[1], argb[2], argb[3]),
							24 => "⍋⏃" //forest
								.truecolor(argb[1], argb[2], argb[3]),
							25 => "⍋⏃" //forest
								.truecolor(argb[1], argb[2], argb[3]),
							26 => "⥾~" //swamp
								.truecolor(argb[1], argb[2], argb[3]),
							27 => "⥾~" //swamp
								.truecolor(argb[1], argb[2], argb[3]),
							28 => "⥾~" //swamp
								.truecolor(argb[1], argb[2], argb[3]),
							29 => "☘⍋" //rainforest
								.bold()
								.truecolor(argb[1], argb[2], argb[3]),
							30 => "☘⍋" //rainforest
								.bold()
								.truecolor(argb[1], argb[2], argb[3]),
							31 => "☘⍋" //rainforest
								.bold()
								.truecolor(argb[1], argb[2], argb[3]),
							32 => "⥾⚶" //shrubland
								.truecolor(argb[1], argb[2], argb[3]),
							33 => "⥾⚶" //shrubland
								.truecolor(argb[1], argb[2], argb[3]),
							34 => "⥾⚶" //shrubland
								.truecolor(argb[1], argb[2], argb[3]),
							35 => "…⚵" //alpine grassland
								.truecolor(argb[1], argb[2], argb[3]),
							36 => "…⚵" //alpine grassland
								.truecolor(argb[1], argb[2], argb[3]),
							37 => "…⚵" //alpine grassland
								.truecolor(argb[1], argb[2], argb[3]),

							_ => "▓▓".truecolor(argb[1], argb[2], argb[3]),
						}
					} //match biomes block
				}; //match rivers

				//Add elevation color
				element = element.on_truecolor(elev as u8, elev as u8, elev as u8);

				//Swap characters every other row for better visuals
				//Or do it randomly
				let random = pseudo_rng::get(0.0, 1.0, gd.lp.wi.seed, render_index);

				if random < 0.5 {
					// Keep the unwraps for now.
					let orig_colors_fg = element.fgcolor().unwrap();
					let orig_colors_bg = element.bgcolor().unwrap();
					let orig_style = element.style();

					let colors_fg = match orig_colors_fg {
						Color::TrueColor { r, g, b } => (r, g, b),
						_ => (255, 255, 255),
					};

					let colors_bg = match orig_colors_bg {
						Color::TrueColor { r, g, b } => (r, g, b),
						_ => (0, 0, 0),
					};

					//Re-apply the colors
					element = element
						.chars()
						.rev()
						.collect::<String>()
						.truecolor(colors_fg.0, colors_fg.1, colors_fg.2)
						.on_truecolor(colors_bg.0, colors_bg.1, colors_bg.2);

					//Re-apply styles one by one

					if orig_style.contains(Styles::Bold) {
						element = element.bold();
					}

					if orig_style.contains(Styles::Dimmed) {
						element = element.dimmed();
					}

					if orig_style.contains(Styles::Underline) {
						element = element.underline();
					}

					if orig_style.contains(Styles::Reversed) {
						element = element.reversed();
					}

					if orig_style.contains(Styles::Italic) {
						element = element.italic();
					}

					if orig_style.contains(Styles::Hidden) {
						element = element.hidden();
					}

					if orig_style.contains(Styles::Blink) {
						element = element.blink();
					}

					if orig_style.contains(Styles::Strikethrough) {
						element = element.strikethrough();
					}
				}

				//Highlight central tile to see current position
				if ((yy as usize) == center_y) && ((xx as usize) == center_x) {
					render_line.push(element.reversed());
				} else {
					render_line.push(element);
				}
			} else {
				//Outside of data range
				render_line.push(".'".truecolor(20, 20, 20));
			}
		} //for j
		for item in render_line.iter() {
			print!("{}", item);
		}
		print!("|");
		println!();
		render_line.clear();
	} //for i
}
