use crate::UI;
use colored::{Color, Colorize};
use game_options::OPTIONS;
use io_ops::readron::{palettes, strings};
use std::io;
use std::io::Write;

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
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
				match OPTIONS.use_textwrap{
					true => {s1 = fill(&s1, Options::new(term_width));},
					false => {}
				}
				match color_good{
					true => {println!("{}", s1.color($text_col));}
					false => {println!("{}", s1);}
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
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
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

// Print a list of vector options (strings)
#[macro_export]
macro_rules! print_list {
	($bul: expr; $text_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
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
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
				let mut s2 = String::new();
				// Print the prompt before the list (same color).
				match OPTIONS.use_textwrap{
					true => {s1 = fill(&s1, Options::new(term_width));},
					false => {}
				}
				match color_good{
					// Put newline before the list itself.
					true => {println!("{}\n", s1.color($text_col));}
					false => {println!("{}\n", s1);}
				}
				// Print the list itself.
				for entry in str_list {
					match OPTIONS.use_textwrap{
						true => {s2 = fill(&entry, Options::new(term_width));},
						false => {s2 = entry.to_string();}
					}
					match color_good{
						true => {println!("{}{}", $bul.color($text_col), s2.color($text_col));}
						false => {println!("{}{}", $bul, s2);}
					}
				}
			}
		})*
	};
}

// Print a list of menu options
#[macro_export]
macro_rules! print_menu {
	($text_col: expr; $lb: expr; $rb: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr, $entries: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self){
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
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
				let mut s2 = String::new();
				// Print the prompt before the list (same color).
				match OPTIONS.use_textwrap{
					true => {s1 = fill(&s1, Options::new(term_width));},
					false => {}
				}
				match color_good{
					// Put newline before the list itself.
					true => {println!("{}\n", s1.color($text_col));}
					false => {println!("{}\n", s1);}
				}
				// Print the list itself.
				for (i, entry) in $entries.iter().enumerate() {
					let i = (i + 1).to_string(); // Start from 1
					// Check for string in .ron file.
					match self.s.contains_key(&entry.to_string()) {
						true => { s2 = self.s[&entry.to_string()].to_string(); },
						false => {
							crate::print_error(entry.to_string());
							std::process::exit(0);
						}
					}

					match OPTIONS.use_textwrap{
						true => {s2 = fill(&self.s[&entry.to_string()], Options::new(term_width));},
						false => {s2 = self.s[&entry.to_string()].to_string();}
					}
					match color_good{
						true => {
							println!("{}{}{}{}",
								$lb.color($text_col), // Left bracket.
								i.color($text_col),
								$rb.color($text_col), // Right bracket.
								s2.color($text_col)
							);
						}
						false => {println!("{}{}{}{}", $lb, i, $rb, s2);}
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
				// Check for string in .ron file.
				match self.s.contains_key($str_name) {
					true => { },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
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
						let chars_half = chars.substring(0, width/2-1).to_string();
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
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
				s1
			}
		})*
	};
}

// Print out numeric progress from within a loop.
#[macro_export]
macro_rules! print_progress {
	($text_col: expr; $val_col: expr;
	$struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name<T>(&self, curr: T, total: T, step: T)
				where
					//String: From<T>,
					T : std::fmt::Display,
					T : std::string::ToString,
					T : Into<usize>,
					T : Copy
			{
				use colored::{Colorize, Color};
				let term_width = termwidth();
				// Chek if color from preset can be parsed, or fallback.
				let color_res1 : Result<Color, ()> = $text_col.parse();
				let color_res2 : Result<Color, ()> = $val_col.parse();
				// Apply textwrap if enabled in options
				// Check for string in .ron file.
				let mut s1 = String::new();
				match self.s.contains_key($str_name) {
					true => { s1 = self.s[$str_name].to_string(); },
					false => {
						crate::print_error($str_name.to_string());
						std::process::exit(0);
					}
				};
				let mut s2 = String::new();
				// Get the percentage value.
				let intervals = 100 / step.into();
				let print_calls = 100 / intervals;
				for interval in 0..=intervals {
					if curr.into() == interval * total.into() / print_calls {
						let percentage = interval * print_calls;
						s2 = [percentage.to_string(), "%".to_string()].concat();

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
				}
			}
		})*
	};
}
