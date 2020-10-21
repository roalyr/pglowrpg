use std::time::SystemTime;

pub fn get() -> usize {
	SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_secs() as usize
}
