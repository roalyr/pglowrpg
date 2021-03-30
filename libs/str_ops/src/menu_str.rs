use io_ops::toml::{options, strings};
use lazy_static::lazy_static;

lazy_static! {
	pub static ref LOC: Locale = initiate();
}

pub struct Locale {
	s: strings::menu_strings::Stuff,
	u: strings::ui_elements::Stuff,
	_p: strings::panic_strings::Stuff,
}

fn initiate() -> Locale {
	let input_locale = options::get().locale;
	Locale {
		s: strings::menu_strings::get(&input_locale),
		u: strings::ui_elements::get(&input_locale),
		_p: strings::panic_strings::get(&input_locale),
	}
}

//▒▒▒▒▒▒▒▒▒▒ UI PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn print_newline(&self) {println!("{}", self.u.newline);}
	pub fn print_sep1(&self) {println!("{}", self.u.separator1);}
	pub fn print_sep2(&self) {println!("{}", self.u.separator2);}
}

//▒▒▒▒▒▒▒▒▒▒ UI RETURN ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn newline(&self) -> &str {&self.u.newline}
	pub fn bul1(&self) -> &str {&self.u.bullet1}
	pub fn sep1(&self) -> &str {&self.u.separator1}
	pub fn sep2(&self) -> &str {&self.u.separator2}
	pub fn prompt1(&self) -> &str {&self.u.prompt1}
	pub fn prompt2(&self) -> &str {&self.u.prompt2}
}

//▒▒▒▒▒▒▒▒▒ STRINGS PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn print_intro(&self) {println!("{}", self.s.mn1);}
	pub fn print_menu(&self) {println!("{}", self.s.mn2);}
}

//▒▒▒▒▒▒▒▒▒▒ STRINGS RETURN ▒▒▒▒▒▒▒▒▒▒▒
//#[rustfmt::skip]
//impl Locale {
	//pub fn sel_preset(&self) -> &str {&self.s.wg3}
//}
