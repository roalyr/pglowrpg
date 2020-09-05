pub mod combine;
pub mod gradients;

use std::fs::File;
use std::io::BufWriter;
use std::path::Path;

#[derive(Copy, Clone)]
pub enum Mode {
	OnlyBg,
	OnlyFg,
	Add,
	Divide,
	Multiply,
	Overlay,
	Screen,
	Shade,
	FgIgnoreZero,
}

#[derive(Copy, Clone)]
pub enum GradMode {
	None,
	Topography,
	Biomes,
	Raw,
	Rainfall,
	Temperature,
	BlackWhitePow,
	BlackBluePow,
	BlackWhiteBin,
	BlackBlueBin,
	RedBlue,
	RegSize,
	RiverSize,
	RawInverse,
	Random,
	RiverMask,
}

pub fn combined_png(
	array_bg: Vec<u8>,
	array_fg: Vec<u8>,
	pngname: &str,
	grad_bg_mode: GradMode,
	grad_fg_mode: GradMode,
	mode: Mode,
	png_size: usize,
) {
	let size = array_bg.len();
	let path = Path::new(&pngname);
	let file = File::create(path).unwrap();
	let bufw = &mut BufWriter::new(file);
	let mut encoder =
		png::Encoder::new(bufw, png_size as u32, png_size as u32);

	let mut idat = vec![0; size * 4];
	//let mut idat = Vec::new();

	encoder.set_color(png::ColorType::RGBA);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header().unwrap();

	let idat_bg = match_grad(array_bg, grad_bg_mode);

	let idat_fg = match_grad(array_fg, grad_fg_mode);

	match mode {
		Mode::OnlyBg => {
			idat = idat_bg;
		}
		Mode::OnlyFg => {
			idat = idat_fg;
		}
		Mode::Add => {
			combine::add::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Multiply => {
			combine::multiply::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Divide => {
			combine::divide::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Overlay => {
			combine::overlay::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Screen => {
			combine::screen::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Shade => {
			combine::shade::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::FgIgnoreZero => {
			combine::fg_ignore_zero::get(&mut idat, idat_bg, idat_fg);
		}
	}
	writer.write_image_data(&idat).unwrap();
}

fn match_grad(
	array: Vec<u8>,
	grad_mode: GradMode,
) -> Vec<u8> {
	match grad_mode {
		GradMode::None => gradients::raw::get(array),
		GradMode::Topography => gradients::topography::get(array),
		GradMode::Biomes => gradients::biomes::get(array),
		GradMode::Raw => gradients::raw::get(array),
		GradMode::BlackWhitePow => {
			gradients::black_white_pow::get(array)
		}
		GradMode::BlackBluePow => {
			gradients::black_blue_pow::get(array)
		}
		GradMode::BlackWhiteBin => {
			gradients::black_white_bin::get(array)
		}
		GradMode::BlackBlueBin => {
			gradients::black_blue_bin::get(array)
		}
		GradMode::Random => gradients::random::get(array),
		GradMode::Rainfall => gradients::rainfall::get(array),
		GradMode::RedBlue => gradients::red_blue::get(array),
		GradMode::Temperature => gradients::temperature::get(array),
		GradMode::RawInverse => gradients::raw_inverse::get(array),
		GradMode::RegSize => gradients::reg_size::get(array),
		GradMode::RiverSize => gradients::river_size::get(array),
		GradMode::RiverMask => gradients::river_mask::get(array),
	}
}

pub fn from_hex(str_hex: &str) -> Vec<u8> {
	let str_v = String::from(str_hex);
	let s = &str_v[1..];
	match hex::decode(s) {
		Ok(val) => val,
		Err(_) => {
			let val: Vec<u8> = [255, 255, 0, 255].to_vec();
			val
		}
	}
}

pub fn adapt_png(array: Vec<u16>) -> Vec<u8> {
	let size = array.len();
	let mut array_new = vec![0; size];
	for index in 0..size {
		array_new[index] = array[index] as u8;
	}
	array_new
}
