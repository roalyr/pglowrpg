// [Revision: v0.1] [Path: libs/lib_image_ops/src/combine/overlay.rs] [Date: 2025-12-13]
// Description: Overlay blending mode

pub fn get(
	idat: &mut Vec<u8>,
	idat_bg: Vec<u8>,
	idat_fg: Vec<u8>,
) {
	let size = idat.len();

	for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
		if idat_bg[i] < 127 {
			*cell_v = (2.0 * idat_bg[i] as f32 * idat_fg[i] as f32 / 255.0) as u8;
		} else {
			*cell_v = (255.0
				* (1.0
					- 2.0
						* (1.0 - idat_bg[i] as f32 / 255.0)
						* (1.0 - idat_fg[i] as f32 / 255.0))) as u8;
		}
	}
}
