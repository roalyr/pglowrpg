pub mod game_str;
pub mod menu_str;
pub mod ui_str;
pub mod worldgen_str;
use colored::{Color, Colorize};
use io_ops::readron::{palettes, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::Write;

use game_options::OPTIONS;

//▒▒▒▒▒▒▒▒▒▒▒▒ LOCALE ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub struct WgStrings {
	s: HashMap<String, String>,
}
pub struct GmStrings {
	s: HashMap<String, String>,
}
pub struct MnStrings {
	s: HashMap<String, String>,
}
pub struct UiStrings {
	s: HashMap<String, String>,
}
lazy_static! {
	pub static ref WS: WgStrings = {
		WgStrings {
			s: strings::worldgen_strings::get(&OPTIONS.locale),
		}
	};
	pub static ref GS: GmStrings = {
		GmStrings {
			s: strings::game_strings::get(&OPTIONS.locale),
		}
	};
	pub static ref MS: MnStrings = {
		MnStrings {
			s: strings::menu_strings::get(&OPTIONS.locale),
		}
	};
	pub static ref UI: UiStrings = {
		UiStrings {
			s: strings::ui_strings::get(&OPTIONS.locale),
		}
	};
}

//▒▒▒▒▒▒▒▒▒▒ PRINT MACROS ▒▒▒▒▒▒▒▒▒▒
// Print a block of text with different options.
#[macro_export]
macro_rules! print_paragraph {
	// A simple case without variables
	($text_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				use colored::{Colorize, Color};
				let term_width = termwidth();
				// Chek if color from preset can be parsed, or fallback.
				let color_res : Result<Color, ()> = $text_col.parse();
				let mut color_good = false;
				match color_res {
					Ok(_) => {
						if OPTIONS.use_text_colors {color_good = true;}
				}
					Err(_) => {}
				}
				let mut s = self.s[$str_name].clone();
				match OPTIONS.use_textwrap{
					true => {s = fill(&s, Options::new(term_width));},
					false => {}
				}
				match color_good{
					true => {println!("{}", s.color($text_col));}
					false => {println!("{}", s);}
				}
			}
		})*
	};
	// With variables
	($text_col: expr; $val_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name<T>(&self, x_gen: T)
				where
					//String: From<T>,
					T : std::fmt::Display,
					T : std::string::ToString,
			{
				use colored::{Colorize, Color};
				let term_width = termwidth();
				let x = x_gen.to_string();
				// Chek if color from preset can be parsed, or fallback.
				let color_res1 : Result<Color, ()> = $text_col.parse();
				let color_res2 : Result<Color, ()> = $val_col.parse();
				// Apply textwrap if enabled in options
				let mut s1 = self.s[$str_name].clone();
				let mut s2 = x.to_owned();
				match OPTIONS.use_textwrap{
					true => {
						s1 = fill(&s1, Options::new(term_width));
						s2 = fill(&s2, Options::new(term_width));
					},
					false => {}
				}
				let mut color1_good = false;
				let mut color2_good = false;
				match color_res1 {
					Ok(_) => {
						if OPTIONS.use_text_colors {color1_good = true;}
				}
					Err(_) => {}
				}
				match color_res2 {
					Ok(_) => {
						if OPTIONS.use_text_colors {color2_good = true;}
				}
					Err(_) => {}
				}
				match color1_good{
					// Added space after {} because value goes after.
					true => {print!("{} ", s1.color($text_col));},
					false => {print!("{} ", s1);},
				}
				match color2_good{
					true => {println!("{}", s2.color($val_col));},
					false => {println!("{}", s2);},
				}
			}
		})*
	};
}

// Print a list of options
#[macro_export]
macro_rules! print_list {
	($bul: expr; $text_col: expr;
	$struct_name: ident($($fn_name: ident);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self, str_list: &Vec<String>){
				use colored::{Colorize, Color};
				let term_width = termwidth();
				// Chek if color from preset can be parsed, or fallback.
				let color_res : Result<Color, ()> = $text_col.parse();
				let mut color_good = false;
				match color_res {
					Ok(_) => {
						if OPTIONS.use_text_colors {color_good = true;}
				}
					Err(_) => {}
				}
				let mut s = String::new();
				println!("");
				for entry in str_list {
					match OPTIONS.use_textwrap{
						true => {s = fill(&entry, Options::new(term_width));},
						false => {s = entry.to_string();}
					}
					match color_good{
						true => {println!("{}{}", $bul.color($text_col), s.color($text_col));}
						false => {println!("{}{}", $bul, s);}
					}
				}
			}
		})*
	};
}

