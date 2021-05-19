use crate::array_ops::noise_maps;
use crate::layer_ops::river_mapping::RgParams;
use constants::world as cw;
use game_data_codec::LayerPack;
use line_drawing::BresenhamCircle;
use unit_systems::translate;

// Has to be imprived, refactoring later
impl RgParams {
	pub fn with_water(
		&mut self,
		lp: &mut LayerPack,
	) {
		let iter = self.dv.x0 + self.dv.y0;
		let random =
			pseudo_rng::get(0.0, 1.0, lp.wi.seed, self.dv.x0 * self.dv.y0 * iter);
		if random < lp.wi.river_rand_vectors {
			self.without_water(lp);
		} else {
			self.seek_waterbodies(lp);
		}
	}

	pub fn without_water(
		&mut self,
		lp: &mut LayerPack,
	) {
		//if there is a pole - go for biased placement
		self.without_water_no_pole(lp);
	}

	pub fn without_water_no_pole(
		&mut self,
		lp: &mut LayerPack,
	) {
		self.project_randomly(lp);
	}

	fn seek_waterbodies(
		&mut self,
		lp: &mut LayerPack,
	) {
		//Aliases
		let size = lp.wi.map_size;
		//take a bit more than sqrt(2)
		let diag = size * 15 / 10;
		loop {
			for (i, j) in BresenhamCircle::new(
				self.dv.x0 as i32,
				self.dv.y0 as i32,
				self.dv.r as i32,
			) {
				let i = i as usize;
				let j = j as usize;
				if (i < size) && (j < size) {
					let index = lp.xy.ind(i, j);
					let wmask = lp.topography.read(lp.topography.WATERMASK, index);
					let temp = lp.climate.read(lp.climate.TEMPERATURE, index);
					let temp_abs = translate::get_abs(
						temp as f32,
						cw::VAL_255_F32,
						lp.wi.abs_temp_min as f32,
						lp.wi.abs_temp_max as f32,
					) as isize;
					if (wmask >= lp.wi.river_attr_pool_size_pow)
						&& (temp_abs > cw::TEMP_POLAR)
					{
						self.dv.x1 = i;
						self.dv.y1 = j;
						self.dv.hit = true;
						break;
					}
				}
			}
			if !self.dv.hit {
				self.dv.r += 1;
			} else {
				//reset stuff
				self.dv.r = cw::ONE_USIZE;
				self.dv.hit = false;
				break;
			}
			//stop if no prev. check worked
			if self.dv.r >= diag {
				break;
			}
		}
	}

	fn project_randomly(
		&mut self,
		lp: &mut LayerPack,
	) {
		//Aliases
		let vec_angle = lp.wi.river_vect_angle;
		let vec_deviation = lp.wi.river_vect_angle_max_deviation;
		let noise_factor = lp.wi.river_vect_angle_noise;
		let size = lp.wi.map_size as f32;
		let radius = size * 1.5;
		let random = pseudo_rng::get(
			0.0,
			1.0,
			lp.wi.seed,
			self.dv.x1 * self.dv.y0 + self.dv.y1 * self.dv.x0,
		);
		let mut shift = 2.0
			* std::f32::consts::PI
			* noise_maps::point_multi(
				noise_factor,
				lp.wi.seed,
				self.dv.x0,
				self.dv.y0,
			);
		if random > 0.5 {
			shift = -shift;
		}
		let mut vec_angle_mod: f32 = vec_angle + shift;
		//limit the deviation of the river vector to keep it aligned
		if vec_angle_mod > vec_angle + vec_deviation {
			vec_angle_mod = vec_angle + vec_deviation;
		}
		if vec_angle_mod < vec_angle - vec_deviation {
			vec_angle_mod = vec_angle - vec_deviation;
		}
		//println!("{:?}", vec_angle_mod);
		let x0 = self.dv.x0 as f32;
		let y0 = self.dv.y0 as f32;
		let xr = radius * vec_angle_mod.cos();
		let yr = radius * vec_angle_mod.sin();
		let mut x1 = x0 + xr;
		let mut y1 = y0 + yr;
		if x1 >= size {
			x1 = size - 1.0;
		}
		if y1 >= size {
			y1 = size - 1.0;
		}
		if x1 < 0.0 {
			x1 = 0.0;
		}
		if y1 < 0.0 {
			y1 = 0.0;
		}
		self.dv.x1 = x1 as usize;
		self.dv.y1 = y1 as usize;
	}
} //impl
