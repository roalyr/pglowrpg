use oorandom::Rand32;

pub fn get(
	min: f32,
	max: f32,
	seed: u32,
	iter: usize,
) -> f32 {
	let mut random = Rand32::new_inc(seed as u64, iter as u64);
	random.rand_float() * (max - min) + min
}
