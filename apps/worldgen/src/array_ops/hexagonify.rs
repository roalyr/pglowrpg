use dep::num;
use lib_unit_systems::coords::Index;
use std::collections::BTreeMap;

//HEXAGONIFY is not currently used.
pub fn hexagonify<T>(
	mut array: Vec<T>,
	size: u32,
) -> Vec<T>
where
	T: PartialOrd + Clone + Copy + Ord + num::NumCast,
{
	let mut off_i_odd = -7;
	let mut off_i_even = 0;
	let off_j_odd = -4;
	let off_j_even = 0;
	let rows = size / 14 + 1;
	for _ in 0..rows {
		array = hexrow(array, size, off_i_odd, off_j_odd);
		array = hexrow(array, size, off_i_even, off_j_even);
		off_i_odd += 14;
		off_i_even += 14;
	}
	array
}

//For now let it hang around.
//Need to make a proper switch between max / mean value use.
#[allow(unused_assignments, unused_variables)]
fn hexrow<T>(
	mut array: Vec<T>,
	size: u32,
	off_i: i32,
	off_j: i32,
) -> Vec<T>
where
	T: PartialOrd + Clone + Copy + Ord + num::NumCast,
{
	let index = Index { map_size: size };
	let hexagon = [
		[0, 0, 0, 1, 1, 0, 0, 0],
		[0, 1, 1, 1, 1, 1, 1, 0],
		[1, 1, 1, 1, 1, 1, 1, 1],
		[1, 1, 1, 1, 1, 1, 1, 1],
		[1, 1, 1, 1, 1, 1, 1, 1],
		[1, 1, 1, 1, 1, 1, 1, 1],
		[1, 1, 1, 1, 1, 1, 1, 1],
		[0, 1, 1, 1, 1, 1, 1, 0],
		[0, 0, 0, 1, 1, 0, 0, 0],
	];
	let mut k = 0;
	let mut l = 0;
	for n in 0..(size / 8 + 1) as i32 {
		//Read mean value.
		//int val = 0;
		let mut cell_vals = Vec::new();
		let mut num_vals = 0;
		let mut sum_val = 0.0;
		let mut mean_val = 0.0;
		for j in 0..8 {
			for i in 0..9 {
				if (hexagon[k][l] == 1)
					&& ((i + off_i) >= 0 && (i + off_i) < size as i32)
					&& ((j + off_j + 8 * n) >= 0 && (j + off_j + 8 * n) < size as i32)
				{
					//println!("x {}, y {:?}", i + off_i, j + off_j + 8 * n);
					let index = index.get((i + off_i) as u32, (j + off_j + 8 * n) as u32);
					cell_vals.push(array[index]);
					sum_val += num::cast::<T, f32>(array[index]).unwrap();
					num_vals += 1;
				}
				l += 1;
				if l == 8 {
					break;
				}
			}
			l = 0;
			k += 1;
			if k == 9 {
				break;
			}
		}
		k = 0;
		//Get mean value in cell.
		if num_vals > 0 {
			mean_val = sum_val / (num_vals as f32);
		} else {
			mean_val = 0.0;
		}
		//Get the most frequent value in cell.
		let mut counts = BTreeMap::new();
		for val in cell_vals.iter() {
			*counts.entry(val).or_insert(0) += 1;
		}
		let result = counts.into_iter().max_by_key(|&(_, count)| count);
		let max_val = match result {
			Some(x) => x,
			None => {
				//Return early if not found anything outstanding in numbers.
				return array;
			}
		};
		//Put data in hexagons.
		for j in 0..8 {
			for i in 0..9 {
				if (hexagon[k][l] == 1)
					&& ((i + off_i) >= 0 && (i + off_i) < size as i32)
					&& ((j + off_j + 8 * n) >= 0 && (j + off_j + 8 * n) < size as i32)
				{
					let index = index.get((i + off_i) as u32, (j + off_j + 8 * n) as u32);
					//Store either max value or mean value (todo).
					array[index] = *max_val.0;
				}
				l += 1;
				if l == 8 {
					break;
				}
			}
			l = 0;
			k += 1;
			if k == 9 {
				break;
			}
		}
		k = 0;
	}
	array
}
