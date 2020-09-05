use std::io;
use std::io::Write;

pub fn new_line_io(prompt_str: &str) -> String {
	let mut input = String::new();
	println!("{}", prompt_str);
	print!(" >> ");
	let _ = io::stdout().flush();
	io::stdin().read_line(&mut input).unwrap();
	print!("{}", input.trim().to_string());
	let _ = io::stdout().flush();
	input.trim().to_string()
}

pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}
