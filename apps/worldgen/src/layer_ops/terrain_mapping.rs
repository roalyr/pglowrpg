use crate::array_ops;
use crate::array_ops::noise_maps;
use crate::array_ops::noise_maps::NoiseMode;
use game_data_codec::LayerPack;
//use constants::world as cw;

pub fn get(lp: &mut LayerPack) {
	let mut array = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		lp.wi.topog_concentrator,
		lp.wi.topog_scope,
		lp.wi.topog_continuity,
		lp.wi.seed,
	);
	//Filter from the bottom.
	array_ops::modify::level(&mut array, lp.wi.topog_filter);
	//Specific erosion and additional noise.
	erode(
		&mut array,
		lp.wi.noisemap_size,
		lp.wi.topog_noise_size,
		lp.wi.topog_noise_weight,
		lp.wi.topog_erosion_factor,
		lp.wi.seed,
	);
	//Stretch the map between 0 and 256.
	array_ops::modify::normalize(&mut array);
	//Resize to world map size.
	let topog_map = array_ops::interpolate::mitchell(
		array,
		lp.wi.noisemap_size,
		lp.wi.map_size,
	);
	//Write the map.
	for ind in 0..lp.layer_vec_len {
		lp.topography
			.write(topog_map[ind] as u16, lp.topography.TERRAIN, ind)
	}
}

//Terrain-specific array operations.
//This "erode" function is yet to be improved for better hi-freq noise.
fn erode(
	array: &mut Vec<f32>,
	size: usize,
	elevation_noise_size: f32,
	elevation_noise_weight: f32,
	erosion_factor: f32,
	seed: usize,
) {
	//TODO: improve fine noise
	let nm1 = noise_maps::get(size, elevation_noise_size, seed, NoiseMode::Multi);
	let nm2 = noise_maps::get(
		size,
		elevation_noise_size * 0.7,
		seed + 100,
		NoiseMode::Multi,
	);
	let nm3 = noise_maps::get(
		size,
		elevation_noise_size * 0.3,
		seed + 1000,
		NoiseMode::Multi,
	);
	for ind in 0..array.len() {
		let noise = (0.2 * nm1[ind] + 0.4 * nm2[ind] + 0.6 * nm3[ind])
			* elevation_noise_weight
			+ (1.0 - elevation_noise_weight);
		array[ind] = (array[ind] * noise).powf(erosion_factor);
	}
}
