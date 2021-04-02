pub mod game_str;
pub mod menu_str;
pub mod ui_str;
pub mod worldgen_str;
use io_ops::readron::{options, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;

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
//PARAGRAPHS
#[macro_export]
macro_rules! print_paragraph {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				println!("{}", fill(
					&self.s[$str_name].replace(&['\n', '\t',][..], ""),
					Options::new(termwidth())
					)//fill
				);//print
			}//fn
		})*//impl
	};//macro
}

//PARAGRAPHS WITH ESCAPE
#[macro_export]
macro_rules! print_paragraph_with_escape {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				println!("{}", fill(
					&self.s[$str_name],
					Options::new(termwidth())
					)//fill
				);//print
			}//fn
		})*//impl
	};//macro
}

//PARAGRAPHS COLORED FG
#[macro_export]
macro_rules! print_paragraph_with_color {
	($struct_name: ident($($fn_name: ident, $str_name: expr, $fg: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				use colored::Colorize;
				println!("{}", fill(
					&self.s[$str_name].replace(&['\n', '\t',][..], ""),
					Options::new(termwidth())
					).truecolor($fg.0, $fg.1, $fg.2)
				);//print
			}//fn
		})*//impl
	};//macro
} //macro rules

#[macro_export]
//PARAGRAPHS WITH VARIABLE
macro_rules! print_paragraph_with_var {
	($struct_name: ident($($fn_name: ident, $str_name: expr, $type: ty);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self, x: &$type) {
				println!("{} {}", fill(
					&self.s[$str_name].replace(&['\n', '\t',][..], ""),
					Options::new(termwidth())
					), x
				);//print
			}//fn
		})*//impl
	};//macro
} //macro rules

//▒▒▒▒▒▒▒▒▒ RETURN PARAGRAPH ▒▒▒▒▒▒▒▒▒▒
#[macro_export]
macro_rules! return_paragraph {
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
//SEPARATOR
macro_rules! print_separator {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) {
				use terminal_size::{Width, Height, terminal_size};
				let size = terminal_size();
				if let Some((Width(w), Height(h))) = size {
					println!("{}", &self.s[$str_name].repeat(w.into()));
				} else {
					println!("{}", &self.s[$str_name].repeat(50));
				}
			}//fn
		})*//impl
	};//macro
} //macro rules

//BANNER WITH TITLE
#[macro_export]
macro_rules! print_banner_with_title {
	($struct_name: ident($($fn_name: ident, $str_name: expr, $type: ty);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self, title: $type) {
				use terminal_size::{Width, Height, terminal_size};
				let size = terminal_size();
				if let Some((Width(w), Height(h))) = size {
					let width = w as usize - title.len();
					println!("{} {} {}",
						&self.s[$str_name].repeat(width/2 - 1),
						title,
						&self.s[$str_name].repeat(
							// Take into account odd width value
							width/2 - if let 0 = width % 2 {1} else {0}
						)
					);
				} else {
					println!("{} {} {}",
						&self.s[$str_name]
							.repeat(constants_app::TERM_WIDTH_FALLBACK/2 - 1),
						title,
						&self.s[$str_name].repeat(
							// Take into account odd width value
							constants_app::TERM_WIDTH_FALLBACK/2
							- if let 0 = constants_app::TERM_WIDTH_FALLBACK % 2 {1} else {0}
						)
					);
				}
			}//fn
		})*//impl
	};//macro
} //macro rules

//BANNER WITH TITLE COLOR
#[macro_export]
macro_rules! print_banner_with_title_color {
	($struct_name: ident($($fn_name: ident, $str_name: expr, $type: ty, $fg: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self, title: $type) {
				use colored::Colorize;
				use terminal_size::{Width, Height, terminal_size};
				let size = terminal_size();
				if let Some((Width(w), Height(h))) = size {
					let width = w as usize - title.len();
					println!("{} {} {}",
						&self.s[$str_name].repeat(width/2 - 1).truecolor($fg.0, $fg.1, $fg.2),
						title.truecolor($fg.0, $fg.1, $fg.2),
						&self.s[$str_name].repeat(
							// Take into account odd width value
							width/2 - if let 0 = width % 2 {1} else {0}
						).truecolor($fg.0, $fg.1, $fg.2)
					);
				} else {
					// No color here since it is a fallback, which means terminal
					// is not so good to support advanced stuff.
					println!("{} {} {}",
						&self.s[$str_name]
							.repeat(constants_app::TERM_WIDTH_FALLBACK/2 - 1),
						title,
						&self.s[$str_name].repeat(
							// Take into account odd width value
							constants_app::TERM_WIDTH_FALLBACK/2
							- if let 0 = constants_app::TERM_WIDTH_FALLBACK % 2 {1} else {0}
						)
					);
				}
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
