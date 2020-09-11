pub mod toml;
pub mod writepng;

use std::fs::File;
use std::io::Read;

pub fn file_to_string(path: &std::path::PathBuf) -> String {
	let mut file = match File::open(&path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	let mut data = String::new();
	match file.read_to_string(&mut data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	data
}
