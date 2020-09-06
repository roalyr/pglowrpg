pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		idat[index + 3] = 255;
		idat[index + 0] = *cell_v;
		idat[index + 1] = *cell_v;
		idat[index + 2] = *cell_v;
	}
	idat
}
