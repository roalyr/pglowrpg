use resize::Pixel::Gray8;
use resize::Type::Mitchell;

pub fn mitchell(
  array: Vec<f32>,
  size: usize,
  map_size: usize,
) -> Vec<u8> {
  let mut src = vec![0; size * size];
  let mut dst = vec![0; map_size * map_size];
  for index in 0..size * size {
    src[index] = array[index] as u8;
  }
  let mut resizer =
    resize::new(size, size, map_size, map_size, Gray8, Mitchell);
  resizer.resize(&src, &mut dst);
  dst
}
