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
	seed: usize,
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
		(seed as f64),
	]) as f32
}

//Returns a map.
fn multi(
	size: usize,
	noise_size: f32,
	seed: usize,
) -> Vec<f32> {
	let xy = Index { map_size: size };
	let noise = BasicMulti::new();
	let rotate = RotatePoint::new(&noise)
		.set_x_angle(22.0)
		.set_y_angle(22.0)
		.set_z_angle(45.0);
	let abs = Abs::new(&rotate);
	let mut array = vec![0.0; size * size];
	for i in 0..size {
		for j in 0..size {
			array[xy.ind(i, j)] = abs.get([
				(i as f64) * f64::from(noise_size),
				(j as f64) * f64::from(noise_size),
				(seed as f64),
			]) as f32;
		}
	}
	array
}

//Returns a map.
fn perlin(
	size: usize,
	noise_size: f32,
	seed: usize,
) -> Vec<f32> {
	let xy = Index { map_size: size };
	let noise = Perlin::new();
	let rotate = RotatePoint::new(&noise)
		.set_x_angle(22.0)
		.set_y_angle(22.0)
		.set_z_angle(45.0);
	let abs = Abs::new(&rotate);
	let mut array = vec![0.0; size * size];
	for i in 0..size {
		for j in 0..size {
			array[xy.ind(i, j)] = abs.get([
				(i as f64) * f64::from(noise_size),
				(j as f64) * f64::from(noise_size),
				(seed as f64),
			]) as f32;
		}
	}
	array
}

//WRAPPER
pub fn get(
	size: usize,
	noise_size: f32,
	seed: usize,
	noise_type: NoiseMode,
) -> Vec<f32> {
	match noise_type {
		NoiseMode::Multi => multi(size, noise_size, seed),
		NoiseMode::Perlin => perlin(size, noise_size, seed),
	}
}
