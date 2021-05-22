pub mod readron;

use lz4;
use lz4::{Decoder, EncoderBuilder};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::path::Path;

// TODO: merge those two functions?
// Return the file contents of the directory.
pub fn dir_file_contents(
	path_str: &str,
	seek_extension: &str,
) -> Vec<String> {
	let path = Path::new(path_str);
	// Get directory contents.
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	// Make a vector for paths.
	let mut contents = Vec::new();
	for entry in contents_iter {
		// TODO: make unwrap_or here?
		let entry_unwrapped = entry.unwrap().path();
		let entry_name = &entry_unwrapped.file_stem().unwrap().to_str().unwrap();
		let entry_extension =
			&entry_unwrapped.extension().unwrap().to_str().unwrap();
		// Gather the files by extension.
		if *entry_extension == seek_extension {
			contents.push(entry_name.to_string());
		}
	}
	contents
}

// Returns the directories within a directory.
pub fn dir_dir_contents(path_str: &str) -> Vec<String> {
	let path = Path::new(path_str);
	// Get directory contents.
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	// Make a vector for paths.
	let mut contents = Vec::new();
	for entry in contents_iter {
		// TODO: unwrap_or?
		let entry_unwrapped = entry.unwrap().path();
		let entry_name = &entry_unwrapped.file_stem().unwrap().to_str().unwrap();
		//Clone raw data to vec
		contents.push(entry_name.to_string());
	}
	contents
}

// Reads file to string.
pub fn file_to_string(path_vec: &Vec<std::path::PathBuf>) -> String {
	let mut data = String::new();
	for path in path_vec.iter() {
		// Find the file (try to).
		match File::open(&path) {
			Ok(mut file) => {
				match file.read_to_string(&mut data) {
					Ok(f) => f,
					Err(e) => {
						println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
						std::process::exit(0);
					}
				};
				// Since we have found the first matching file name, it means
				// that we can break the loop.
				break;
			}
			Err(_) => {}
		};
	}
	// Throw an error if no file was opened at all.
	for path in path_vec.iter() {
		if data.is_empty() {
			match File::open(&path) {
				Ok(_) => {}
				Err(e) => {
					println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
					std::process::exit(0);
				}
			};
		}
	}
	data
}

// Make a directory.
pub fn create_dir(path: &std::path::PathBuf) {
	match std::fs::create_dir(path) {
		Ok(()) => {}
		Err(e) => match e.kind() {
			std::io::ErrorKind::AlreadyExists => {}
			_ => {
				println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
				std::process::exit(0);
			}
		},
	}
}

// Make a new file and overwrite if it exists.
pub fn create_file_overwrite(path: &std::path::PathBuf) -> File {
	match File::create(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ COMPRESSION ▒▒▒▒▒▒▒▒▒▒▒▒▒
// Read the data and write it down to storage in a .lz4
pub fn compress_to_storage(
	data: Vec<u8>,
	path: &std::path::PathBuf,
) {
	let output_file = match File::create(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	let mut encoder = match EncoderBuilder::new().level(4).build(output_file) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	// Using Cursor because it has Read trait.
	match io::copy(&mut Cursor::new(data), &mut encoder) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	match encoder.finish().1 {
		Ok(()) => {}
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	}
}

// Read from storage the .lz4 data and return raw to memory as bytes.
pub fn decompress_to_memory(path: &std::path::PathBuf) -> Vec<u8> {
	let input_file = match File::open(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	let mut decoder = match Decoder::new(input_file) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	let mut output = Vec::new();
	match io::copy(&mut decoder, &mut output) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	output
}
