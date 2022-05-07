use crate::array_ops;
use crate::array_ops::noise_maps::NoiseMode;
use lib_constants::generic as cg;
use lib_game_data_codec::LayerPack;

pub fn get(lp: &mut LayerPack) {
	temperature(lp);
	rainfall(lp);
}

////////////////////////////////////////////////////////////////////////////////
// TEMPERATURE
fn temperature(lp: &mut LayerPack) {
	let mut array = vec![cg::ZERO_F32; lp.noisemap_vec_len as usize];
	//Main gradient according to pole location.
	let array_grad =
		array_ops::gradients::get(lp.wi.noisemap_size, lp.wi.temp_mode);
	//Multi-freq noise layers.
	let array_noise1 = array_ops::noise_maps::get(
		lp.wi.noisemap_size,
		lp.wi.temp_noise_size,
		lp.wi.seed + 816624830,
		NoiseMode::Multi,
	);
	let array_noise2 = array_ops::noise_maps::get(
		lp.wi.noisemap_size,
		lp.wi.temp_noise_size * 0.75,
		lp.wi.seed + 978563541,
		NoiseMode::Perlin,
	);
	//Additional noise to compensate for polar regions gradient
	//values close to 0.
	let array_noise_polar = array_ops::noise_maps::get(
		lp.wi.noisemap_size,
		lp.wi.temp_noise_size,
		lp.wi.seed + 642578358,
		NoiseMode::Multi,
	);
	//Combine all the maps.
	for (index, cell_v) in array
		.iter_mut()
		.enumerate()
		.take(lp.noisemap_vec_len as usize)
	{
		let grad_rel = array_grad[index] / cg::VAL_255_F32;
		*cell_v = array_grad[index]
			* (1.0 - lp.wi.temp_noise_weight)
			* (grad_rel + array_noise_polar[index] * (1.0 - grad_rel))
			+ (array_noise1[index] + array_noise2[index])
				* cg::VAL_127_F32
				* lp.wi.temp_noise_weight
				* grad_rel;
		if *cell_v < 0.0 {
			*cell_v = 0.0;
		}
	}
	//Stretch between 0 and 256.
	array_ops::modify::normalize(&mut array);
	//Scale up.
	let temp_map = array_ops::interpolate::mitchell(
		array,
		lp.wi.noisemap_size,
		lp.wi.map_size,
	);
	for (ind, cell_v) in temp_map.iter().enumerate() {
		lp.climate
			.write(*cell_v as u16, lp.climate.TEMPERATURE, ind)
	}
}

////////////////////////////////////////////////////////////////////////////////
// RAINFALL
fn rainfall(lp: &mut LayerPack) {
	let mut array = vec![cg::ZERO_F32; lp.noisemap_vec_len as usize];
	//Gradient to account for polar regions being dryer.
	let array_grad =
		array_ops::gradients::get(lp.wi.noisemap_size, lp.wi.temp_mode);
	//Noise maps that will be subtracted and added to base value
	//of 128 to make rainfall and desert zones.
	//Proportional to landmass size (topog_scope).
	let array_ds1 = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		0.1,
		lp.wi.topog_scope / 1.5,
		0.5,
		lp.wi.seed + 978524325,
	);
	let array_ds2 = array_ops::diamond_square::get(
		lp.wi.noisemap_size,
		0.0,
		lp.wi.topog_scope / 1.5,
		0.5,
		lp.wi.seed + 978457544,
	);
	//Flat noise.
	let array_noise = array_ops::noise_maps::get(
		lp.wi.noisemap_size,
		lp.wi.rain_noise_size,
		lp.wi.seed + 3625946807,
		NoiseMode::Multi,
	);
	//Combining all the noise maps together.
	for (index, cell_v) in array
		.iter_mut()
		.enumerate()
		.take(lp.noisemap_vec_len as usize)
	{
		*cell_v = (cg::VAL_127_F32 - array_ds1[index] + array_ds2[index])
			* array_grad[index]
			/ cg::VAL_255_F32
			* (1.0 - lp.wi.rain_noise_weight)
			+ (array_noise[index] * cg::VAL_255_F32 * lp.wi.rain_noise_weight);
		if *cell_v < 0.0 {
			*cell_v = 0.0;
		}
	}
	//Stretch between 0 and 256.
	array_ops::modify::normalize(&mut array);
	//Scale up.
	let rain_map = array_ops::interpolate::mitchell(
		array,
		lp.wi.noisemap_size,
		lp.wi.map_size,
	);

	for (ind, cell_v) in rain_map.iter().enumerate() {
		lp.climate.write(*cell_v as u16, lp.climate.RAINFALL, ind)
	}
}
