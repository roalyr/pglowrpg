use crate::toml::colors::colors_river_size;

use crate::writepng::from_hex;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let rs: colors_river_size::Stuff = colors_river_size::get();

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v {
			0 => from_hex(&rs.color_0),

			1 => from_hex(&rs.color_1),
			2 => from_hex(&rs.color_2),
			3 => from_hex(&rs.color_3),
			4 => from_hex(&rs.color_4),
			5 => from_hex(&rs.color_5),
			6 => from_hex(&rs.color_6),
			7 => from_hex(&rs.color_7),
			8 => from_hex(&rs.color_8),
			9 => from_hex(&rs.color_9),
			10 => from_hex(&rs.color_10),
			11 => from_hex(&rs.color_11),
			12 => from_hex(&rs.color_12),

			_ => from_hex(&rs.color_100),
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index + 0] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
