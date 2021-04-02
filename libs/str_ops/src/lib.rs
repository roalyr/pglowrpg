pub mod game_str;
pub mod menu_str;
pub mod ui_str;
pub mod worldgen_str;
use io_ops::readron::{options, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;

//▒▒▒▒▒▒▒▒▒▒▒▒ INIT ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub struct Locale {
	s: HashMap<String, String>,
}
pub struct UiStrings {
	s: HashMap<String, String>,
}

lazy_static! {
	pub static ref WS: Locale = {
		Locale {
			s: strings::worldgen_strings::get(&options::get().locale),
		}
	};
	pub static ref GS: Locale = {
		Locale {
			s: strings::game_strings::get(&options::get().locale),
		}
	};
	pub static ref MS: Locale = {
		Locale {
			s: strings::menu_strings::get(&options::get().locale),
		}
	};
	pub static ref UI: UiStrings = {
		UiStrings {
			s: strings::ui_strings::get(&options::get().locale),
		}
	};
}

//▒▒▒▒▒▒▒▒▒▒▒▒ MACRO ▒▒▒▒▒▒▒▒▒▒▒▒▒
// Those macrose define print_ and string_ functions that print and
// return the strings. For declarations see included modules.
#[macro_export]
macro_rules! printable_strings {
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
	}//macro
} //macro rules

#[macro_export]
macro_rules! printable_strings_with_arg {
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
	}//macro
} //macro rules

#[macro_export]
macro_rules! returnable_strings {
	($struct_name: ident($($fn_name: ident, $str_name: expr);*;)) =>
	{
		$(impl $struct_name {
			pub fn $fn_name(&self,) -> String {
				self.s[$str_name].replace(&['\n', '\t',][..], "")
			}//fn
		})*//impl
	}//macro
} //macro rules
