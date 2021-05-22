use io_ops::readron::palettes::temperature;

use crate::from_hex;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let te: temperature::Stuff = temperature::get();

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v {
			0..=10 => from_hex(&te.color_0),
			11..=20 => from_hex(&te.color_1),
			21..=30 => from_hex(&te.color_2),
			31..=40 => from_hex(&te.color_3),
			41..=50 => from_hex(&te.color_4),
			51..=60 => from_hex(&te.color_5),
			61..=70 => from_hex(&te.color_6),
			71..=80 => from_hex(&te.color_7),
			81..=90 => from_hex(&te.color_8),
			91..=100 => from_hex(&te.color_9),
			101..=110 => from_hex(&te.color_10),
			111..=120 => from_hex(&te.color_11),
			121..=130 => from_hex(&te.color_12),
			131..=140 => from_hex(&te.color_13),
			141..=150 => from_hex(&te.color_14),
			151..=160 => from_hex(&te.color_15),
			161..=170 => from_hex(&te.color_16),
			171..=180 => from_hex(&te.color_17),
			181..=190 => from_hex(&te.color_18),
			191..=200 => from_hex(&te.color_19),
			201..=210 => from_hex(&te.color_20),
			211..=220 => from_hex(&te.color_21),
			221..=230 => from_hex(&te.color_22),
			231..=240 => from_hex(&te.color_23),
			241..=250 => from_hex(&te.color_24),
			251..=255 => from_hex(&te.color_25),
		};

		let index = i * 4;

		idat[index + 3] = argb[0];
		idat[index] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
