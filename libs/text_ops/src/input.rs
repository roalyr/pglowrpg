use crate::UI;
use colored::{Color, Colorize};
use game_options::OPTIONS;
use io_ops::readron::palettes;
use std::io;
use std::io::Write;

// Prompts start after empty newline.
fn new_line_input(prompt_symbol: &str) -> String {
	let mut input = String::new();
	// Chek if color from preset can be parsed, or fallback.
	let text_col = palettes::text_colors::get().prompt;
	let color_res: Result<Color, ()> = text_col.parse();
	let mut color_good = false;
	if color_res.is_ok() && OPTIONS.use_text_colors {
		color_good = true;
	}
	match color_good {
		true => {
			print!("\n{}", prompt_symbol.color(text_col));
		}
		false => {
			print!("\n{}", prompt_symbol);
		}
	}
	let _ = io::stdout().flush();
	match io::stdin().read_line(&mut input) {
		Ok(_) => {}
		Err(e) => {
			println!("ERROR: input failed: {}", e.to_string());
			panic!();
		}
	}
	println!(); // Empty line after input.
	input.trim().to_string()
}

// Returns just the input as is.
pub fn prompt_option() -> String {
	new_line_input(&UI.s["prompt_option"])
}

// Returns just the input as is.
pub fn prompt_number() -> String {
	new_line_input(&UI.s["prompt_number"])
}

// Returns input but only if it matvhes the list of allowed words.
pub fn prompt_word(allowed_words: &Vec<String>) -> String {
	// If allowed words is empty return as is.
	if allowed_words.is_empty() {
		return new_line_input(&UI.s["prompt_word"]);
	}
	let input = new_line_input(&UI.s["prompt_word"]);
	if input.is_empty() {
		return String::new();
	}
	let mut selected_queue = Vec::new();
	let mut priority_queue = Vec::new();
	//Gather all matches
	for entry in allowed_words {
		if entry.contains(&input.as_str()) {
			selected_queue.push(entry.to_string());
		}
	}
	//Priority is decided by character appearance in word
	//The earlier - the higher the priority
	for entry in selected_queue.iter().by_ref() {
		let word_len = entry.len();
		let offset = entry.find(&input).unwrap_or(word_len);
		let priority = entry.clone().drain(..offset).count();
		priority_queue.push(priority);
		//println!("{} {:?}", entry, priority);
	}
	//Pick the highest priority one
	let min = priority_queue.iter().min().unwrap_or(&0);
	let index = priority_queue.iter().position(|x| x == min).unwrap_or(0);
	if selected_queue.is_empty() {
		String::new()
	} else {
		selected_queue[index].clone()
	}
}

// Return an input according to specifics. Execute specific functions
// before input (print prompts, for instance).
#[macro_export]
macro_rules! prompt_input {
	// If word list is supplied the prompt_word will be used
	($flag: expr;  $word_list: expr; $b: block) => {
		{
			use text_ops::input::prompt_word;
			let mut input = String::new();
			match OPTIONS.repeat_text_if_no_input{
				// Keep pestering the player to no end
				true => {
					while input.is_empty() {
						// Call all the suggested functions (prompts)
						$b
						match $flag {
							"word" => {input = prompt_word($word_list);},
							_ => {
								text_ops::input_flag_error_word($flag.to_string());
								panic!();
							}
						}
					}
				},
				// Or ask once.
				false => {
					// Call all the suggested functions (prompts)
					$b
					match $flag {
						"word" => {input = prompt_word($word_list);},
						_ => {
							text_ops::input_flag_error_word($flag.to_string());
							panic!();
						}
					}
				}
			}
			input
		}
	};
	// For number or option prompt.
	($flag: expr; $b: block) => {
		{
			use text_ops::input::prompt_option;
			use text_ops::input::prompt_number;
			let mut input = String::new();
			match OPTIONS.repeat_text_if_no_input{
				// Keep pestering the player to no end
				true => {
					while input.is_empty() {
						// Call all the suggested functions (prompts)
						$b
						match $flag {
							"num" => {input = prompt_number();},
							"opt" => {input = prompt_option();},
							_ => {
								text_ops::input_flag_error($flag.to_string());
								panic!();
							}
						}
					}
				},
				// Or ask once.
				false => {
					// Call all the suggested functions (prompts)
					$b
					match $flag {
						"num" => {input = prompt_number();},
						"opt" => {input = prompt_option();},
						_ => {
							text_ops::input_flag_error($flag.to_string());
							panic!();
						}
					}
				}
			}
			input
		}
	};
}
