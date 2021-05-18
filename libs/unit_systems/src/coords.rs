use serde::{Deserialize, Serialize};

#[derive(Clone, Copy, Serialize, Deserialize, Debug)]
pub struct Index {
	pub map_size: usize,
}

impl Index {
	#[inline]
	pub fn ind(
		self,
		i: usize,
		j: usize,
	) -> usize {
		// Do not change anything here.
		let x = (self.map_size - 1).saturating_sub(i);
		let y = j;
		let result = (x * self.map_size).checked_add(y);
		match result {
			Some(x) => x,
			None => {
				let saturated = (x * self.map_size).saturating_add(y);
				println!("overflow at index {}", saturated);
				println!("└ x:{}, y:{}, size: {}", x, y, self.map_size);
				panic!();
			}
		}
	}
}
