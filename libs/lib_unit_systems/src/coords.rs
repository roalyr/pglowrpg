// [Revision: v0.2] [Path: libs/lib_unit_systems/src/coords.rs] [Date: 2025-12-13]
// Description: 2D coordinate indexing with overflow protection

use serde::{Deserialize, Serialize};

/// Simple 2D coordinate.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub struct Coord {
	pub x: u32,
	pub y: u32,
}

impl Coord {
	pub fn new(x: u32, y: u32) -> Self {
		Self { x, y }
	}

	/// Calculate distance to another coordinate.
	pub fn distance_to(&self, other: &Coord) -> f32 {
		let dx = (self.x as f32) - (other.x as f32);
		let dy = (self.y as f32) - (other.y as f32);
		(dx * dx + dy * dy).sqrt()
	}
}

/// Index calculator for 2D grid coordinates.
/// Converts (x, y) coordinates to linear array indices.
#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Index {
	pub map_size: u32,
}

impl Index {
	pub fn get(
		self,
		x: u32,
		y: u32,
	) -> usize {
		let result = (y * self.map_size).checked_add(x);
		match result {
			Some(x) => x as usize,
			None => {
				let saturated = (y * self.map_size).saturating_add(x);
				println!("ERROR: overflow at index {}.", saturated);
				println!("x:{}, y:{}, map size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
	pub fn get_mirrored_h(
		self,
		x: u32,
		y: u32,
	) -> usize {
		let result = ((self.map_size - y - 1) * self.map_size).checked_add(x);
		match result {
			Some(x) => x as usize,
			None => {
				let saturated =
					((self.map_size - y - 1) * self.map_size).saturating_add(x);
				println!(
					"ERROR: overflow at index {} while reading mirrored horizontally.",
					saturated
				);
				println!("x:{}, y:{}, map size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
	pub fn get_mirrored_v(
		self,
		x: u32,
		y: u32,
	) -> usize {
		let result = (y * self.map_size).checked_add(self.map_size - x - 1);
		match result {
			Some(x) => x as usize,
			None => {
				let saturated =
					(y * self.map_size).saturating_add(self.map_size - x - 1);
				println!(
					"ERROR: overflow at index {} while reading mirrored vertically.",
					saturated
				);
				println!("x:{}, y:{}, map size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
	pub fn get_mirrored_hv(
		self,
		x: u32,
		y: u32,
	) -> usize {
		let result = ((self.map_size - y - 1) * self.map_size)
			.checked_add(self.map_size - x - 1);
		match result {
			Some(x) => x as usize,
			None => {
				let saturated = ((self.map_size - y - 1) * self.map_size)
					.saturating_add(self.map_size - x - 1);
				println!("ERROR: overflow at index {} while reading mirrored (horizontally and vertically).", saturated);
				println!("x:{}, y:{}, map size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
}
