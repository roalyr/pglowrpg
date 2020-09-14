pub mod toml;
pub mod writepng;

use std::fs::File;
use std::io::Read;

pub fn file_to_string(path_vec: &Vec<std::path::PathBuf>) -> String {
	let mut data = String::new();

	for path in path_vec.iter() {
		//Find the file
		match File::open(&path) {
			Ok(mut file) => {
				match file.read_to_string(&mut data) {
					Ok(f) => f,
					Err(e) => {
						println!(
							"{}: {}",
							e.to_string(),
							path.to_str().unwrap()
						);
						std::process::exit(0);
					}
				};
				break;
			}
			Err(_) => {
				//Suppress message at this point
			}
		};
	}

	//Throw an error if no file was opened at all
	for path in path_vec.iter() {
		if data.is_empty() {
			match File::open(&path) {
				Ok(_) => {}
				Err(e) => {
					println!(
						"{}: {}",
						e.to_string(),
						path.to_str().unwrap()
					);
					std::process::exit(0);
				}
			};
		}
	}

	data
}

pub fn create_dir(path: &std::path::PathBuf) {
	match std::fs::create_dir(path) {
		Ok(()) => {}
		Err(e) => match e.kind() {
			std::io::ErrorKind::AlreadyExists => {}
			_ => {
				println!(
					"{}: {}",
					e.to_string(),
					path.to_str().unwrap()
				);
				std::process::exit(0);
			}
		},
	}
}

pub fn create_file_overwrite(path: &std::path::PathBuf) -> File {
	match File::create(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	}
}
