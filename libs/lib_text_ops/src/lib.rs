pub mod game_str;
pub mod input;
pub mod menu_str;
pub mod output;
pub mod ui_str;
pub mod worldgen_str;

use lazy_static::lazy_static;
use lib_constants::app::*;
use lib_game_options::OPTIONS;
use lib_io_ops::get_strings_hashmaps;
use std::collections::HashMap;
use std::path::Path;

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
	pub static ref worldgen_strings: WgStrings = {
		WgStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_WORLDGEN),
		}
	};
	pub static ref game_strings: GmStrings = {
		GmStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_GAME),
		}
	};
	pub static ref menu_strings: MnStrings = {
		MnStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_MENU),
		}
	};
	pub static ref interface_strings: UiStrings = {
		UiStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_UI),
		}
	};
}

pub fn print_error(st: String) {
	println!(
		"ERROR in locale .ron file. Key for string not found: \n{}\n",
		st
	);
}

pub fn input_flag_error(st: String) {
	println!(
		"ERROR, wrong flag used in macro invocation for input: \n{}\n",
		st
	);
	println!("Should be either opt or num (or you forgot the wordlist).");
}

pub fn input_flag_error_word(st: String) {
	println!(
		"ERROR, wrong flag used in macro invocation for input: \n{}\n",
		st
	);
	println!("Should be word, and there must be a wordlist provided.");
}
