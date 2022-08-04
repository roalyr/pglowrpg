use crate::layer_ops::river_mapping::RgParams;
use dep::line_drawing::BresenhamCircle;
use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_game_data_codec::LayerPack;
use lib_unit_systems::translate;

impl RgParams {
	// If water bodies exist, make rivers fall into them, but
	// also allow some to go randomly.
	pub fn rivers_with_water_bodies(
		&mut self,
		lp: &mut LayerPack,
	) {
		let size = lp.wi.map_size;
		// Take a bit more than sqrt(2).
		let diag = size * 15 / 10;
		loop {
			for (i, j) in BresenhamCircle::new(
				self.dv.x0 as i32,
				self.dv.y0 as i32,
				self.dv.r as i32,
			) {
				let i = i as u32;
				let j = j as u32;
				if (i < size) && (j < size) {
					let index = lp.index.get(i, j);
					let wmask = lp.topography.read(lp.topography.WATERMASK, index);
					let temp = lp.climate.read(lp.climate.TEMPERATURE, index);
					let temp_abs = translate::get_abs(
						temp as f32,
						cg::VAL_255_F32,
						lp.wi.abs_temp_min as f32,
						lp.wi.abs_temp_max as f32,
					) as i32;
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
				// Reset.
				self.dv.r = cg::ONE_U32;
				self.dv.hit = false;
				break;
			}
			// Stop, in case previous check failed.
			if self.dv.r >= diag {
				break;
			}
		}
	}

	// If no water bodies exist - all the rivers should go randomly.
	pub fn rivers_without_water_bodies(
		&mut self,
		lp: &mut LayerPack,
	) {
		let ind = lp.index.get(self.dv.x0, self.dv.y0);
		let random = lib_pseudo_rng::get(0.0, 1.0, lp.wi.seed + 754532541, ind);
		// Decide which edge to put the end onto.
		if random > 0.5 {
			self.randomize_end_x(lp, ind);
		} else {
			self.randomize_end_y(lp, ind);
		}
	}

	fn randomize_end_x(
		&mut self,
		lp: &mut LayerPack,
		ind: usize,
	) {
		// Put it on either of the edges.
		let random = lib_pseudo_rng::get(0.0, 1.0, lp.wi.seed + 578241369, ind);
		if random > 0.5 {
			self.dv.y1 = 0;
		} else {
			self.dv.y1 = lp.wi.map_size - 1;
		}
		// At a random coordinate.
		self.dv.x1 =
			lib_pseudo_rng::get(0.0, lp.wi.map_size as f32, lp.wi.seed + 3, ind)
				as u32;
	}

	fn randomize_end_y(
		&mut self,
		lp: &mut LayerPack,
		ind: usize,
	) {
		// Put it on either of the edges.
		let random = lib_pseudo_rng::get(0.0, 1.0, lp.wi.seed + 607421365, ind);
		if random > 0.5 {
			self.dv.x1 = 0;
		} else {
			self.dv.x1 = lp.wi.map_size - 1;
		}
		// At a random coordinate.
		self.dv.y1 =
			lib_pseudo_rng::get(0.0, lp.wi.map_size as f32, lp.wi.seed + 4, ind)
				as u32;
	}
}
