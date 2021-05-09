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
		// Attempt to sync i, j which are used in worldgen, and x, y
		// coordinates in the game... But, well, I think it turned out
		// to be y, x instead.
		// TODO: move in-game coordinate interpreting function here and
		// make them synchronized.
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
