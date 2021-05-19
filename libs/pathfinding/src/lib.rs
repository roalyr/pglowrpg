use pathfinding::prelude::astar;
use unit_systems::coords::Index;

// Weights of moving orthogonally and diagonally.
// If we imagine a square with a size of a side being 1,
// then the diagonal will be 2^(1/2) ~ 1.41.
// Here it is used as integer, so it is multiplied by 100.
const W_ORT: usize = 100; // Aka sqrt from 10000.
const W_DIA: usize = 141; // Aka sqrt from 20000.

pub struct DirVector {
	pub x0: usize,
	pub y0: usize,
	pub x1: usize,
	pub y1: usize,
	pub r: usize,
	pub hit: bool,
	pub path_heuristic: usize,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub usize, pub usize);

impl Pos {
	// Return approximate distance (as per source of information).
	#[inline] // Is it necessary?
	fn distance(
		&self,
		other: &Pos,
	) -> usize {
		((self.0 as i32 - other.0 as i32).abs()
			+ (self.1 as i32 - other.1 as i32).abs()) as usize
	}

	// Returns a queue of neighbors adjacent, weighted.
	#[allow(clippy::ptr_arg)]
	#[inline] // Is it necessary?
	fn neighbors<T>(
		&self,
		map: &Vec<T>,
		map_size: usize,
		index: Index,
		step: usize,
		goal: Pos,
		diag_flag: bool,
	) -> Vec<(Pos, usize)>
	where
		T: Into<usize> + Copy + Clone,
	{
		let &Pos(x, y) = self;
		let mut vect = Vec::new();
		// Diagonal directions. bounded, if enabled.
		if diag_flag {
			let i = x + step;
			let j = y + step;
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;
				let ind = index.get(i, j);
				let map_val = dir_weight * map[ind].into();
				vect.push((Pos(i, j), map_val))
			};
			let i = x.saturating_sub(step);
			let j = y.saturating_sub(step);
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;
				let ind = index.get(i, j);
				let map_val = dir_weight * map[ind].into();
				vect.push((Pos(i, j), map_val))
			};
			let i = x + step;
			let j = y.saturating_sub(step);
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;
				let ind = index.get(i, j);
				let map_val = dir_weight * map[ind].into();
				vect.push((Pos(i, j), map_val))
			};
			let i = x.saturating_sub(step);
			let j = y + step;
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;
				let ind = index.get(i, j);
				let map_val = dir_weight * map[ind].into();
				vect.push((Pos(i, j), map_val))
			};
		}
		// Orthogonal directions. bounded.
		let i = x;
		let j = y + step;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;
			let ind = index.get(i, j);
			let map_val = dir_weight * map[ind].into();
			vect.push((Pos(i, j), map_val))
		};
		let i = x;
		let j = y.saturating_sub(step);
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;
			let ind = index.get(i, j);
			let map_val = dir_weight * map[ind].into();
			vect.push((Pos(i, j), map_val))
		};
		let i = x + step;
		let j = y;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;
			let ind = index.get(i, j);
			let map_val = dir_weight * map[ind].into();
			vect.push((Pos(i, j), map_val))
		};
		let i = x.saturating_sub(step);
		let j = y;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;
			let ind = index.get(i, j);
			let map_val = dir_weight * map[ind].into();
			vect.push((Pos(i, j), map_val))
		};
		// Check to force pathfinding to converge when step is > 1.
		// Distance < step length. A bit longer for safe measure.
		if self.distance(&goal) < step * 2 {
			vect.push((goal, 0))
		}
		vect
	}
}

#[inline] // Is it necessary?
fn within_bounds(
	i: usize,
	j: usize,
	size: usize,
) -> bool {
	(i < size) && (j < size)
}

#[allow(clippy::ptr_arg)]
pub fn make<T>(
	v: &DirVector, // Directional vector (see above).
	map: &Vec<T>,
	map_size: usize,
	diag_flag: bool, // Whether to go 8 directions or 4 cardinal directions.
	step: usize,     // How many cells to skip on map.
) -> (std::vec::Vec<Pos>, usize)
where
	T: Into<usize> + Copy + Clone,
{
	let goal: Pos = Pos(v.x1, v.y1);
	let start: Pos = Pos(v.x0, v.y0);
	let index = Index { map_size };
	let path = astar(
		&start,
		|p| p.neighbors(map, map_size, index, step, goal, diag_flag),
		|p| p.distance(&goal) * v.path_heuristic,
		|p| *p == goal,
	);
	// Throw an error if result is not successful.
	match path {
		Some(x) => x,
		None => {
			panic!(
				"ERROR: in pathfinding. Start: {:?}, goal: {:?}",
				start, goal
			)
		}
	}
}
