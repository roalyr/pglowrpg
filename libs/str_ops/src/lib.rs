pub mod game_str;
pub mod menu_str;
pub mod ui_str;
pub mod worldgen_str;
use colored::{Color, Colorize};
use io_ops::readron::palettes;
use io_ops::readron::{options, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;
use std::io;
use std::io::Write;

//▒▒▒▒▒▒▒▒▒▒▒▒ INIT ▒▒▒▒▒▒▒▒▒▒▒▒▒
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
			s: strings::worldgen_strings::get(&options::get().locale),
		}
	};
	pub static ref GS: GmStrings = {
		GmStrings {
			s: strings::game_strings::get(&options::get().locale),
		}
	};
	pub static ref MS: MnStrings = {
		MnStrings {
			s: strings::menu_strings::get(&options::get().locale),
		}
	};
	pub static ref UI: UiStrings = {
		UiStrings {
			s: strings::ui_strings::get(&options::get().locale),
		}
	};
}

//▒▒▒▒▒▒▒▒▒▒ PRINT PARAGRAPH ▒▒▒▒▒▒▒▒▒▒
#[macro_export]
macro_rules! print_paragraph {
	// A simple case without variables
	($exclude: expr; $text_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				use colored::{Colorize, Color};
				// Chek if color from preset can be parsed, or fallback.
				let color_res : Result<Color, ()> = $text_col.parse();
				match color_res{
					Ok(col) => {println!("{}", fill(&self.s[$str_name]
						.replace(&$exclude[..], ""), Options::new(termwidth())).color(col));}
					Err(_) => {println!("{}", fill(&self.s[$str_name]
						.replace(&$exclude[..], ""), Options::new(termwidth())));}
				}
			}
		})*
	};
	// With variables
	($exclude: expr; $text_col: expr; $val_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name<T>(&self, x_gen: T)
				where
					//String: From<T>,
					T : std::fmt::Display,
					T : std::string::ToString,
			{
				let x = x_gen.to_string();
				use colored::{Colorize, Color};
				// Chek if color from preset can be parsed, or fallback.
				let color_res1 : Result<Color, ()> = $text_col.parse();
				let color_res2 : Result<Color, ()> = $val_col.parse();
				let s1 = fill(&self.s[$str_name].replace(&$exclude[..], ""), Options::new(termwidth()));
				let s2 = x.to_owned();
				let mut color1_good = false;
				let mut color2_good = false;
				match color_res1 {
					Ok(_) => {color1_good = true;}
					Err(_) => {}
				}
				match color_res2 {
					Ok(_) => {color2_good = true;}
					Err(_) => {}
				}
				match color1_good{
					// Added space after {}.
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

//▒▒▒▒▒▒▒▒▒ RETURN STRING ▒▒▒▒▒▒▒▒▒▒
#[macro_export]
macro_rules! return_string {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) -> String {
				self.s[$str_name].replace(&['\n', '\t',][..], "")
			}//fn
		})*//impl
	};//macro
} //macro rules

//▒▒▒▒▒▒▒▒▒▒ PRINT BANNERS ▒▒▒▒▒▒▒▒▒▒▒
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
				let title = title_gen.to_string();
				use terminal_size::{Width, Height, terminal_size};
				use colored::{Colorize, Color};
				// Chek if color from preset can be parsed, or fallback.
				let color_res1 : Result<Color, ()> = $fg.parse();
				let mut color1_good = false;
				match color_res1 {
					Ok(_) => {color1_good = true;}
					Err(_) => {}
				}
				let mut term_width = 0;
				let mut width = 0;
				// If term size can be found
				if let Some((Width(w), Height(h))) = terminal_size() {
					term_width = termwidth();
					width = w as usize - title.len();
				// If term size can't be found
				} else {
					term_width = constants_app::TERM_WIDTH_FALLBACK;
					width = term_width - title.len();
				}
				match title.len(){
					// If no title - just draw a separator
					0 => {
						match color1_good {
							true =>{println!("{}", &self.s[$str_name]
								.repeat(term_width.into()).color($fg));
							},
							false =>{println!("{}", &self.s[$str_name]
								.repeat(term_width.into()));
							},
						}
					},
					// If title is shorter than screen width and can fit in
					x if x < (term_width-2).into() => {
						match color1_good {
							true =>{
								println!("{} {} {}",
									&self.s[$str_name].repeat(width/2 - 1).color($fg),
									title.color($fg),
									&self.s[$str_name].repeat(
										// Take into account odd width value
										width/2 - if let 0 = width % 2 {1} else {0}
									).color($fg)
								);
							},
							false =>{
								println!("{} {} {}",
									&self.s[$str_name].repeat(width/2 - 1),
									title,
									&self.s[$str_name].repeat(
										// Take into account odd width value
										width/2 - if let 0 = width % 2 {1} else {0}
									)
								);
							},
						}
					},
					// Otherwise text-wrap it
					_ => {
						match color1_good {
							true =>{
								println!("{}", &self.s[$str_name].repeat(term_width.into()).color($fg));
								println!("{}", fill(&title, Options::new(termwidth())).color($fg));
								println!("{}", &self.s[$str_name].repeat(term_width.into()).color($fg));
							},
							false =>{
								println!("{}", &self.s[$str_name].repeat(term_width.into()));
								println!("{}", fill(&title, Options::new(termwidth())));
								println!("{}", &self.s[$str_name].repeat(term_width.into()));
							},
						}
					}
				} // match title len
			}//fn
		})*//impl
	};//macro
} //macro rules

//▒▒▒▒▒▒▒▒▒▒ RETURN BANNERS ▒▒▒▒▒▒▒▒▒▒▒
#[macro_export]
macro_rules! return_banner {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) -> String{
				use terminal_size::{Width, Height, terminal_size};
				let size = terminal_size();
				if let Some((Width(w), Height(h))) = size {
					self.s[$str_name].repeat(w.into())
				} else {
					self.s[$str_name].repeat(constants_app::TERM_WIDTH_FALLBACK)
				}
			}//fn
		})*//impl
	};//macro
} //macro rules

//▒▒▒▒▒▒▒▒▒▒▒▒ INPUT ▒▒▒▒▒▒▒▒▒▒▒▒▒
// Prompts start after empty newline.
fn new_line_input(prompt_symbol: &String) -> String {
	let mut input = String::new();
	print!(
		"\n{}",
		prompt_symbol.color(palettes::text_colors::get().prompt)
	);
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

//▒▒▒▒▒▒▒▒▒▒▒▒ CONFIRMATION ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub fn selected(
	prompt: &str,
	input: &str,
) {
	println!("{}", [&prompt, "\"", &input, "\""].concat());
}
