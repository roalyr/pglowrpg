pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		idat[index + 3] = 255;
		idat[index] = *cell_v;
		idat[index + 1] = *cell_v;
		idat[index + 2] = *cell_v;
	}
	idat
}

pub fn get_binary(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		if *cell_v != 0 {
			idat[index + 3] = 255;
			idat[index] = 255;
			idat[index + 1] = 255;
			idat[index + 2] = 255;
		}
	}
	idat
}

pub fn get_binary_inverse(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		if *cell_v != 0 {
			idat[index + 3] = 0;
			idat[index] = 0;
			idat[index + 1] = 0;
			idat[index + 2] = 0;
		} else {
			idat[index + 3] = 255;
			idat[index] = 255;
			idat[index + 1] = 255;
			idat[index + 2] = 255;
		}
	}
	idat
}

pub fn get_curved(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	let f = 0.5;

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		idat[index + 3] = 255;
		idat[index] = (255.0 * ((*cell_v as f32 / 255.0).powf(f))) as u8;
		idat[index + 1] = (255.0 * ((*cell_v as f32 / 255.0).powf(f))) as u8;
		idat[index + 2] = (255.0 * ((*cell_v as f32 / 255.0).powf(f))) as u8;
	}
	idat
}

pub fn get_inverse(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let index = i * 4;

		idat[index + 3] = 255;
		idat[index] = 255 - *cell_v;
		idat[index + 1] = 255 - *cell_v;
		idat[index + 2] = 255 - *cell_v;
	}
	idat
}
