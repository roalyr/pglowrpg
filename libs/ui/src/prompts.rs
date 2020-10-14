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
	let mut selected_queue = Vec::new();
	let mut priority_queue = Vec::new();

	if input.is_empty() {
		return String::new();
	}

	//Gather all matches
	for entry in options {
		if entry.contains(&input.as_str()) {
			selected_queue.push(entry.to_string());
		}
	}

	//Priority is decided by character appearance in word
	//The earlier - the higher the priority
	for entry in selected_queue.iter().by_ref() {
		let offset = entry.find(input).unwrap_or(entry.len());
		let priority = entry.clone().drain(..offset).count();
		priority_queue.push(priority);
		//println!("{} {:?}", entry, priority);
	}

	//Pick the highest priority one
	let min = priority_queue.iter().min().unwrap_or(&0);
	let index =
		priority_queue.iter().position(|x| x == min).unwrap_or(0);

	if selected_queue.is_empty() {
		return String::new();
	} else {
		selected_queue[index].clone()
	}

	//if entry.contains(&input.as_str()) {
	//output = entry.to_string();
	//}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ CONFIRMATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}
