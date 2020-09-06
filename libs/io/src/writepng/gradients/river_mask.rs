use constants::world_constants::*;

pub fn get(array: &Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut idat = vec![0; size * 4];

	for (i, cell_v) in array.iter().enumerate().take(size) {
		let argb: Vec<u8> = match *cell_v as u16 {
			NO_RIVER => [255, 0, 0, 0].to_vec(),
			RIVER_SOURCE => [255, 0, 180, 0].to_vec(),
			RIVER_BODY => [255, 255, 255, 255].to_vec(),
			RIVER_END => [255, 255, 0, 0].to_vec(),
			RIVER_WATERFALL => [255, 80, 80, 255].to_vec(),
			RIVER_WATERFALLS_MUL => [255, 0, 255, 255].to_vec(),
			_ => [255, 255, 255, 0].to_vec(),
		};

		let index = i * 4;

		idat[index + 3] = 255;
		idat[index + 0] = argb[1];
		idat[index + 1] = argb[2];
		idat[index + 2] = argb[3];
	}
	idat
}
