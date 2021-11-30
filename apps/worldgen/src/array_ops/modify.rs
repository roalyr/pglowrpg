use lib_constants::generic as cg;

//FILTER
//Set min and max values, thresholds.
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
//Decrease every cell by given amount, threshold is 0.
pub fn level(
	array: &mut Vec<f32>,
	low_v: u32,
) {
	let size = array.len();
	for cell_v in array.iter_mut().take(size) {
		*cell_v -= low_v as f32;
		if *cell_v < cg::ZERO_F32 {
			*cell_v = cg::ZERO_F32;
		}
	}
}

//NORMALIZE
//Stretch the map between the 0 and maximum value.
pub fn normalize(array: &mut Vec<f32>) {
	let size = array.len();
	let mut max_v = cg::ZERO_F32;
	for cell_v in array.iter_mut().take(size) {
		if *cell_v > max_v {
			max_v = *cell_v;
		}
	}
	let k = cg::VAL_255_F32 / max_v;
	for cell_v in array.iter_mut().take(size) {
		*cell_v *= k;
	}
}
