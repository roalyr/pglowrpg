use std::io;
use std::io::Write;
use std::path::Path;
use std::fs;

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

pub fn dir_contents(path_str: &str, separator: &str) -> String {
	let path = Path::new(path_str);
	let contents_iter = fs::read_dir(path).unwrap();
	let mut contents_str = "".to_owned();
	for entry in contents_iter {
		contents_str.push_str(entry.unwrap().path().file_stem().unwrap().to_str().unwrap());
		contents_str.push_str(separator);
	}
	contents_str
}