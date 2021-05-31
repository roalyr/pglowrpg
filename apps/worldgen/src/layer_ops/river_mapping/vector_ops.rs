use crate::layer_ops::river_mapping::RgParams;
use game_data_codec::LayerPack;

#[rustfmt::skip]
impl RgParams {
	pub fn vector_within_len(&self, allowed: usize,) -> bool {self.length() <= allowed}
	pub fn vector_start(&mut self, i: usize, j: usize,) {self.dv.x0 = i; self.dv.y0 = j;}
	
	pub fn vector_end(&mut self, lp: &mut LayerPack,) {
		// Decide how to find the end of the river vector.
		if self.water_bodies_present {
			self.rivers_with_water_bodies(lp);
		} else {
			self.rivers_without_water_bodies(lp);
		}
	}
	
	pub fn vector_bound(&mut self,allowed: usize,) {
		// Bound up.
		if self.dv.x0 >= allowed {self.dv.x0 = allowed - 1;}
		if self.dv.y0 >= allowed {self.dv.y0 = allowed - 1;}
		if self.dv.x1 >= allowed {self.dv.x1 = allowed - 1;}
		if self.dv.y1 >= allowed {self.dv.y1 = allowed - 1;}
		// Bound down.
		if self.dv.x0 == 0 {self.dv.x0 = 0;}
		if self.dv.y0 == 0 {self.dv.y0 = 0;}
		if self.dv.x1 == 0 {self.dv.x1 = 0;}
		if self.dv.y1 == 0 {self.dv.y1 = 0;}
	}
	
	pub fn length(&self) -> usize {
		((self.dv.x0 as f32 - self.dv.x1 as f32).powf(2.0) + (self.dv.y0 as f32 - self.dv.y1 as f32).powf(2.0))
		.powf(0.5) as usize
	}
	
	pub fn distance(&self) -> usize {
		((self.dv.x0 as i32 - self.dv.x1 as i32).abs() + (self.dv.y0 as i32 - self.dv.y1 as i32).abs()) as usize
	}
}
