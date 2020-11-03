pub mod action_ops;
pub mod data_ops;
pub mod formatting_ops;
pub mod input_ops;
pub mod printing_ops;

use action_ops::*;
use data_ops::*;
use formatting_ops::*;
use input_ops::*;
use printing_ops::*;

use codec::*;
use colored::*;
use constants_app::*;
use coords::Index;
use io_ops::decompress_to_memory;
use io_ops::toml::{options, strings};
use std::path::Path;
use ui::prompts;
use units::translate;

//temp
use io_ops::toml::palettes::biomes;
use io_ops::writepng::from_hex;

//Common shared data for all the functions
pub struct GameData {
	//World
	lp: LayerPack,
	//Structs
	options: options::Stuff,
	gm_str: strings::game_strings::Stuff,
	panic_str: strings::panic_strings::Stuff,
	ui_el: strings::ui_elements::Stuff,
	//Commands
	commands: Vec<String>,
}

pub fn start(
	options: options::Stuff,
	gm_str: strings::game_strings::Stuff,
	panic_str: strings::panic_strings::Stuff,
	ui_el: strings::ui_elements::Stuff,
) {
	//Initiate the struct that will have all the necessary data in
	//one place, and will be easy to share among functions, mods
	let mut gd = GameData {
		//Load world first
		lp: get_layerpack(&gm_str, &ui_el),
		//Move previously cloned structs here
		options,
		gm_str,
		panic_str,
		ui_el,
		//Commands
		commands: get_commands(),
	};

	//Welcoming message
	//Banner
	println!("{}", &gd.gm_str.gm1);
	//Intro message
	println!("{}", &gd.gm_str.gm2);

	//Main loop init
	let map_size = gd.lp.wi.map_size;
	let xy = Index { map_size };
	let mut x = 0;
	let mut y = 0;

	//A temporary string
	#[allow(unused_assignments)]
	let mut s = String::new();

	//Main loop
	loop {
		//Coordinates 1D,  2D, height
		//+x is north, +y is east
		let index = xy.ind(x, y);

		//Read data
		//Into data ops
		let temp = translate::get_abs(
			gd.lp.climate.read(gd.lp.climate.TEMPERATURE, index)
				as f32,
			255.0,
			gd.lp.wi.abs_temp_min as f32,
			gd.lp.wi.abs_temp_max as f32,
		) as isize;

		let rain = translate::get_abs(
			gd.lp.climate.read(gd.lp.climate.RAINFALL, index) as f32,
			255.0,
			gd.lp.wi.abs_rain_min as f32,
			gd.lp.wi.abs_rain_max as f32,
		) as usize;

		let elev = translate::get_abs(
			gd.lp.topography.read(gd.lp.topography.TERRAIN, index)
				as f32,
			255.0,
			gd.lp.wi.abs_elev_min as f32,
			gd.lp.wi.abs_elev_max as f32,
		) as usize;

		let water =
			gd.lp.topography.read(gd.lp.topography.WATERMASK, index);

		let biome = gd.lp.biomes.read(index);
		let georeg_id = gd.lp.georeg_id.read(index);

		let river_id = gd.lp.rivers_id.read(index);
		let river_width =
			gd.lp.rivers.read(gd.lp.rivers.WIDTH, index);
		let river_element =
			gd.lp.rivers.read(gd.lp.rivers.ELEMENT, index);
		let river_upstream =
			gd.lp.rivers.read(gd.lp.rivers.UPSTREAM, index);
		let river_downstream =
			gd.lp.rivers.read(gd.lp.rivers.DOWNSTREAM, index);

		//Format strings
		//Into formatting ops
		let coord_str = [
			&gd.gm_str.gm6,
			"x:",
			&(x.to_string()),
			" y:",
			&(y.to_string()),
			" index:",
			&(index.to_string()),
			" ",
		]
		.concat();

		let temp_str =
			[&gd.gm_str.gm8, &(temp.to_string()), " ℃"].concat();

		let biome_str =
			[&gd.gm_str.gm16, &(biome.to_string()), ""].concat();

		let georeg_id_str =
			[&gd.gm_str.gm17, &(georeg_id.to_string()), ""].concat();

		let rain_str =
			[&gd.gm_str.gm9, &(rain.to_string()), " mm"].concat();

		let elev_str = [
			&gd.gm_str.gm10,
			{
				//Must be less or equal
				if elev <= gd.lp.wi.waterlevel {
					s = [&(elev.to_string()), " m ", &gd.gm_str.gm14]
						.concat();
					&s
				} else {
					s = [&(elev.to_string()), " m ", &gd.gm_str.gm15]
						.concat();
					&s
				}
			},
			"",
		]
		.concat();

		let water_str = [
			&gd.gm_str.gm11,
			{
				match water {
					0 => &gd.gm_str.gm12,
					_ => {
						s = [
							&gd.gm_str.gm13,
							&(water.to_string()),
							"",
						]
						.concat();
						&s
					}
				}
			},
			"",
		]
		.concat();

		let river_str = [
			&gd.gm_str.gm18,
			{
				match river_id {
					0 => &gd.gm_str.gm19,
					_ => {
						s = [
							//id
							&gd.gm_str.gm20,
							&(river_id.to_string()),
							"\n",
							//width
							&gd.gm_str.gm21,
							&(river_width.to_string()),
							"\n",
							//element
							&gd.gm_str.gm22,
							&(river_element.to_string()),
							"\n",
							//upstream
							&gd.gm_str.gm23,
							&(river_upstream.to_string()),
							"\n",
							//downstream
							&gd.gm_str.gm24,
							&(river_downstream.to_string()),
							"",
						]
						.concat();
						&s
					}
				}
			},
			"",
		]
		.concat();

		//Into printing ops
		//Output handling
		println!("{}", coord_str);
		println!("{}", elev_str);
		println!("{}", water_str);
		println!("{}", temp_str);
		println!("{}", rain_str);
		println!("{}", biome_str);
		println!("{}", georeg_id_str);
		println!("{}", river_str);

		//temporary here
		render_land(&gd, x, y, 13);

		//Input handling
		let mut input = prompts::new_line_io("", &gd.ui_el.prompt2);
		input = prompts::autocomplete(&input, &gd.commands);
		println!("{}", &gd.ui_el.separator2);

		//Into action ops
		//Movement directions
		match input.as_str() {
			"west" => {
				y = y.saturating_sub(1);
			}
			"north" => {
				x = x.saturating_add(1);
				if x >= map_size {
					x = map_size - 1;
				}
			}
			"east" => {
				y = y.saturating_add(1);
				if y >= map_size {
					y = map_size - 1;
				}
			}
			"south" => {
				x = x.saturating_sub(1);
			}
			&_ => {}
		}

		//Teleport
		match input.as_str() {
			"x" => {
				x = prompts::new_line_io(
					&gd.gm_str.gm7,
					&gd.ui_el.prompt2,
				)
				.trim()
				.parse::<usize>()
				.unwrap_or(x);
				if x >= map_size {
					x = map_size - 1;
				}
			}
			"y" => {
				y = prompts::new_line_io(
					&gd.gm_str.gm7,
					&gd.ui_el.prompt2,
				)
				.trim()
				.parse::<usize>()
				.unwrap_or(y);
				if y >= map_size {
					y = map_size - 1;
				}
			}
			&_ => {}
		}

		//Common commands?
		match input.as_str() {
			"q" => return,
			"render surrounding" => {
				let render_size = prompts::new_line_io(
					&gd.gm_str.gm25,
					&gd.ui_el.prompt2,
				)
				.trim()
				.parse::<usize>()
				.unwrap_or(10);
				println!("{}", &gd.ui_el.separator2);
				render_land(&gd, x, y, render_size);
				println!("{}", &gd.ui_el.separator2);
			}
			"?" => {
				println!("{}", &gd.gm_str.gm2);
				//Make this better
				println!(
					"Registered commands are:\n{:?}",
					&gd.commands
				);
				println!("{}", &gd.ui_el.separator2);
			}
			&_ => {}
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MESSY CODE AHEAD ▒▒▒▒▒▒▒▒▒▒▒▒▒
//Move to other module or lib
pub fn render_land(
	gd: &GameData,
	center_x: usize,
	center_y: usize,
	render_size: usize,
) {
	let mut render_line = Vec::new();
	let map_size = gd.lp.wi.map_size;
	let xy = Index { map_size };
	let bi: biomes::Stuff = biomes::get();

	for i in 0..render_size * 2 {
		let shift_x: isize = i as isize - render_size as isize;
		for j in (0..render_size * 2).rev() {
			let shift_y: isize = j as isize - render_size as isize;

			let x = (center_x as isize - shift_x) as isize;
			let y = (center_y as isize - shift_y) as isize;

			if (x >= 0)
				&& (y >= 0) && ((x as usize) < map_size)
				&& ((y as usize) < map_size)
			{
				let index = xy.ind(x as usize, y as usize);
				let elev = gd
					.lp
					.topography
					.read(gd.lp.topography.TERRAIN, index);
				//let elev_rel = (elev as f32) / 255.0;

				let river_width =
					gd.lp.rivers.read(gd.lp.rivers.WIDTH, index);
				let biome = gd.lp.biomes.read(index);

				//Forgive me what comes below. I will fix it later.
				let mut element =
					match river_width {
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

								_ => from_hex(&bi.color_100),
							};

							match biome {
								0 => "▒▒" //water
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								1 => "▒▒" //water
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								2 => "▒▒" //water
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								3 => "▒▒" //water
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								4 => "▒▒" //water
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								5 => "⁛⁚" //desert
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								6 => "⁛⁚" //desert
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								7 => "⁛⁚" //desert
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								8 => "⁛⁚" //desert
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								9 => "██" //glacier
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								10 => "⁖⁖" //barren tundra
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								11 => "██" //mountain top
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								12 => "██" //mountain top
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								13 => "██" //mountain top
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								14 => "██" //mountain top
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								15 => "██" //mountain top
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								16 => "⚵⚶" //grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								17 => "⚵⚶" //grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								18 => "⚵⚶" //grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								19 => "⚵⚶" //grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								//⥾⨡⸶⸷⸙⸙☘⍋⍙⌇⌄⌄…‴⏃
								20 => "⥾⏃" //woodland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								21 => "⥾⏃" //woodland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								22 => "⥾⏃" //woodland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								23 => "⍋⏃" //forest
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								24 => "⍋⏃" //forest
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								25 => "⍋⏃" //forest
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								26 => "⥾~" //swamp
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								27 => "⥾~" //swamp
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								28 => "⥾~" //swamp
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								29 => "☘⍋" //rainforest
									.bold()
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								30 => "☘⍋" //rainforest
									.bold()
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								31 => "☘⍋" //rainforest
									.bold()
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								32 => "⥾⚶" //shrubland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								33 => "⥾⚶" //shrubland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								34 => "⥾⚶" //shrubland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								35 => "…⚵" //alpine grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								36 => "…⚵" //alpine grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),
								37 => "…⚵" //alpine grassland
									.truecolor(
										argb[1], argb[2], argb[3],
									),

								_ => "▓▓".truecolor(
									argb[1], argb[2], argb[3],
								),
							}
						} //match biomes block
					}; //match rivers

				//Add elevation color
				element = element
					.on_truecolor(elev as u8, elev as u8, elev as u8);

				//Swap characters every other row for better visuals
				//Or do it randomly
				let random =
					prng::get(0.0, 1.0, gd.lp.wi.seed, index);

				if random < 0.5 {
					//if x % 2 == 0 {
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
						.truecolor(
							colors_fg.0,
							colors_fg.1,
							colors_fg.2,
						)
						.on_truecolor(
							colors_bg.0,
							colors_bg.1,
							colors_bg.2,
						);

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
				if ((x as usize) == center_x)
					&& ((y as usize) == center_y)
				{
					render_line.push(element.reversed());
				} else {
					render_line.push(element);
				}
			} else {
				//Outside of data range
				render_line.push("██".truecolor(20, 20, 20));
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
