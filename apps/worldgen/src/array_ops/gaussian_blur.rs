//https://docs.rs/fastblur/0.1.0/src/fastblur/blur.rs.html#1-9

use crate::array_ops::translate;
use std::cmp::min;

pub fn get(array: Vec<Vec<u8>>) -> Vec<Vec<u8>> {
	let size = array.len();

	let mut src = translate::to1d(array, size);

	gaussian_blur(&mut src, size, size, 50.0);

	translate::to2d(src, size)
}

fn gaussian_blur(
	data: &mut Vec<u8>,
	width: usize,
	height: usize,
	blur_radius: f32,
) {
	let boxes = create_box_gauss(blur_radius, 3);

	let mut backbuf = data.clone();

	for box_size in boxes.iter() {
		let radius = ((box_size - 1) / 2) as usize;

		box_blur(&mut backbuf, data, width, height, radius, radius);
	}
}

fn create_box_gauss(
	sigma: f32,
	n: usize,
) -> Vec<i32> {
	if sigma > 0.0 {
		let n_float = n as f32;

		let w_ideal = (12.0 * sigma * sigma / n_float).sqrt() + 1.0;

		let mut wl: i32 = w_ideal.floor() as i32;

		if wl % 2 == 0 {
			wl -= 1;
		};

		let wu = wl + 2;
		let wl_float = wl as f32;
		let m_ideal = (12.0 * sigma * sigma
			- n_float * wl_float * wl_float
			- 4.0 * n_float * wl_float
			- 3.0 * n_float)
			/ (-4.0 * wl_float - 4.0);

		let m: usize = m_ideal.round() as usize;

		let mut sizes = Vec::<i32>::new();
		for i in 0..n {
			if i < m {
				sizes.push(wl);
			} else {
				sizes.push(wu);
			}
		}
		sizes
	} else {
		vec![1; n]
	}
}

fn box_blur(
	backbuf: &mut Vec<u8>,
	frontbuf: &mut Vec<u8>,
	width: usize,
	height: usize,
	blur_radius_horz: usize,
	blur_radius_vert: usize,
) {
	box_blur_horz(backbuf, frontbuf, width, height, blur_radius_horz);

	box_blur_vert(frontbuf, backbuf, width, height, blur_radius_vert);
}

fn box_blur_vert(
	backbuf: &[u8],
	frontbuf: &mut [u8],
	width: usize,
	height: usize,
	blur_radius: usize,
) {
	if blur_radius == 0 {
		frontbuf.copy_from_slice(backbuf);
		return;
	}

	let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;

	for i in 0..width {
		let col_start = i;
		let col_end = i + width * (height - 1);
		let mut ti: usize = i;
		let mut li: usize = ti;
		let mut ri: usize = ti + blur_radius * width;

		let fv: u8 = backbuf[col_start];
		let lv: u8 = backbuf[col_end];

		let mut val_r: isize =
			(blur_radius as isize + 1) * isize::from(fv);

		let get_top = |i: usize| {
			if i < col_start {
				fv
			} else {
				backbuf[i]
			}
		};

		let get_bottom = |i: usize| {
			if i > col_end {
				lv
			} else {
				backbuf[i]
			}
		};

		for j in 0..min(blur_radius, height) {
			let bb = backbuf[ti + j * width];
			val_r += isize::from(bb);
		}
		if blur_radius > height {
			val_r +=
				(blur_radius - height) as isize * isize::from(lv);
		}

		for _ in 0..min(height, blur_radius + 1) {
			let bb = get_bottom(ri);
			ri += width;
			val_r += isize::from(bb) - isize::from(fv);

			frontbuf[ti] = round(val_r as f32 * iarr) as u8;

			ti += width;
		}

		if height > blur_radius {
			for _ in (blur_radius + 1)..(height - blur_radius) {
				let bb1 = backbuf[ri];
				ri += width;
				let bb2 = backbuf[li];
				li += width;

				val_r += isize::from(bb1) - isize::from(bb2);

				frontbuf[ti] = round(val_r as f32 * iarr) as u8;

				ti += width;
			}

			for _ in 0..min(height - blur_radius - 1, blur_radius) {
				let bb = get_top(li);
				li += width;

				val_r += isize::from(lv) - isize::from(bb);

				frontbuf[ti] = round(val_r as f32 * iarr) as u8;

				ti += width;
			}
		}
	}
}

fn box_blur_horz(
	backbuf: &[u8],
	frontbuf: &mut [u8],
	width: usize,
	height: usize,
	blur_radius: usize,
) {
	if blur_radius == 0 {
		frontbuf.copy_from_slice(backbuf);
		return;
	}

	let iarr = 1.0 / (blur_radius + blur_radius + 1) as f32;

	for i in 0..height {
		let row_start: usize = i * width;
		let row_end: usize = (i + 1) * width - 1;
		let mut ti: usize = i * width;
		let mut li: usize = ti;
		let mut ri: usize = ti + blur_radius;

		let fv: u8 = backbuf[row_start];
		let lv: u8 = backbuf[row_end];

		let mut val_r: isize =
			(blur_radius as isize + 1) * isize::from(fv);

		let get_left = |i: usize| {
			if i < row_start {
				fv
			} else {
				backbuf[i]
			}
		};

		let get_right = |i: usize| {
			if i > row_end {
				lv
			} else {
				backbuf[i]
			}
		};

		for j in 0..min(blur_radius, width) {
			let bb = backbuf[ti + j];
			val_r += isize::from(bb);
		}

		if blur_radius > width {
			val_r +=
				(blur_radius - height) as isize * isize::from(lv);
		}

		for _ in 0..min(width, blur_radius + 1) {
			let bb = get_right(ri);
			ri += 1;
			val_r += isize::from(bb) - isize::from(fv);
			frontbuf[ti] = round(val_r as f32 * iarr) as u8;

			ti += 1;
		}

		if width > blur_radius {
			for _ in (blur_radius + 1)..(width - blur_radius) {
				let bb1 = backbuf[ri];
				ri += 1;
				let bb2 = backbuf[li];
				li += 1;
				val_r += isize::from(bb1) - isize::from(bb2);

				frontbuf[ti] = round(val_r as f32 * iarr) as u8;

				ti += 1;
			}

			for _ in 0..min(width - blur_radius - 1, blur_radius) {
				let bb = get_left(li);
				li += 1;
				val_r += isize::from(lv) - isize::from(bb);

				frontbuf[ti] = round(val_r as f32 * iarr) as u8;

				ti += 1;
			}
		}
	}
}

// Source:https://stackoverflow.com/a/42386149/585725
fn round(mut x: f32) -> f32 {
	x += 123.0;
	x -= 123.0;
	x
}
