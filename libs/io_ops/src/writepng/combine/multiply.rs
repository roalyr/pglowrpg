pub fn get(
  idat: &mut Vec<u8>,
  idat_bg: Vec<u8>,
  idat_fg: Vec<u8>,
) {
  let size = idat.len();

  for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
    *cell_v = (idat_bg[i] as f32 * idat_fg[i] as f32 / 255.0) as u8;
  }
}
