pub mod readron;
pub mod writepng;

use lz4;
use lz4::{Decoder, EncoderBuilder};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

//▒▒▒▒▒▒▒▒▒▒▒▒ FILESYSTEM ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn dir_file_contents(
	path_str: &str,
	seek_extension: &str,
) -> Vec<String> {
	let path = Path::new(path_str);

	//Get directory contents
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	//Make a vector for paths and names
	let mut contents = Vec::new();

	for entry in contents_iter {
		let entry_unwrapped = entry.unwrap().path();
		let entry_name = &entry_unwrapped.file_stem().unwrap().to_str().unwrap();
		let entry_extension =
			&entry_unwrapped.extension().unwrap().to_str().unwrap();

		if *entry_extension == seek_extension {
			contents.push(entry_name.to_string());
		}
	}
	contents
}

pub fn dir_dir_contents(
	path_str: &str,
	prefix: &str,
	separator: &str,
) -> (String, Vec<String>) {
	let path = Path::new(path_str);

	//Get directory contents
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	//Make a vector for paths
	let mut contents_paths = Vec::new();

	//Make a neat string
	let mut contents_str = "".to_owned();

	for entry in contents_iter {
		let entry_unwrapped = entry.unwrap().path();

		let entry_name = &entry_unwrapped.file_stem().unwrap().to_str().unwrap();

		//Clone raw data to vec
		contents_paths.push(entry_name.to_string());

		contents_str.push_str(prefix);
		contents_str.push_str(entry_name);
		contents_str.push_str(separator);
	}

	(contents_str, contents_paths)
}

pub fn file_to_string(path_vec: &Vec<std::path::PathBuf>) -> String {
	let mut data = String::new();

	for path in path_vec.iter() {
		//Find the file
		match File::open(&path) {
			Ok(mut file) => {
				match file.read_to_string(&mut data) {
					Ok(f) => f,
					Err(e) => {
						println!("{}: {}", e.to_string(), path.to_str().unwrap());
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
					println!("{}: {}", e.to_string(), path.to_str().unwrap());
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
				println!("{}: {}", e.to_string(), path.to_str().unwrap());
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

//▒▒▒▒▒▒▒▒▒▒▒▒ COMPRESSION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn compress_to_storage(
	data: Vec<u8>,
	path: &std::path::PathBuf,
) {
	let output_file = match File::create(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	let mut encoder = match EncoderBuilder::new().level(4).build(output_file) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	//Using Cursor because it has Read trait
	match io::copy(&mut Cursor::new(data), &mut encoder) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	match encoder.finish().1 {
		Ok(()) => {}
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	}
}

pub fn decompress_to_memory(path: &std::path::PathBuf) -> Vec<u8> {
	let input_file = match File::open(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	let mut decoder = match Decoder::new(input_file) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	let mut output = Vec::new();
	match io::copy(&mut decoder, &mut output) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	output
}
