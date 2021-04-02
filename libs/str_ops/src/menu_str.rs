use io_ops::readron::{options, strings};
use lazy_static::lazy_static;
use std::collections::HashMap;

lazy_static! {
	pub static ref LOC: Locale = initiate();
}

pub struct Locale {
	s: HashMap<String, String>,
	u: HashMap<String, String>,
	_p: HashMap<String, String>,
}

fn initiate() -> Locale {
	let input_locale = options::get().locale;
	Locale {
		//call it by a macro, macro be in io_ops
		s: strings::menu_strings::get(&input_locale),
		u: strings::ui_strings::get(&input_locale),
		_p: strings::panic_strings::get(&input_locale),
	}
}

//▒▒▒▒▒▒▒▒▒▒ UI PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn print_newline(&self) {println!("{}", self.u["newline"]);}
	pub fn print_sep1(&self) {println!("{}", self.u["separator1"]);}
	pub fn print_sep2(&self) {println!("{}", self.u["separator2"]);}
}

//▒▒▒▒▒▒▒▒▒▒ UI RETURN ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn newline(&self) -> &str {&self.u["newline"]}
	pub fn bul1(&self) -> &str {&self.u["bullet1"]}
	pub fn sep1(&self) -> &str {&self.u["separator1"]}
	pub fn sep2(&self) -> &str {&self.u["separator2"]}
	pub fn prompt1(&self) -> &str {&self.u["prompt1"]}
	pub fn prompt2(&self) -> &str {&self.u["prompt2"]}
}

//▒▒▒▒▒▒▒▒▒ STRINGS PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn print_intro(&self) {println!("{}", self.s["intro"]);}
	pub fn print_menu(&self) {println!("{}", self.s["menu"]);}
}

//▒▒▒▒▒▒▒▒▒▒ STRINGS RETURN ▒▒▒▒▒▒▒▒▒▒▒
//#[rustfmt::skip]
//impl Locale {
//pub fn sel_preset(&self) -> &str {&self.s.wg3}
//}
