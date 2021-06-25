use unit_systems::coords::Index;

pub struct FloodFill<'a, T> {
	template_data: &'a Vec<T>, // input to determine adjacent cells
	map_size: u32,
	pub exclusion_map: Vec<bool>, // stuff that was filled
	pub region_map: Vec<bool>,    // to-be filled area
	pub region_size: u32,         // number of cells
	pub region_ox: u32,           // centroid x
	pub region_oy: u32,           // centroid y
	pub x_min: u32,               // region bound
	pub x_max: u32,
	pub y_min: u32,
	pub y_max: u32,
}

impl<T> FloodFill<'_, T> {
	pub fn map(
		&mut self,
		i: u32,
		j: u32,
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
		let size = self.map_size;
		// Just in case, if memeory is overused here, go for u16.
		queue.push((i, j));
		while let Some(point) = queue.pop() {
			let (i, j): (u32, u32) = point;
			let ind = index.get(i, j);
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
				if self.exclusion_map[index.get(*ni, *nj)] {
					continue;
				};
				queue.push((*ni, *nj));
			}
			// Mark the point passed.
			self.exclusion_map[ind] = true;
			self.region_map[ind] = true;
			self.region_size += 1;
			// Bounds coordinates calculation.
			if (i as u32) < self.x_min {
				self.x_min = i;
			}
			if (i as u32) > self.x_max {
				self.x_max = i;
			}
			if (j as u32) < self.y_min {
				self.y_min = j;
			}
			if (j as u32) > self.y_max {
				self.y_max = j;
			}
		}
		// Centroid calculation.
		self.region_ox = (self.x_max - self.x_min) / 2 + self.x_min;
		self.region_oy = (self.y_max - self.y_min) / 2 + self.y_min;
	}
	// The main function.
	pub fn new(
		template_data: &Vec<T>,
		map_size: u32,
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
