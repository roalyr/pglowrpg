use crate::array_ops;
use crate::array_ops::noise_maps::NoiseMode::*;
use crate::array_ops::translate;
use crate::worldgen;

const NO_VAL: u8 = 0;

pub fn get(lp: &mut worldgen::LayerPack) {
	let topog_map = translate::decode(&lp.to.array_map);

	lp.rto.layer_name =
		["river_topog_", &{ lp.wi.seed.to_string() }].concat();

	let mut rtopog_map = vec![NO_VAL; lp.layer_vec_len];

	let mut array_noise1 = array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size1,
		lp.wi.seed,
		Multi,
	);

	let array_noise2 = array_ops::noise_maps::get(
		lp.wi.map_size,
		lp.wi.river_noise_size2 * 2.0,
		lp.wi.seed + 1000,
		Perlin,
	);

	for (index, cell_v) in
		array_noise1.iter_mut().enumerate().take(lp.layer_vec_len)
	{
		*cell_v = (*cell_v as f32)
			* 255.0 * (1.0 - lp.wi.river_noise_blend)
			+ array_noise2[index] * 255.0 * lp.wi.river_noise_blend;
	}

	//array_ops::modify::normalize(
	//&mut array_noise1);

	for (index, cell_v) in
		rtopog_map.iter_mut().enumerate().take(lp.layer_vec_len)
	{
		*cell_v = (topog_map[index] as f32
			* (1.0 - lp.wi.river_noise_weight)) as u8
			+ ((array_noise1[index] * lp.wi.river_noise_weight) as u8);
	}
	lp.rto.array_map = translate::encode(rtopog_map);
}
