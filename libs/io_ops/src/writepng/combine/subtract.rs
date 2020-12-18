pub fn get(
  idat: &mut Vec<u8>,
  idat_bg: Vec<u8>,
  idat_fg: Vec<u8>,
) {
  let size = idat.len() / 4;

  for i in 0..size {
    let index = i * 4;
    idat[index + 3] = 255; //Alpha channel
    idat[index] = idat_bg[index].saturating_sub(idat_fg[index]);
    idat[index + 1] = idat_bg[index + 1].saturating_sub(idat_fg[index + 1]);
    idat[index + 2] = idat_bg[index + 2].saturating_sub(idat_fg[index + 2]);
  }
}
