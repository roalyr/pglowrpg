pub mod game_str;
pub mod input;
pub mod menu_str;
pub mod output;
pub mod ui_str;
pub mod worldgen_str;

use constants_app::*;
use game_options::OPTIONS;
use io_ops::get_strings_hashmaps;
use lazy_static::lazy_static;
use serde::{Deserialize, Serialize};
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
	pub static ref WS: WgStrings = {
		WgStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_WORLDGEN),
		}
	};
	pub static ref GS: GmStrings = {
		GmStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_GAME),
		}
	};
	pub static ref MS: MnStrings = {
		MnStrings {
			s: get_strings_hashmaps!(OPTIONS.locale, NAME_STRINGS_MENU),
		}
	};
	pub static ref UI: UiStrings = {
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
