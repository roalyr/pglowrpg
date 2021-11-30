use lib_io_ops::readron::palettes::biomes;

use crate::from_hex;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let bi: biomes::Stuff = biomes::get();

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v {
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
				println!(
					"ERROR: couldn't write BIOMES colored image, unexpected value."
				);
				panic!();
			}
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
