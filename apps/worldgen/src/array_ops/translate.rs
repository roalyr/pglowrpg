use flate2::write::{GzDecoder, GzEncoder};
use flate2::Compression;
//use std::cmp::Ordering;
use std::io::prelude::*;

pub fn read_small_int16_(
	storage: u16,
	offset: u8,
	n_bits: u8,
) -> u8 {
	//println!("{:?}", ((storage >> offset as u16) & n_bits as u16).to_be_bytes());
	((storage >> offset as u16) & n_bits as u16).to_be_bytes()[1]
}

pub fn store_small_int16_(
	mut storage: u16,
	value: u8,
	offset: u8,
	cap: u8,
) {
	println!("{:?}", value);
	//check for overflow
	let integer: u16 = if (value as u16) < (cap as u16) {
		value as u16
	} else {
		panic!("the value is greater than {}", cap);
	};
	storage |= integer << offset;
	//println!("{:?}", storage);
}

pub fn to_byte(array: Vec<f32>) -> Vec<u8> {
	let size = array.len();
	let mut array_new = vec![0; size];

	for (index, cell_v) in array_new.iter_mut().enumerate().take(size)
	{
		*cell_v = array[index] as u8;
	}
	array_new
}

pub fn to_float(array: Vec<u8>) -> Vec<f32> {
	let size = array.len();
	let mut array_new = vec![0.0; size];

	for (index, cell_v) in array_new.iter_mut().enumerate().take(size)
	{
		*cell_v = array[index] as f32;
	}
	array_new
}

pub fn to2d<T>(
	array1d: Vec<T>,
	size: usize,
) -> Vec<Vec<T>>
where
	T: PartialOrd + Clone + Copy,
{
	let mut array2d: std::vec::Vec<std::vec::Vec<T>> =
		vec![Vec::with_capacity(size); size];

	for i in 0..size {
		for j in 0..size {
			array2d[i].push(array1d[i * size + j]);
		}
	}
	array2d
}

pub fn to1d<T>(
	array2d: Vec<Vec<T>>,
	size: usize,
) -> Vec<T>
where
	T: PartialOrd + Clone + Copy,
{
	let mut array1d: std::vec::Vec<T> =
		Vec::with_capacity(size * size);

	for (i, row) in array2d.iter().enumerate().take(size) {
		for (j, cell_v) in row.iter().enumerate().take(size) {
			array1d[i * size + j] = *cell_v;
		}
	}
	array1d
}

pub fn encode(array: Vec<u8>) -> Vec<u8> {
	let size = array.len();
	let mut e = GzEncoder::new(Vec::new(), Compression::default());

	for cell_v in array.iter().take(size) {
		e.write_all(&[*cell_v]).unwrap();
	}
	e.finish().unwrap()
}

pub fn decode(byte_array: &[u8]) -> Vec<u8> {
	let w = Vec::new();
	let mut d = GzDecoder::new(w);

	d.write_all(&byte_array).unwrap();
	d.finish().unwrap()
}

pub fn encode16(array: Vec<u16>) -> Vec<u8> {
	let size = array.len();

	let mut e = GzEncoder::new(Vec::new(), Compression::default());

	for cell_v in array.into_iter().take(size) {
		let val: [u8; 2] =
			unsafe { std::mem::transmute(cell_v.to_le()) };

		e.write_all(&[val[0]]).unwrap();
		e.write_all(&[val[1]]).unwrap();
	}
	e.finish().unwrap()
}

pub fn decode16(byte_array: &[u8]) -> Vec<u16> {
	let w = Vec::new();
	let mut d = GzDecoder::new(w);

	d.write_all(&byte_array).unwrap();
	let double_array = d.finish().unwrap();
	let size = double_array.len();

	let mut array_final = vec![0; size / 2];

	for index in 0..size / 2 {
		let val: u16 = unsafe {
			std::mem::transmute::<[u8; 2], u16>([
				double_array[2 * index],
				double_array[2 * index + 1],
			])
		};
		array_final[index] = val;
	}
	array_final
}

pub fn get_abs(
	val: f32,
	val_cap: f32,
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val / val_cap * (abs_max - abs_min) + abs_min
}

pub fn get_rel(
	val_abs: f32,
	val_cap: f32,
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val_cap * (val_abs - abs_min) / (abs_max - abs_min)
}
