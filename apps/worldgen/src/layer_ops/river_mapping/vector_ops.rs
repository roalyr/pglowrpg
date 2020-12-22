use crate::layer_ops::river_mapping::*;

impl RgParams {
	pub fn vector_start(
		&mut self,
		i: usize,
		j: usize,
	) {
		self.dv.x0 = i;
		self.dv.y0 = j;
	}

	pub fn vector_end(
		&mut self,
		lp: &mut LayerPack,
	) {
		let mut water_bodies = false;
		for index in 0..lp.layer_vec_len {
			let wmask = lp.topography.read(lp.topography.WATERMASK, index);
			if wmask >= lp.wi.river_attr_pool_size_pow {
				water_bodies = true;
			}
		}
		if water_bodies {
			self.with_water(lp);
		} else {
			self.without_water(lp);
		}
	}

	pub fn vector_within_len(
		&self,
		allowed: usize,
	) -> bool {
		self.length() <= allowed
	}

	pub fn length(&self) -> usize {
		((self.dv.x0 as f32 - self.dv.x1 as f32).powf(2.0)
			+ (self.dv.y0 as f32 - self.dv.y1 as f32).powf(2.0))
		.powf(0.5) as usize
	}

	pub fn distance(&self) -> usize {
		((self.dv.x0 as i32 - self.dv.x1 as i32).abs()
			+ (self.dv.y0 as i32 - self.dv.y1 as i32).abs()) as usize
	}

	pub fn vector_bound(
		&mut self,
		allowed: usize,
	) {
		//bound up
		if self.dv.x0 >= allowed {
			self.dv.x0 = allowed - 1;
		}
		if self.dv.y0 >= allowed {
			self.dv.y0 = allowed - 1;
		}
		if self.dv.x1 >= allowed {
			self.dv.x1 = allowed - 1;
		}
		if self.dv.y1 >= allowed {
			self.dv.y1 = allowed - 1;
		}
		//bound down
		if self.dv.x0 == 0 {
			self.dv.x0 = 0;
		}
		if self.dv.y0 == 0 {
			self.dv.y0 = 0;
		}
		if self.dv.x1 == 0 {
			self.dv.x1 = 0;
		}
		if self.dv.y1 == 0 {
			self.dv.y1 = 0;
		}
	}

	pub fn vector_end_stream(
		&mut self,
		lp: &mut LayerPack,
	) {
		self.dv.x1 = lp.wi.map_size / 2;
		self.dv.y1 = lp.wi.map_size / 2;
	}
} //impl
