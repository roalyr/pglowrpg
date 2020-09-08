use coords::Index;
use pathfinding::prelude::astar;
use std::io::Read;

const W_ORT: usize = 100;
const W_DIA: usize = 141; //aka sqrt from 2

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
	#[inline]
	fn distance(
		&self,
		other: &Pos,
	) -> usize {
		((self.0 as i32 - other.0 as i32).abs()
			+ (self.1 as i32 - other.1 as i32).abs()) as usize
	} //fn

	#[allow(clippy::ptr_arg)]
	#[inline]
	fn neighbors<T>(
		&self,
		map: &Vec<T>,
		map_size: usize,
		xy: Index,
		step: usize,
		goal: Pos,
		diag_flag: bool,
	) -> Vec<(Pos, usize)>
	where
		T: Into<usize> + Copy + Clone,
	{
		let &Pos(x, y) = self;
		let mut vect = Vec::new();

		//diagonal directions. bounded, if enabled
		if diag_flag {
			let i = x + step;
			let j = y + step;
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;

				let index = xy.ind(i, j);
				let map_val = dir_weight * map[index].into();
				vect.push((Pos(i, j), map_val))
			};

			let i = x.saturating_sub(step);
			let j = y.saturating_sub(step);
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;

				let index = xy.ind(i, j);
				let map_val = dir_weight * map[index].into();
				vect.push((Pos(i, j), map_val))
			};

			let i = x + step;
			let j = y.saturating_sub(step);
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;

				let index = xy.ind(i, j);
				let map_val = dir_weight * map[index].into();
				vect.push((Pos(i, j), map_val))
			};

			let i = x.saturating_sub(step);
			let j = y + step;
			if within_bounds(i, j, map_size) {
				let dir_weight = W_DIA;

				let index = xy.ind(i, j);
				let map_val = dir_weight * map[index].into();
				vect.push((Pos(i, j), map_val))
			};
		}

		//orthogonal directions. bounded
		let i = x;
		let j = y + step;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;

			let index = xy.ind(i, j);
			let map_val = dir_weight * map[index].into();
			vect.push((Pos(i, j), map_val))
		};

		let i = x;
		let j = y.saturating_sub(step);
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;

			let index = xy.ind(i, j);
			let map_val = dir_weight * map[index].into();
			vect.push((Pos(i, j), map_val))
		};

		let i = x + step;
		let j = y;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;

			let index = xy.ind(i, j);
			let map_val = dir_weight * map[index].into();
			vect.push((Pos(i, j), map_val))
		};

		let i = x.saturating_sub(step);
		let j = y;
		if within_bounds(i, j, map_size) {
			let dir_weight = W_ORT;

			let index = xy.ind(i, j);
			let map_val = dir_weight * map[index].into();
			vect.push((Pos(i, j), map_val))
		};

		//check to force pathfinding to converge
		if self.distance(&goal) < step * 2 {
			vect.push((goal, 0))
		}

		vect
	} //fn
} //impl

#[inline]
fn within_bounds(
	i: usize,
	j: usize,
	size: usize,
) -> bool {
	(i < size) && (j < size)
}

#[allow(clippy::ptr_arg)]
pub fn make<T>(
	v: &DirVector,
	map: &Vec<T>,
	map_size: usize,
	diag_flag: bool,
	step: usize,
) -> (std::vec::Vec<Pos>, usize)
where
	T: Into<usize> + Copy + Clone,
{
	let goal: Pos = Pos(v.x1, v.y1);
	let start: Pos = Pos(v.x0, v.y0);

	let xy = Index { map_size };

	let path = astar(
		&start,
		|p| p.neighbors(map, map_size, xy, step, goal, diag_flag),
		|p| p.distance(&goal) * v.path_heuristic,
		|p| *p == goal,
	);

	match path {
		Some(x) => x,
		None => panic!(
			"Error in pathfinding. Start: {:?}, goal: {:?}",
			start, goal
		),
	}
}
