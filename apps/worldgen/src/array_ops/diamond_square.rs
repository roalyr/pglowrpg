use crate::array_ops::interpolate::mitchell;
use coords::Index;

const DS_CROP: usize = 20;
const ONE_F32: f32 = 1.0;

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
	size: usize, land_concentrator: f32, land_scope: f32,
	land_continuity: f32, seed: usize,
) -> Vec<f32> {
	let xy = Index { map_size: size };

	//size+1 in both directions
	let size_big = size * size + 2 * size + 1;

	let mut p = Params {
		size,
		array: vec![ONE_F32; size_big],
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

				let sum = p.array[xy.ind(x, y)]
					+ p.array[xy.ind(x, region_y)]
					+ p.array[xy.ind(region_x, y)]
					+ p.array[xy.ind(region_x, region_y)];

				let shift = prng::get(
					-half_size_f
						+ land_continuity
							* half_size_f * (ONE_F32 - land_concentrator),
					half_size_f
						+ land_continuity
							* half_size_f * (ONE_F32 - land_concentrator),
					p.seed,
					p.iter,
				);

				let avg = sum / 4.0
					+ (ONE_F32 - land_concentrator)
						* land_scope * sum * shift
					+ land_concentrator * shift;

				p.array[xy.ind(center_x, center_y)] = avg;

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

fn diamond_substep(p: &mut Params, center_x: usize, center_y: usize) {
	let xy = Index { map_size: p.size };

	p.iter += 1;
	let half_size = p.step_len / 2;
	let half_size_f = half_size as f32;
	let sum2 = p.array[xy
		.ind((center_x.saturating_sub(half_size)) % p.size, center_y)]
		+ p.array[xy.ind((center_x + half_size) % p.size, center_y)]
		+ p.array[xy.ind(
			center_x,
			(center_y.saturating_sub(half_size)) % p.size,
		)] + p.array
		[xy.ind(center_x, (center_y + half_size) % p.size)];

	let shift = prng::get(
		-half_size_f
			+ p.land_continuity
				* half_size_f * (ONE_F32 - p.land_concentrator),
		half_size_f
			+ p.land_continuity
				* half_size_f * (ONE_F32 - p.land_concentrator),
		p.seed,
		p.iter,
	);

	let avg2 = (p.land_concentrator * sum2 / 4.0 * half_size_f
		+ (ONE_F32 - p.land_concentrator) * p.land_scope * sum2 * shift
		+ p.land_concentrator * p.land_scope * shift
		+ (ONE_F32 - p.land_concentrator) * sum2 / 4.0)
		/ (p.land_concentrator * half_size_f
			+ (ONE_F32 - p.land_concentrator));

	p.array[xy.ind(center_x, center_y)] = avg2;
}

fn normalize_crop(mut array: Vec<f32>, size: usize) -> Vec<f32> {
	let xy = Index { map_size: size };
	let xy_big = Index {
		map_size: size + DS_CROP,
	};

	let mut array_final = vec![0.0; size * size];

	//size+1 in both directions
	let size_big = size * size + 2 * size + 1;

	let mut max_v = 0.0;
	for cell_v in array.iter_mut().take(size_big) {
		*cell_v *= *cell_v;

		if *cell_v > max_v {
			max_v = *cell_v;
		}
	}
	let k = 255.0 / max_v;

	for (index, cell_v) in
		array_final.iter_mut().enumerate().take(size * size)
	{
		*cell_v = array[index] * k;
		//println!("{:?}", cell_v);
	}

	let array_sized =
		mitchell(array_final.clone(), size, size + DS_CROP);

	for i in 0..size {
		for j in 0..size {
			array_final[xy.ind(i, j)] = array_sized
				[xy_big.ind(i + DS_CROP / 2, j + DS_CROP / 2)]
				as f32;
		}
	}
	array_final
}
