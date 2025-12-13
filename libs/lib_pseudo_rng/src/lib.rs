// [Revision: v0.1] [Path: libs/lib_pseudo_rng/src/lib.rs] [Date: 2025-12-13]
// Description: Deterministic pseudo-random number generation wrapper

use dep::oorandom::Rand32;

/// Generate a random float in range [min, max] using seed and iteration.
/// Deterministic: same seed + iter always produces same result.
pub fn get(
	min: f32,
	max: f32,
	seed: u32,
	iter: usize,
) -> f32 {
	let mut random = Rand32::new_inc(seed as u64, iter as u64);
	random.rand_float() * (max - min) + min
}
