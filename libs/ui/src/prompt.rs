use io_ops::toml::strings;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

pub fn new_line_io(prompt_str: &str) -> String {
	let mut input = String::new();

	//Print a prompt message
	println!("{}", prompt_str);

	//Print "ready" symbol
	print!(" >> ");
	let _ = io::stdout().flush();

	//Read the input
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}

pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}

pub fn dir_contents(
	path_str: &str,
	seek_extension: &str,
	separator: &str,
	_panic_str: &strings::panic_strings::Stuff,
) -> String {
	let path = Path::new(path_str);

	//Get directory contents
	let contents_iter = match fs::read_dir(path) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};

	//Make a neat string
	let mut contents_str = "".to_owned();

	for entry in contents_iter {
		let entry_unwrapped = entry.unwrap().path();
		let entry_name =
			&entry_unwrapped.file_stem().unwrap().to_str().unwrap();
		let entry_extension =
			&entry_unwrapped.extension().unwrap().to_str().unwrap();

		if *entry_extension == seek_extension {
			contents_str.push_str(entry_name);
			contents_str.push_str(separator);
		}
	}
	contents_str
}
