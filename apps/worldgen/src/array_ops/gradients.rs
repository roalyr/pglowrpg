use constants_world::*;
use coords::Index;
use std::f32::consts::PI;

const SQUARE_255: f32 = 32512.5;

//GRADIENT BOTH
fn grad_both(size: usize) -> Vec<f32> {
  let xy = Index { map_size: size };
  let mut array = vec![0.0; size * size];
  for i in 0..size {
    for j in 0..size {
      array[xy.ind(i, j)] = (SQUARE_255
        * ((PI * (2.0 / (size as f32) * (i as f32) - 1.0)).cos() + 1.0))
        .sqrt();
    }
  }
  array
}

//GRADIENT NONE
fn grad_none(size: usize) -> Vec<f32> {
  vec![255.0; size * size]
}

//GRADIENT A
fn grad_a(size: usize) -> Vec<f32> {
  let xy = Index { map_size: size };
  let mut array = vec![0.0; size * size];
  for i in 0..size {
    for j in 0..size {
      array[xy.ind(i, j)] = (SQUARE_255
        * ((PI * (1.3 / (size as f32) * (i as f32) - 1.0)).cos() + 1.0))
        .sqrt();
    }
  }
  array
}

//GRADIENT B
fn grad_b(size: usize) -> Vec<f32> {
  let xy = Index { map_size: size };
  let mut array = vec![0.0; size * size];
  for i in 0..size {
    for j in 0..size {
      array[xy.ind(size - i - 1, j)] = (SQUARE_255
        * ((PI * (1.3 / (size as f32) * (i as f32) - 1.0)).cos() + 1.0))
        .sqrt();
    }
  }
  array
}

//WRAPPER
pub fn get(
  size: usize,
  grad_type: TempGrad,
) -> Vec<f32> {
  match grad_type {
    TempGrad::North => grad_b(size),
    TempGrad::South => grad_a(size),
    TempGrad::Both => grad_both(size),
    TempGrad::Neither => grad_none(size),
  }
}
