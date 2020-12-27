use crate::array_ops;
use crate::array_ops::noise_maps;
use crate::array_ops::noise_maps::NoiseMode::*;
use codec::LayerPack;

pub fn get(lp: &mut LayerPack) {
	let mut array = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		lp.wi.topog_concentrator,
		lp.wi.topog_scope,
		lp.wi.topog_continuity,
		lp.wi.seed,
	);
	array_ops::modify::level(&mut array, lp.wi.topog_filter);
	erode(
		&mut array,
		lp.wi.noisemap_size,
		lp.wi.topog_noise_size,
		lp.wi.topog_noise_weight,
		lp.wi.topog_erosion_factor,
		lp.wi.seed,
	);
	array_ops::modify::normalize(&mut array);
	let topog_map = array_ops::interpolate::mitchell(
		array,
		lp.wi.noisemap_size,
		lp.wi.map_size,
	);
	for index in 0..lp.layer_vec_len {
		lp.topography
			.write(topog_map[index] as u16, lp.topography.TERRAIN, index)
	}
}

fn erode(
	array: &mut Vec<f32>,
	size: usize,
	elevation_noise_size: f32,
	elevation_noise_weight: f32,
	erosion_factor: f32,
	seed: usize,
) {
	let nm1 = noise_maps::get(size, elevation_noise_size, seed, Multi);
	let nm2 =
		noise_maps::get(size, elevation_noise_size * 0.7, seed + 100, Multi);
	let nm3 =
		noise_maps::get(size, elevation_noise_size * 0.3, seed + 1000, Multi);
	for index in 0..array.len() {
		let noise = (0.2 * nm1[index] + 0.4 * nm2[index] + 0.6 * nm3[index])
			* elevation_noise_weight
			+ (1.0 - elevation_noise_weight);
		array[index] = (array[index] * noise).powf(erosion_factor);
	}
}
