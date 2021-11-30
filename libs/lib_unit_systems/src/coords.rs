use serde::{Deserialize, Serialize};

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
