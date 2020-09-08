use crate::array_ops;
use crate::worldgen;

//use crate::array_ops::translate;

pub fn get(lp: &mut worldgen::LayerPack) {
	let mut array = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		lp.wi.topog_concentrator,
		lp.wi.topog_scope,
		lp.wi.topog_continuity,
		lp.wi.seed,
	);

	array_ops::modify::level(&mut array, lp.wi.topog_filter);

	array_ops::modify::erode(
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
		lp.topography.write(
			topog_map[index] as u16,
			lp.topography.TERRAIN,
			index,
		)
	}
}
