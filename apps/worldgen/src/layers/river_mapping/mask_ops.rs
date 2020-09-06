//use crate::layers::river_mapping::*;

//fields must be continuous
const MASK_FIELD: u16 = 0b_0000_0000_0000_0111;
const DNSTR_FIELD: u16 = 0b_0000_0000_0011_1000;
const UPSTR_FIELD: u16 = 0b_0000_0001_1100_0000;

//Make this into encoder/decoder lib and unufy names, even if
//there are no bit-wise ops going on, for unification.
//▒▒▒▒▒▒▒▒▒▒▒▒ WRITING ▒▒▒▒▒▒▒▒▒▒▒▒▒
//First field
pub fn river_mask_set(
	cell: &mut u16,
	mask_type: u8,
) {
	*cell |= (mask_type as u16) << MASK_FIELD.trailing_zeros();
}

//Second field
pub fn river_downstream_set(
	cell: &mut u16,
	neighbor_index: u8,
) {
	*cell |= (neighbor_index as u16) << DNSTR_FIELD.trailing_zeros();
}

//Third field
pub fn river_upstream_set(
	cell: &mut u16,
	neighbor_index: u8,
) {
	*cell |= (neighbor_index as u16) << UPSTR_FIELD.trailing_zeros();
}

//▒▒▒▒▒▒▒▒▒▒▒▒ READING ▒▒▒▒▒▒▒▒▒▒▒▒▒
//First field
pub fn river_mask_get(cell: u16) -> u8 {
	((cell >> MASK_FIELD.trailing_zeros())
		& 2u16.pow(MASK_FIELD.count_ones()) - 1)
		.to_be_bytes()[1]
}

//Second field
pub fn river_downstream_get(cell: u16) -> u8 {
	((cell >> DNSTR_FIELD.trailing_zeros())
		& 2u16.pow(DNSTR_FIELD.count_ones()) - 1)
		.to_be_bytes()[1]
}

//Third field
pub fn river_upstream_get(cell: u16) -> u8 {
	((cell >> UPSTR_FIELD.trailing_zeros())
		& 2u16.pow(UPSTR_FIELD.count_ones()) - 1)
		.to_be_bytes()[1]
}
