use io_ops::toml::strings;

use std::fs;
use std::io;
use std::io::Write;
use std::path::Path;

//▒▒▒▒▒▒▒▒▒▒▒▒ INPUT ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn new_line_io(
	prompt_str: &str,
	ui_el: &strings::ui_elements::Stuff,
) -> String {
	let mut input = String::new();

	//Print a prompt message
	println!("{}", prompt_str);

	//Print "ready" symbol
	print!("{}", ui_el.prompt1);
	let _ = io::stdout().flush();

	//Read the input
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CONFIRMATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}
