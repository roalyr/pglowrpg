use crate::array_ops;
use crate::array_ops::noise_maps::NoiseMode::*;
use crate::array_ops::translate;
use crate::worldgen;

const NO_VAL_F: f32 = 0.0;

pub fn get(lp: &mut worldgen::LayerPack) {
	lp.ra.layer_name =
		["rainfall_", &{ lp.wi.seed.to_string() }].concat();

	let mut array = vec![NO_VAL_F; lp.noise_vec_len];

	let array_grad =
		array_ops::gradients::get(lp.wi.noisemap_size, lp.wi.temp_mode);

	let array_ds1 = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		0.1,
		lp.wi.topog_scope / 1.5,
		0.5,
		lp.wi.seed + 10,
	);

	let array_ds2 = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		0.0,
		lp.wi.topog_scope / 1.5,
		0.5,
		lp.wi.seed + 100,
	);

	let array_noise = array_ops::noise_maps::get(
		lp.wi.noisemap_size,
		lp.wi.rain_noise_size,
		lp.wi.seed + 1000,
		Multi,
	);

	for (index, cell_v) in
		array.iter_mut().enumerate().take(lp.noise_vec_len)
	{
		*cell_v = (127.0 - array_ds1[index] + array_ds2[index])
			* array_grad[index]
			/ 255.0 * (1.0 - lp.wi.rain_noise_weight)
			+ (array_noise[index] * 255.0 * lp.wi.rain_noise_weight);

		if *cell_v < 0.0 {
			*cell_v = 0.0;
		}
	}

	array_ops::modify::normalize(&mut array);

	let rain_map = array_ops::interpolate::mitchell(
		array,
		lp.wi.noisemap_size,
		lp.wi.map_size,
	);

	lp.ra.array_map = translate::encode(rain_map);
}
