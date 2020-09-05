pub fn get(
	idat: &mut Vec<u8>,
	idat_bg: Vec<u8>,
	idat_fg: Vec<u8>,
) {
	let size = idat.len();

	for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
		let mut val = idat_bg[i] as usize + idat_fg[i] as usize;
		if val > 255 {
			val = 255;
		}
		*cell_v = val as u8;
	}
}
