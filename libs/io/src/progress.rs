pub fn five_percent_step(
	count: usize,
	total: usize,
) {
	let step = 20;
	for k in 0..=step {
		if count == k * total / 20 {
			let per = k * 100 / step;
			println!("...{}%", per);
		}
	}
}
