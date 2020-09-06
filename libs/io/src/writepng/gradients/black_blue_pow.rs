pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let f = 0.3;

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		idat[index + 3] = 255;
		idat[index + 0] = 0;
		idat[index + 1] = 0;
		idat[index + 2] =
			(255.0 * ((*cell_v as f32 / 255.0).powf(f))) as u8;
	}
	idat
}
