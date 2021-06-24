use noise::*;
use unit_systems::coords::Index;

#[derive(Copy, Clone)]
pub enum NoiseMode {
	Multi,
	Perlin,
}

//Returns a single point, not a map.
pub fn point_multi(
	noise_size: f32,
	seed: u32,
	i: usize,
	j: usize,
) -> f32 {
	let noise = BasicMulti::new();
	let rotate = RotatePoint::new(&noise)
		.set_x_angle(22.0)
		.set_y_angle(22.0)
		.set_z_angle(45.0);
	let abs = Abs::new(&rotate);
	abs.get([
		(i as f64) * f64::from(noise_size),
		(j as f64) * f64::from(noise_size),
		((seed + 376429876) as f64),
	]) as f32
}

//Returns a map.
fn multi(
	size: u32,
	noise_size: f32,
	seed: u32,
) -> Vec<f32> {
	let index = Index { map_size: size };
	let noise = BasicMulti::new();
	let rotate = RotatePoint::new(&noise)
		.set_x_angle(22.0)
		.set_y_angle(22.0)
		.set_z_angle(45.0);
	let abs = Abs::new(&rotate);
	let mut array = vec![0.0; size as usize * size as usize];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = abs.get([
				(x as f64) * f64::from(noise_size),
				(y as f64) * f64::from(noise_size),
				((seed + 980966862) as f64),
			]) as f32;
		}
	}
	array
}

//Returns a map.
fn perlin(
	size: u32,
	noise_size: f32,
	seed: u32,
) -> Vec<f32> {
	let index = Index { map_size: size };
	let noise = Perlin::new();
	let rotate = RotatePoint::new(&noise)
		.set_x_angle(22.0)
		.set_y_angle(22.0)
		.set_z_angle(45.0);
	let abs = Abs::new(&rotate);
	let mut array = vec![0.0; size as usize * size as usize];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = abs.get([
				(x as f64) * f64::from(noise_size),
				(y as f64) * f64::from(noise_size),
				((seed + 282645289) as f64),
			]) as f32;
		}
	}
	array
}

//WRAPPER
pub fn get(
	size: u32,
	noise_size: f32,
	seed: u32,
	noise_type: NoiseMode,
) -> Vec<f32> {
	match noise_type {
		NoiseMode::Multi => multi(size, noise_size, seed),
		NoiseMode::Perlin => perlin(size, noise_size, seed),
	}
}