// Print a banner (a row of symbols) with optional title.
#[macro_export]
macro_rules! print_banner {
	($fg: expr; $struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name<T>(&self, title_gen: T)
				where
					T : std::fmt::Display,
					T : std::string::ToString,
			{
				use colored::{Colorize, Color};
				use substring::Substring;
				let title = title_gen.to_string();
				let term_width = termwidth();
				let width = (term_width as usize).saturating_sub(title.len());
				// Chek if color from preset can be parsed, or fallback.
				let color_res1 : Result<Color, ()> = $fg.parse();
				let mut color1_good = false;
				match color_res1 {
					Ok(_) => {
						if OPTIONS.use_text_colors {color1_good = true;}
				}
					Err(_) => {}
				}
				match title.len(){
					// If no title - just draw a separator
					0 => {
						// Take into account odd width value
						let chars = self.s[$str_name].clone().repeat(width);
						let chars = chars.substring(0, width);
						match color1_good {
							true =>{ println!("{}", chars.color($fg)); },
							false =>{ println!("{}", chars); },
						}
					},
					// If title is shorter than screen width and can fit in
					x if x < (term_width-2).into() => {
						// Take into account odd width value
						let chars = self.s[$str_name].clone().repeat(width);
						let mut chars_half = chars.substring(0, width/2-1).to_string();
						match color1_good {
							true =>{
								println!("{} {} {}",
									chars_half.color($fg),
									title.color($fg),
									{ // Odd width compensation
										let mut ch = chars_half.clone();
										if width %2 != 0 { ch += &self.s[$str_name]; }
										ch.color($fg)
									}
								);
							},
							false =>{
								println!("{} {} {}", chars_half, title,
									{ // Odd width compensation
										let mut ch = chars_half.clone();
										if width %2 != 0 { ch += &self.s[$str_name]; }
										ch
									}
								);
							},
						}
					},
					// Otherwise text-wrap it
					_ => {
						// Take into account odd width value
						let chars = self.s[$str_name].clone().repeat(width);
						let chars = chars.substring(0, width);
						// Check for textwrap option
						let mut s = title;
						match OPTIONS.use_textwrap {
							true => { s = fill(&s, Options::new(term_width)); },
							false => {}
						}
						match color1_good {
							true =>{
								println!("{}", chars.color($fg));
								println!("{}", s.color($fg));
								println!("{}", chars.color($fg));
							},
							false =>{
								println!("{}", chars);
								println!("{}", s);
								println!("{}", chars);
							},
						}
					}
				}
			}
		})*
	};
}

// Doesn't print, but returns a string for some special cases.
#[macro_export]
macro_rules! return_string {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) -> String {
				self.s[$str_name].clone()
			}
		})*
	};
}

//▒▒▒▒▒▒▒▒▒▒▒▒ INPUT ▒▒▒▒▒▒▒▒▒▒▒▒▒
// Prompts start after empty newline.
fn new_line_input(prompt_symbol: &String) -> String {
	let mut input = String::new();
	// Chek if color from preset can be parsed, or fallback.
	let text_col = palettes::text_colors::get().prompt;
	let color_res: Result<Color, ()> = text_col.parse();
	let mut color_good = false;
	match color_res {
		Ok(_) => {
			if OPTIONS.use_text_colors {
				color_good = true;
			}
		}
		Err(_) => {}
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
	io::stdin().read_line(&mut input).unwrap();
	input.trim().to_string()
}

// Returns just the input as is.
pub fn prompt_option() -> String {
	new_line_input(&UI.s["prompt_option"])
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
		let offset = entry.find(&input).unwrap_or(entry.len());
		let priority = entry.clone().drain(..offset).count();
		priority_queue.push(priority);
		//println!("{} {:?}", entry, priority);
	}
	//Pick the highest priority one
	let min = priority_queue.iter().min().unwrap_or(&0);
	let index = priority_queue.iter().position(|x| x == min).unwrap_or(0);
	if selected_queue.is_empty() {
		return String::new();
	} else {
		selected_queue[index].clone()
	}
}

// Return an input according to specifics. Execute specific functions
// before input (print prompts, for instance).
#[macro_export]
macro_rules! prompt_input {
	// If word list is supplied the prompt_word will be used
	($word_list: expr; $b: block) => {
		{
			use text_ops::prompt_word;
			let mut input = String::new();
			match OPTIONS.repeat_text_if_no_input{
				// Keep pestering the player to no end
				true => {
					while input.is_empty() {
						// Call all the suggested functions (prompts)
						$b
						input = prompt_word($word_list);
					}
				},
				// Or ask once.
				false => {
					// Call all the suggested functions (prompts)
					$b
					input = prompt_word($word_list);
				}
			}
			input
		}
	};
	// If no word list is supplied the prompt_word will be used
	($b: block) => {
		{
			use text_ops::prompt_option;
			let mut input = String::new();
			match OPTIONS.repeat_text_if_no_input{
				// Keep pestering the player to no end
				true => {
					while input.is_empty() {
						// Call all the suggested functions (prompts)
						$b
						input = prompt_option();
					}
				},
				// Or ask once.
				false => {
					// Call all the suggested functions (prompts)
					$b
					input = prompt_option();
				}
			}
			input
		}
	};
}

//TODO add tw
//▒▒▒▒▒▒▒▒▒▒▒▒ CONFIRMATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn selected(
	// Make a macro for different colors?
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}

pub fn print_progress(
	count: usize,
	total: usize,
	step: usize,
) {
	for k in 0..=step {
		if count == k * total / 20 {
			let per = k * 100 / step;
			println!("...{}%", per); //color of numbers
		}
	}
}
