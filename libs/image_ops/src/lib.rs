pub mod combine;
pub mod gradients;

use std::io::BufWriter;

#[derive(Copy, Clone)]
pub enum Mode {
	Add,
	Subtract,
	Divide,
	Multiply,
	Overlay,
	Screen,
}

#[derive(Copy, Clone)]
pub enum GradMode {
	PaletteTerrain,
	PaletteTemperature,
	PaletteBiomes,
	PaletteRainfall,
	PaletteRegionSize,
	PaletteRiverWidth,
	PaletteRiverElement,

	Raw,
	RawInverse,
	RawCurved,
	RawBinary,
	RawBinaryInverse,

	Blue,
	BlueInverse,
	BlueCurved,
	BlueBinary,
	BlueBinaryInverse,

	Random,
}

// TODO: make things more neat here via macros?
pub fn combined_png(
	array_bg: &Vec<u8>,
	array_fg: &Vec<u8>,
	path: &std::path::PathBuf,
	grad_bg_mode: GradMode,
	grad_fg_mode: GradMode,
	mode: Mode,
	png_size: usize,
) {
	let size = array_bg.len();
	let file = io_ops::create_file_overwrite(&path);
	let bufw = &mut BufWriter::new(file);
	let mut encoder = png::Encoder::new(bufw, png_size as u32, png_size as u32);

	let mut idat = vec![0; size * 4];
	//let mut idat = Vec::new();

	encoder.set_color(png::ColorType::RGBA);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header().unwrap();

	let idat_bg = match_grad(&array_bg, grad_bg_mode);
	let idat_fg = match_grad(&array_fg, grad_fg_mode);

	match mode {
		Mode::Add => {
			combine::add::get(&mut idat, idat_bg, idat_fg);
		}
		Mode::Subtract => {
			combine::subtract::get(&mut idat, idat_bg, idat_fg);
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
	}
	writer.write_image_data(&idat).unwrap();
}

pub fn simple_png(
	array: &Vec<u8>,
	path: &std::path::PathBuf,
	grad_mode: GradMode,
	png_size: usize,
) {
	let file = io_ops::create_file_overwrite(&path);
	let bufw = &mut BufWriter::new(file);
	let mut encoder = png::Encoder::new(bufw, png_size as u32, png_size as u32);

	encoder.set_color(png::ColorType::RGBA);
	encoder.set_depth(png::BitDepth::Eight);

	let mut writer = encoder.write_header().unwrap();

	let idat = match_grad(&array, grad_mode);

	writer.write_image_data(&idat).unwrap();
}

fn match_grad(
	array: &Vec<u8>,
	grad_mode: GradMode,
) -> Vec<u8> {
	match grad_mode {
		//Value from palette (.toml colorcodes)
		GradMode::PaletteTerrain => gradients::palettes::terrain::get(array),
		GradMode::PaletteBiomes => gradients::palettes::biomes::get(array),
		GradMode::PaletteRainfall => gradients::palettes::rainfall::get(array),
		GradMode::PaletteTemperature => {
			gradients::palettes::temperature::get(array)
		}
		GradMode::PaletteRegionSize => gradients::palettes::region_size::get(array),
		GradMode::PaletteRiverWidth => gradients::palettes::river_width::get(array),
		GradMode::PaletteRiverElement => {
			gradients::palettes::river_element::get(array)
		}

		//Value from finction (black-white image)
		GradMode::Raw => gradients::raw::get(array),
		GradMode::RawInverse => gradients::raw::get_inverse(array),
		GradMode::RawCurved => gradients::raw::get_curved(array),
		GradMode::RawBinary => gradients::raw::get_binary(array),
		GradMode::RawBinaryInverse => gradients::raw::get_binary_inverse(array),

		//Value from finction (black-blue image)
		GradMode::Blue => gradients::blue::get(array),
		GradMode::BlueInverse => gradients::blue::get_inverse(array),
		GradMode::BlueCurved => gradients::blue::get_curved(array),
		GradMode::BlueBinary => gradients::blue::get_binary(array),
		GradMode::BlueBinaryInverse => gradients::blue::get_binary_inverse(array),

		//Random color to value according to constant seed
		GradMode::Random => gradients::random::get(array),
	}
}

pub fn from_hex(str_hex: &str) -> Vec<u8> {
	let str_v = String::from(str_hex);
	let s = &str_v[1..];
	match hex::decode(s) {
		Ok(val) => val,
		Err(_) => {
			//ARGB error code
			let val: Vec<u8> = [127, 255, 0, 255].to_vec();
			val
		}
	}
}
