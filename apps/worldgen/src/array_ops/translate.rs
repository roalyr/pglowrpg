pub fn get_abs(
	val: f32,
	val_cap: f32,
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val / val_cap * (abs_max - abs_min) + abs_min
}

pub fn get_rel(
	val_abs: f32,
	val_cap: f32,
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val_cap * (val_abs - abs_min) / (abs_max - abs_min)
}
