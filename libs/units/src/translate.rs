// Return absolute value from relative.
pub fn get_abs(
	val_rel: f32,
	val_cap: f32, // Maximum possible value (like 255.0).
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val_rel / val_cap * (abs_max - abs_min) + abs_min
}

// Return relative from absolute.
pub fn get_rel(
	val_abs: f32,
	val_cap: f32,
	abs_min: f32,
	abs_max: f32,
) -> f32 {
	val_cap * (val_abs - abs_min) / (abs_max - abs_min)
}

// A table that returns a value of power x from y = 2^x.
// Used in storing region size as u8.
pub fn get_pow_2_size(thing_size: usize) -> u8 {
	match thing_size {
		0..=2 => 1,
		3..=4 => 2,
		5..=8 => 3,
		9..=16 => 4,
		17..=32 => 5,
		33..=64 => 6,
		65..=128 => 7,
		129..=256 => 8,
		257..=512 => 9,
		513..=1024 => 10,
		1025..=2048 => 11,
		2049..=4096 => 12,
		4097..=8192 => 13,
		8193..=16_384 => 14,
		16_385..=32_768 => 15,
		32_769..=65_536 => 16,
		65_537..=131_072 => 17,
		131_073..=262_144 => 18,
		262_145..=524_288 => 19,
		524_289..=1_048_576 => 20,
		1_048_577..=2_097_152 => 21,
		2_097_153..=4_194_304 => 22,
		4_194_305..=8_388_608 => 23,
		8_388_609..=16_777_216 => 24,
		16_777_217..=33_554_432 => 25,
		33_554_433..=67_108_864 => 26,
		67_108_865..=134_217_728 => 27,
		134_217_729..=268_435_456 => 28,
		_ => panic!("thing size is greater than 2^28"),
	}
}
