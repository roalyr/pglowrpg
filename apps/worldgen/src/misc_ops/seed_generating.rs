// [Revision: v0.1] [Path: apps/worldgen/src/misc_ops/seed_generating.rs] [Date: 2025-12-13]
// Description: Random seed generation from system time

use std::time::SystemTime;

pub fn get() -> u32 {
	SystemTime::now()
		.duration_since(SystemTime::UNIX_EPOCH)
		.unwrap()
		.as_secs() as u32
}
