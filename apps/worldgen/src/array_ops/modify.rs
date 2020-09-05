use crate::array_ops::noise_maps;
use crate::array_ops::noise_maps::NoiseMode::*;

pub const PI: f32 = std::f64::consts::PI as f32;

pub fn erode(
	array: &mut Vec<f32>,
	size: usize,
	elevation_noise_size: f32,
	elevation_noise_weight: f32,
	erosion_factor: f32,
	seed: usize,
) {
	let nm1 =
		noise_maps::get(size, elevation_noise_size, seed, Multi);

	let nm2 = noise_maps::get(
		size,
		elevation_noise_size * 0.7,
		seed + 100,
		Multi,
	);

	let nm3 = noise_maps::get(
		size,
		elevation_noise_size * 0.3,
		seed + 1000,
		Multi,
	);

	for index in 0..array.len() {
		//-----------------------------------------
		let noise =
			(0.2 * nm1[index] + 0.4 * nm2[index] + 0.6 * nm3[index])
				* elevation_noise_weight
				+ (1.0 - elevation_noise_weight);

		array[index] = (array[index] * noise).powf(erosion_factor);
		//-----------------------------------------
	}
}

//COASTLINE
pub fn restore_coastline(
	array: &mut Vec<f32>,
	array_coastline: Vec<f32>,
) {
	let size = array.len();
	for (index, cell_v) in array.iter_mut().enumerate().take(size) {
		if (array_coastline[index] > 1.0) && (*cell_v < 1.0) {
			*cell_v = 1.0;
		}
	}
}

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
