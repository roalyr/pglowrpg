use coords::Index;
use pathfinding::prelude::astar;

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
	fn neighbors(
		&self,
		array_input: &Vec<u8>,
		xy: Index,
		step: usize,
		goal: Pos,
		diag_flag: bool,
	) -> Vec<(Pos, usize)> {
		let &Pos(x, y) = self;
		let mut vect = Vec::new();

		//diagonal directions. bounded, if enabled
		if diag_flag {
			if within_bounds(x + step, y + step, xy.map_size) {
				vect.push((
					Pos(x + step, y + step),
					W_DIA
						* array_input[xy.ind(x + step, y + step)]
							as usize,
				))
			};

			if within_bounds(
				x.saturating_sub(step),
				y.saturating_sub(step),
				xy.map_size,
			) {
				vect.push((
					Pos(
						x.saturating_sub(step),
						y.saturating_sub(step),
					),
					W_DIA
						* array_input[xy.ind(
							x.saturating_sub(step),
							y.saturating_sub(step),
						)] as usize,
				))
			};

			if within_bounds(
				x + step,
				y.saturating_sub(step),
				xy.map_size,
			) {
				vect.push((
					Pos(x + step, y.saturating_sub(step)),
					W_DIA
						* array_input
							[xy.ind(x + step, y.saturating_sub(step))]
							as usize,
				))
			};

			if within_bounds(
				x.saturating_sub(step),
				y + step,
				xy.map_size,
			) {
				vect.push((
					Pos(x.saturating_sub(step), y + step),
					W_DIA
						* array_input
							[xy.ind(x.saturating_sub(step), y + step)]
							as usize,
				))
			};
		}

		//orthogonal directions. bounded
		if within_bounds(x, y + step, xy.map_size) {
			vect.push((
				Pos(x, y + step),
				W_ORT * array_input[xy.ind(x, y + step)] as usize,
			))
		};

		if within_bounds(x, y.saturating_sub(step), xy.map_size) {
			vect.push((
				Pos(x, y.saturating_sub(step)),
				W_ORT
					* array_input[xy.ind(x, y.saturating_sub(step))]
						as usize,
			))
		};

		if within_bounds(x + step, y, xy.map_size) {
			vect.push((
				Pos(x + step, y),
				W_ORT * array_input[xy.ind(x + step, y)] as usize,
			))
		};

		if within_bounds(x.saturating_sub(step), y, xy.map_size) {
			vect.push((
				Pos(x.saturating_sub(step), y),
				W_ORT
					* array_input[xy.ind(x.saturating_sub(step), y)]
						as usize,
			))
		};

		//check to force pathfinding to converge
		if self.distance(&goal) <= step {
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
pub fn make(
	v: &DirVector,
	array_input: &Vec<u8>,
	map_size: usize,
	diag_flag: bool,
	step: usize,
) -> (std::vec::Vec<Pos>, usize) {
	let goal: Pos = Pos(v.x1, v.y1);
	let start: Pos = Pos(v.x0, v.y0);

	let xy = Index { map_size };

	let path = astar(
		&start,
		|p| p.neighbors(array_input, xy, step, goal, diag_flag),
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
