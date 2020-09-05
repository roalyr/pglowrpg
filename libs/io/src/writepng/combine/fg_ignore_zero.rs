pub fn get(
	idat: &mut Vec<u8>,
	idat_bg: Vec<u8>,
	idat_fg: Vec<u8>,
) {
	let size = idat.len();

	for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
		if idat_fg[i] != 0 {
			*cell_v = idat_fg[i];
		} else {
			*cell_v = idat_bg[i];
		}
	}
}
