pub fn percent_step(count: usize, total: usize, step: usize) {
	for k in 0..=step {
		if count == k * total / 20 {
			let per = k * 100 / step;
			println!("...{}%", per);
		}
	}
}
