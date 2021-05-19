use unit_systems::coords::Index;

pub struct FloodFill<'a, T> {
	template_data: &'a Vec<T>, // input to determine adjacent cells
	map_size: usize,
	pub exclusion_map: Vec<bool>, // stuff that was filled
	pub region_map: Vec<bool>,    // to-be filled area
	pub region_size: usize,       // number of cells
	pub region_ox: usize,         // centroid x
	pub region_oy: usize,         // centroid y
	pub x_min: usize,             // region bound
	pub x_max: usize,
	pub y_min: usize,
	pub y_max: usize,
}

impl<T> FloodFill<'_, T> {
	pub fn map(
		&mut self,
		i: usize,
		j: usize,
	) where
		T: PartialEq + Copy + Clone,
	{
		let index = Index {
			map_size: self.map_size,
		};
		// Reset previous region data.
		for y in self.y_min..=self.y_max {
			for x in self.x_min..=self.x_max {
				self.region_map[index.get(x, y)] = false;
			}
		}
		self.region_size = 0;
		self.region_ox = i;
		self.region_oy = j;
		self.x_min = i;
		self.x_max = i;
		self.y_min = j;
		self.y_max = j;
		let mut queue = Vec::new();
		let mut neighbors = Vec::with_capacity(4);
		let target = self.template_data[index.get(i, j)];
		// TODO: why u16 here?
		let size = self.map_size as u16;
		queue.push((i as u16, j as u16));
		while let Some(point) = queue.pop() {
			let (i, j): (u16, u16) = point;
			let ind = index.get(i as usize, j as usize);
			if self.template_data[ind] != target {
				continue;
			}
			neighbors.clear();
			// TODO: make match here instead.
			// Also make saturating ops?
			if i < size - 1 {
				neighbors.push((i + 1, j))
			};
			if i > 0 {
				neighbors.push((i - 1, j))
			};
			if j < size - 1 {
				neighbors.push((i, j + 1))
			};
			if j > 0 {
				neighbors.push((i, j - 1))
			};
			for (ni, nj) in neighbors.iter() {
				if self.exclusion_map[index.get(*ni as usize, *nj as usize)] {
					continue;
				};
				queue.push((*ni, *nj));
			}
			// Mark the point passed.
			self.exclusion_map[ind] = true;
			self.region_map[ind] = true;
			self.region_size += 1;
			// Bounds coordinates calculation.
			if (i as usize) < self.x_min {
				self.x_min = i as usize;
			}
			if (i as usize) > self.x_max {
				self.x_max = i as usize;
			}
			if (j as usize) < self.y_min {
				self.y_min = j as usize;
			}
			if (j as usize) > self.y_max {
				self.y_max = j as usize;
			}
		}
		// Centroid calculation.
		self.region_ox = (self.x_max - self.x_min) / 2 + self.x_min;
		self.region_oy = (self.y_max - self.y_min) / 2 + self.y_min;
	}
	// The main function.
	#[allow(clippy::ptr_arg)]
	pub fn new(
		template_data: &Vec<T>,
		map_size: usize,
	) -> FloodFill<T> {
		let exclusion_map = vec![false; template_data.len()];
		let region_map = vec![false; template_data.len()];
		let region_size = 0;
		let region_ox = 0;
		let region_oy = 0;
		let x_min = 0;
		let x_max = 0;
		let y_min = 0;
		let y_max = 0;
		FloodFill {
			template_data,
			map_size,
			exclusion_map,
			region_map,
			region_size,
			region_ox,
			region_oy,
			x_min,
			x_max,
			y_min,
			y_max,
		}
	}
}
