pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		//-----------------------------------------
		let index = i * 4;

		if *cell_v != 0 {
			idat[index + 3] = 255;
			idat[index + 0] =
				prng::get(0.0, 255.0, 1, *cell_v as usize) as u8;

			idat[index + 1] =
				prng::get(0.0, 255.0, 2, *cell_v as usize) as u8;

			idat[index + 2] =
				prng::get(0.0, 255.0, 3, *cell_v as usize) as u8;
		} else {
			idat[index + 3] = 255;
			idat[index + 0] = 0;
			idat[index + 1] = 0;
			idat[index + 2] = 0;
		}
		//-----------------------------------------
	}
	idat
}
