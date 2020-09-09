use crate::toml::palettes::river_element;

use crate::writepng::from_hex;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let re: river_element::Stuff = river_element::get();

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v {
			0 => from_hex(&re.color_0),

			1 => from_hex(&re.color_1),
			2 => from_hex(&re.color_2),
			3 => from_hex(&re.color_3),
			4 => from_hex(&re.color_4),
			5 => from_hex(&re.color_5),

			_ => from_hex(&re.color_100),
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
