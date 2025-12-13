// [Revision: v0.1] [Path: libs/lib_io_ops/src/lib.rs] [Date: 2025-12-13]
// Description: File I/O operations and LZ4 compression utilities

pub mod readron;

use dep::lz4::{Decoder, EncoderBuilder};
use std::fs;
use std::fs::File;
use std::io;
use std::io::Cursor;
use std::io::Read;
use std::path::{Path, PathBuf};

/// Return the file names (without extension) in a directory matching an extension.
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
	let mut contents = Vec::new();
	for entry in contents_iter {
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

/// Return full paths of files in a directory matching an extension.
pub fn dir_file_contents_full_paths(
	path_str: &str,
	seek_extension: &str,
) -> Vec<PathBuf> {
	let path = Path::new(path_str);
	// Get directory contents.
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	let mut contents = Vec::new();
	for entry in contents_iter {
		let entry_unwrapped = entry.unwrap().path();
		let entry_extension =
			&entry_unwrapped.extension().unwrap().to_str().unwrap();
		// Gather the files by extension.
		if *entry_extension == seek_extension {
			contents.push(entry_unwrapped);
		}
	}
	contents
}

/// Return subdirectory names within a directory.
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
	let mut contents = Vec::new();
	for entry in contents_iter {
		let entry_unwrapped = entry.unwrap().path();
		let entry_name = &entry_unwrapped.file_stem().unwrap().to_str().unwrap();
		contents.push(entry_name.to_string());
	}
	contents
}

/// Read a single file to string.
pub fn single_file_to_string(path: std::path::PathBuf) -> String {
	let mut data = String::new();
	if let Ok(mut file) = File::open(&path) {
		match file.read_to_string(&mut data) {
			Ok(f) => f,
			Err(e) => {
				println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
				std::process::exit(0);
			}
		};
	};
	data
}

/// Read first matching file from path vector to string.
pub fn file_to_string(path_vec: &Vec<std::path::PathBuf>) -> String {
	let mut data = String::new();
	for path in path_vec.iter() {
		// Find the file (try to).
		if let Ok(mut file) = File::open(&path) {
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

/// Create a directory (no error if already exists).
pub fn create_dir(path: &Path) {
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

/// Create a new file (overwrite if exists).
pub fn create_file_overwrite(path: &Path) -> File {
	match File::create(path) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	}
}

////////////////////////////////////////////////////////////////////////////////
// LZ4 COMPRESSION

/// Compress data and write to storage as .lz4 file.
pub fn compress_to_storage(
	data: Vec<u8>,
	path: &Path,
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

/// Read .lz4 file from storage and decompress to memory.
pub fn decompress_to_memory(path: &Path) -> Vec<u8> {
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
