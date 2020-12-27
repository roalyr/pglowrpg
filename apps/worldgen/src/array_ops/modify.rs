//FILTER
pub fn filter(
	array: &mut Vec<f32>,
	low_v: f32,
	high_v: f32,
) {
	let size = array.len();
	for cell_v in array.iter_mut().take(size) {
		if *cell_v < low_v {
			*cell_v = low_v;
		} else if *cell_v > high_v {
			*cell_v = high_v;
		}
	}
}

//LEVEL
pub fn level(
	array: &mut Vec<f32>,
	low_v: usize,
) {
	let size = array.len();
	for cell_v in array.iter_mut().take(size) {
		*cell_v -= low_v as f32;
		if *cell_v < 0.0 {
			*cell_v = 0.0;
		}
	}
}

//NORMALIZE
pub fn normalize(array: &mut Vec<f32>) {
	let size = array.len();
	let mut max_v = 0.0;
	for cell_v in array.iter_mut().take(size) {
		if *cell_v > max_v {
			max_v = *cell_v;
		}
	}
	let k = 255.0 / max_v;
	for cell_v in array.iter_mut().take(size) {
		*cell_v *= k;
	}
}
