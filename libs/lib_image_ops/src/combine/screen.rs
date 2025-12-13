// [Revision: v0.1] [Path: libs/lib_image_ops/src/combine/screen.rs] [Date: 2025-12-13]
// Description: Screen blending mode

pub fn get(
	idat: &mut Vec<u8>,
	idat_bg: Vec<u8>,
	idat_fg: Vec<u8>,
) {
	let size = idat.len();

	for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
		*cell_v = (255.0
			* (1.0
				- (1.0 - idat_bg[i] as f32 / 255.0)
					* (1.0 - idat_fg[i] as f32 / 255.0))) as u8;
	}
}
