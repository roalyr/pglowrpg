pub mod game_str;
pub mod input;
pub mod menu_str;
pub mod output;
pub mod ui_str;
pub mod worldgen_str;

use game_options::OPTIONS;
use io_ops::readron::{palettes, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;

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
