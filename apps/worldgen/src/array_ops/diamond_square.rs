use crate::array_ops::interpolate::mitchell;
use constants::world as cw;
use unit_systems::coords::Index;

// A margin that is meant to crop the border artifacts in DS algorithm
const DS_CROP: usize = 20;

struct Params {
	size: usize,
	array: Vec<f32>,
	land_concentrator: f32,
	land_scope: f32,
	land_continuity: f32,
	seed: usize,
	iter: usize,
	step_len: usize,
	step_increment: usize,
}

pub fn get(
	size: usize,
	land_concentrator: f32,
	land_scope: f32,
	land_continuity: f32,
	seed: usize,
) -> Vec<f32> {
	let index = Index { map_size: size };
	//size+1 in both directions
	let size_big = size * size + 2 * size + 1;
	let mut p = Params {
		size,
		array: vec![cw::ONE_F32; size_big],
		land_concentrator,
		land_scope,
		land_continuity,
		seed,
		iter: 0,
		step_len: size,
		step_increment: 1,
	};
	while p.step_len > 1 {
		//square step
		let mut x = 0;
		for _ in 0..p.step_increment {
			let mut y = 0;
			for _ in 0..p.step_increment {
				let region_x = p.step_len + x;
				let region_y = p.step_len + y;
				p.iter += 1;
				let half_size = (region_x - x) / 2;
				let half_size_f = half_size as f32;
				let center_x = x + half_size;
				let center_y = y + half_size;
				let sum = p.array[index.get(x, y)]
					+ p.array[index.get(x, region_y)]
					+ p.array[index.get(region_x, y)]
					+ p.array[index.get(region_x, region_y)];
				let shift = pseudo_rng::get(
					-half_size_f
						+ land_continuity * half_size_f * (cw::ONE_F32 - land_concentrator),
					half_size_f
						+ land_continuity * half_size_f * (cw::ONE_F32 - land_concentrator),
					p.seed,
					p.iter,
				);
				let avg = sum / 4.0
					+ (cw::ONE_F32 - land_concentrator) * land_scope * sum * shift
					+ land_concentrator * shift;
				p.array[index.get(center_x, center_y)] = avg;
				y += p.step_len;
			}
			x += p.step_len;
		}
		//diamond step
		let mut x = 0;
		for _ in 0..p.step_increment {
			let mut y = 0;
			for _ in 0..p.step_increment {
				let region_x = p.step_len + x;
				let region_y = p.step_len + y;
				diamond_substep(&mut p, x, (region_y + y) / 2);
				diamond_substep(&mut p, region_x, (region_y + y) / 2);
				diamond_substep(&mut p, (region_x + x) / 2, y);
				diamond_substep(&mut p, (region_x + x) / 2, region_y);
				y += p.step_len;
			}
			x += p.step_len;
		}
		p.step_len /= 2;
		p.step_increment *= 2;
	}
	normalize_crop(p.array, p.size)
}

fn diamond_substep(
	p: &mut Params,
	center_x: usize,
	center_y: usize,
) {
	let index = Index { map_size: p.size };
	p.iter += 1;
	let half_size = p.step_len / 2;
	let half_size_f = half_size as f32;
	let sum2 = p.array
		[index.get((center_x.saturating_sub(half_size)) % p.size, center_y)]
		+ p.array[index.get((center_x + half_size) % p.size, center_y)]
		+ p.array
			[index.get(center_x, (center_y.saturating_sub(half_size)) % p.size)]
		+ p.array[index.get(center_x, (center_y + half_size) % p.size)];
	let shift = pseudo_rng::get(
		-half_size_f
			+ p.land_continuity * half_size_f * (cw::ONE_F32 - p.land_concentrator),
		half_size_f
			+ p.land_continuity * half_size_f * (cw::ONE_F32 - p.land_concentrator),
		p.seed,
		p.iter,
	);
	let avg2 = (p.land_concentrator * sum2 / 4.0 * half_size_f
		+ (cw::ONE_F32 - p.land_concentrator) * p.land_scope * sum2 * shift
		+ p.land_concentrator * p.land_scope * shift
		+ (cw::ONE_F32 - p.land_concentrator) * sum2 / 4.0)
		/ (p.land_concentrator * half_size_f + (cw::ONE_F32 - p.land_concentrator));
	p.array[index.get(center_x, center_y)] = avg2;
}

fn normalize_crop(
	mut array: Vec<f32>,
	size: usize,
) -> Vec<f32> {
	let index = Index { map_size: size };
	let index_big = Index {
		map_size: size + DS_CROP,
	};
	let mut array_final = vec![cw::ZERO_F32; size * size];
	//size+1 in both directions
	let size_big = size * size + 2 * size + 1;
	let mut max_v = cw::ZERO_F32;
	for cell_v in array.iter_mut().take(size_big) {
		*cell_v *= *cell_v;
		if *cell_v > max_v {
			max_v = *cell_v;
		}
	}
	let k = cw::VAL_255_F32 / max_v;
	for (ind, cell_v) in array_final.iter_mut().enumerate().take(size * size) {
		*cell_v = array[ind] * k;
	}
	let array_sized = mitchell(array_final.clone(), size, size + DS_CROP);
	for i in 0..size {
		for j in 0..size {
			array_final[index.get(i, j)] =
				array_sized[index_big.get(i + DS_CROP / 2, j + DS_CROP / 2)] as f32;
		}
	}
	array_final
}
