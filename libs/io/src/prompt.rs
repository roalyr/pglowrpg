use std::io;
use std::io::Write;

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
