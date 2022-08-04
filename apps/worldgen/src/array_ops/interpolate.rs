use dep::resize;
use dep::resize::Pixel::Gray8;
use dep::resize::Type::Mitchell;

pub fn mitchell(
	array: Vec<f32>,
	size: u32,
	map_size: u32,
) -> Vec<u8> {
	let size = size as usize;
	let map_size = map_size as usize;
	let mut src = vec![0; size * size];
	let mut dst = vec![0; map_size * map_size];
	for index in 0..size * size {
		src[index] = array[index] as u8;
	}
	let mut resizer =
		resize::new(size, size, map_size, map_size, Gray8, Mitchell).unwrap();
	let _res = resizer.resize(&src, &mut dst);
	dst
}
