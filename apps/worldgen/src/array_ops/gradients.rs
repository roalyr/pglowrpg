use constants::generic as cg;
use constants::world as cw;
use std::f32::consts::PI;
use unit_systems::coords::Index;

const HALF_SQUARE_255: f32 = 32512.5;

fn gradient_both(size: usize) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size * size];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = (HALF_SQUARE_255
				* ((PI * (2.0 / (size as f32) * (x as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

fn grad_none(size: usize) -> Vec<f32> {
	vec![cg::VAL_255_F32; size * size]
}

fn grad_south(size: usize) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size * size];
	for y in 0..size {
		for x in 0..size {
			array[index.get(size - x - 1, y)] = (HALF_SQUARE_255
				* ((PI * (1.3 / (size as f32) * (x as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

fn gradient_north(size: usize) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size * size];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = (HALF_SQUARE_255
				* ((PI * (1.3 / (size as f32) * (x as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

//WRAPPER
pub fn get(
	size: usize,
	grad_type: cw::TempGrad,
) -> Vec<f32> {
	match grad_type {
		cw::TempGrad::North => gradient_north(size),
		cw::TempGrad::South => grad_south(size),
		cw::TempGrad::Both => gradient_both(size),
		cw::TempGrad::Neither => grad_none(size),
	}
}
