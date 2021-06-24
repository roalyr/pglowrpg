use std::time::SystemTime;

pub fn get() -> u32 {
	SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_secs() as u32
}
