// [Revision: v0.1] [Path: libs/lib_image_ops/src/combine/add.rs] [Date: 2025-12-13]
// Description: Additive blending mode

pub fn get(
	idat: &mut Vec<u8>,
	idat_bg: Vec<u8>,
	idat_fg: Vec<u8>,
) {
	let size = idat.len();

	for (i, cell_v) in idat.iter_mut().enumerate().take(size) {
		*cell_v = idat_bg[i].saturating_add(idat_fg[i]);
	}
}
