use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Index {
	pub map_size: usize,
}

impl Index {
	#[inline]
	pub fn get(
		self,
		x: usize,
		y: usize,
	) -> usize {
		let result = (x * self.map_size).checked_add(y);
		match result {
			Some(x) => x,
			None => {
				let saturated = (x * self.map_size).saturating_add(y);
				println!("ERROR: overflow at index {}", saturated);
				println!("x:{}, y:{}, map size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
}
