use crate::toml::colors::colors_topography;

use crate::writepng::from_hex;

pub fn get(array: Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let to: colors_topography::Stuff = colors_topography::get();

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v {
			0 => from_hex(&to.color_0),
			1 => from_hex(&to.color_1),
			2 => from_hex(&to.color_3),
			3 => from_hex(&to.color_4),
			4 => from_hex(&to.color_5),
			5 => from_hex(&to.color_6),
			6 => from_hex(&to.color_7),
			7..=8 => from_hex(&to.color_8),
			9..=10 => from_hex(&to.color_9),
			11..=12 => from_hex(&to.color_10),
			13..=15 => from_hex(&to.color_11),
			16..=19 => from_hex(&to.color_12),
			20..=23 => from_hex(&to.color_13),
			24..=28 => from_hex(&to.color_14),
			29..=34 => from_hex(&to.color_15),
			35..=42 => from_hex(&to.color_16),
			43..=52 => from_hex(&to.color_17),
			53..=63 => from_hex(&to.color_18),
			64..=77 => from_hex(&to.color_19),
			78..=94 => from_hex(&to.color_20),
			95..=115 => from_hex(&to.color_21),
			116..=140 => from_hex(&to.color_22),
			141..=171 => from_hex(&to.color_23),
			172..=209 => from_hex(&to.color_24),
			210..=255 => from_hex(&to.color_25),
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index + 0] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
