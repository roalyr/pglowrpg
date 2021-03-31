use crate::readron::palettes::region_size;

use crate::writepng::from_hex;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let rs: region_size::Stuff = region_size::get();

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
			13 => from_hex(&rs.color_13),
			14 => from_hex(&rs.color_14),
			15 => from_hex(&rs.color_15),
			16 => from_hex(&rs.color_16),
			17 => from_hex(&rs.color_17),
			18 => from_hex(&rs.color_18),
			19 => from_hex(&rs.color_19),
			20 => from_hex(&rs.color_20),
			21 => from_hex(&rs.color_21),
			22 => from_hex(&rs.color_22),
			23 => from_hex(&rs.color_23),
			24 => from_hex(&rs.color_24),
			25 => from_hex(&rs.color_25),
			26 => from_hex(&rs.color_26),
			27 => from_hex(&rs.color_27),
			28 => from_hex(&rs.color_28),
			_ => from_hex(&rs.color_100),
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
