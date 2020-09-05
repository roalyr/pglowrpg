use coords::Index;

pub struct FloodFill<'a, T> {
	template_data: &'a Vec<T>,
	map_size: usize,
	pub exclusion_map: Vec<bool>,
	pub region_map: Vec<bool>,
	pub region_size: usize,
	pub region_ox: usize,
	pub region_oy: usize,
	pub x_min: usize,
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
		let xy = Index {
			map_size: self.map_size,
		};

		//reset prev. region
		for x in self.x_min..=self.x_max {
			for y in self.y_min..=self.y_max {
				self.region_map[xy.ind(x, y)] = false;
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
		let target = self.template_data[xy.ind(i, j)];

		let size = self.map_size as u16;
		queue.push((i as u16, j as u16));

		let mut neighbors = Vec::with_capacity(4);

		while let Some(point) = queue.pop() {
			let (i, j): (u16, u16) = point;

			let index = xy.ind(i as usize, j as usize);

			if self.template_data[index] != target {
				continue;
			}

			neighbors.clear();

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
				if self.exclusion_map
					[xy.ind(*ni as usize, *nj as usize)]
				{
					continue;
				};
				queue.push((*ni, *nj));
			}

			self.exclusion_map[index] = true;
			self.region_map[index] = true;
			self.region_size += 1;

			//result.push(point);
			//}
			//for (i, j) in &result {

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
		self.region_ox = (self.x_max - self.x_min) / 2 + self.x_min;

		self.region_oy = (self.y_max - self.y_min) / 2 + self.y_min;
	}

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
