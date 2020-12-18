use crate::array_ops;
use crate::array_ops::noise_maps::NoiseMode::*;
use codec::LayerPack;

pub fn get(lp: &mut LayerPack) {
  temperature(lp);
  rainfall(lp);
}

//▒▒▒▒▒▒▒▒▒▒ TEMPERATURE ▒▒▒▒▒▒▒▒▒▒▒
fn temperature(lp: &mut LayerPack) {
  let mut array = vec![0.0; lp.noisemap_vec_len];

  let array_grad =
    array_ops::gradients::get(lp.wi.noisemap_size, lp.wi.temp_mode);

  let array_noise1 = array_ops::noise_maps::get(
    lp.wi.noisemap_size,
    lp.wi.temp_noise_size,
    lp.wi.seed + 1,
    Multi,
  );

  let array_noise2 = array_ops::noise_maps::get(
    lp.wi.noisemap_size,
    lp.wi.temp_noise_size * 0.75,
    lp.wi.seed + 10,
    Perlin,
  );

  let array_noise_polar = array_ops::noise_maps::get(
    lp.wi.noisemap_size,
    lp.wi.temp_noise_size,
    lp.wi.seed + 100,
    Multi,
  );

  for (index, cell_v) in array.iter_mut().enumerate().take(lp.noisemap_vec_len)
  {
    let grad_rel = array_grad[index] / 255.0;
    *cell_v = array_grad[index]
      * (1.0 - lp.wi.temp_noise_weight)
      * (grad_rel + array_noise_polar[index] * (1.0 - grad_rel))
      + (array_noise1[index] + array_noise2[index])
        * 127.0
        * lp.wi.temp_noise_weight
        * grad_rel;

    if *cell_v < 0.0 {
      *cell_v = 0.0;
    }
  }

  array_ops::modify::normalize(&mut array);

  let temp_map = array_ops::interpolate::mitchell(
    array,
    lp.wi.noisemap_size,
    lp.wi.map_size,
  );

  for index in 0..lp.layer_vec_len {
    lp.climate
      .write(temp_map[index] as u16, lp.climate.TEMPERATURE, index)
  }
}

//▒▒▒▒▒▒▒▒▒▒▒ RAINFALL ▒▒▒▒▒▒▒▒▒▒▒▒▒
fn rainfall(lp: &mut LayerPack) {
  let mut array = vec![0.0; lp.noisemap_vec_len];

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

  for (index, cell_v) in array.iter_mut().enumerate().take(lp.noisemap_vec_len)
  {
    *cell_v = (127.0 - array_ds1[index] + array_ds2[index]) * array_grad[index]
      / 255.0
      * (1.0 - lp.wi.rain_noise_weight)
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

  for index in 0..lp.layer_vec_len {
    lp.climate
      .write(rain_map[index] as u16, lp.climate.RAINFALL, index)
  }
}
