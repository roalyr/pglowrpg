use std::io;
use std::io::Write;

//▒▒▒▒▒▒▒▒▒▒▒▒ INPUT ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn new_line_io(
	prompt_str: &str,
	prompt_sym: &str,
) -> String {
	let mut input = String::new();

	//Print a prompt message
	print!("{}", prompt_str);

	//Print "ready" symbol
	print!("{}", prompt_sym);
	let _ = io::stdout().flush();

	//Read the input
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}

pub fn autocomplete(
	input: &String,
	options: &Vec<String>,
) -> String {
	//This will allow to enter parts of the name optionally
	//Basically an autocomplete
	let mut output = String::new();
	for entry in options {
		if entry.contains(&input.as_str()) {
			output = entry.to_string();
		}
	}
	output
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CONFIRMATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}
