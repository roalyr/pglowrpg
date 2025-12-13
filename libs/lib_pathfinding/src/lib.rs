// [Revision: v0.1] [Path: libs/lib_pathfinding/src/lib.rs] [Date: 2025-12-13]
// Description: A* pathfinding wrapper for weighted grid traversal

use dep::pathfinding::prelude::astar;
use lib_unit_systems::coords::Index;

// Movement weights for orthogonal and diagonal directions.
// Unit square side = 1, diagonal = sqrt(2) ≈ 1.41.
// Multiplied by 100 for integer precision.
const W_ORT: u32 = 100;
const W_DIA: u32 = 141;

/// Directional vector for pathfinding from (x0,y0) to (x1,y1).
pub struct DirVector {
	pub x0: u32,
	pub y0: u32,
	pub x1: u32,
	pub y1: u32,
	pub r: u32,
	pub hit: bool,
	pub path_heuristic: u32,
}

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub struct Pos(pub u32, pub u32);

impl Pos {
	#[inline]
	fn distance(
		&self,
		other: &Pos,
	) -> u32 {
		((self.0 as i32 - other.0 as i32).abs()
			+ (self.1 as i32 - other.1 as i32).abs()) as u32
	}

	#[inline]
	fn neighbors<T>(
		&self,
		map: &Vec<T>,
		map_size: u32,
		index: Index,
		step: u32,
		goal: Pos,
		diag_flag: bool,
	) -> Vec<(Pos, u32)>
	where
		T: Into<u32> + Copy + Clone,
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

#[inline]
fn within_bounds(
	i: u32,
	j: u32,
	size: u32,
) -> bool {
	(i < size) && (j < size)
}

pub fn make<T>(
	v: &DirVector, // Directional vector (see above).
	map: &Vec<T>,
	map_size: u32,
	diag_flag: bool, // Whether to go 8 directions or 4 cardinal directions.
	step: u32,       // How many cells to skip on map.
) -> (std::vec::Vec<Pos>, u32)
where
	T: Into<u32> + Copy + Clone,
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
