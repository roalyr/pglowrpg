// [Revision: v0.1] [Path: apps/worldgen/src/array_ops/interpolate.rs] [Date: 2025-12-13]
// Description: Image interpolation/resizing using Mitchell filter

use dep::resize;
use dep::resize::Pixel::Gray8;
use dep::resize::Type::Mitchell;
use dep::resize::px::Gray;

pub fn mitchell(
	array: Vec<f32>,
	size: u32,
	map_size: u32,
) -> Vec<u8> {
	let size = size as usize;
	let map_size = map_size as usize;
	let mut src: Vec<Gray<u8>> = vec![Gray::new(0); size * size];
	let mut dst: Vec<Gray<u8>> = vec![Gray::new(0); map_size * map_size];
	for index in 0..size * size {
		src[index] = Gray::new(array[index] as u8);
	}
	let mut resizer =
		resize::new(size, size, map_size, map_size, Gray8, Mitchell).unwrap();
	let _res = resizer.resize(&src, &mut dst);
	// Convert Gray<u8> back to u8
	dst.into_iter().map(|g| g.value()).collect()
}
