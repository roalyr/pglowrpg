use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_unit_systems::coords::Index;
use std::f32::consts::PI;

const HALF_SQUARE_255: f32 = 32512.5;

fn gradient_both(size: u32) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size as usize * size as usize];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = (HALF_SQUARE_255
				* ((PI * (2.0 / (size as f32) * (y as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

fn grad_none(size: u32) -> Vec<f32> {
	vec![cg::VAL_255_F32; size as usize * size as usize]
}

fn grad_south(size: u32) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size as usize * size as usize];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, y)] = (HALF_SQUARE_255
				* ((PI * (1.3 / (size as f32) * (y as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

fn gradient_north(size: u32) -> Vec<f32> {
	let index = Index { map_size: size };
	let mut array = vec![cg::ZERO_F32; size as usize * size as usize];
	for y in 0..size {
		for x in 0..size {
			array[index.get(x, size - y - 1)] = (HALF_SQUARE_255
				* ((PI * (1.3 / (size as f32) * (y as f32) - 1.0)).cos() + 1.0))
				.sqrt();
		}
	}
	array
}

//WRAPPER
pub fn get(
	size: u32,
	grad_type: cw::TempGrad,
) -> Vec<f32> {
	match grad_type {
		cw::TempGrad::North => gradient_north(size),
		cw::TempGrad::South => grad_south(size),
		cw::TempGrad::Both => gradient_both(size),
		cw::TempGrad::Neither => grad_none(size),
	}
}
