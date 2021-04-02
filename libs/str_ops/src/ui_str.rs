use crate::printable_strings;
use crate::{UiStrings, UI};
use textwrap::{fill, termwidth, Options};

//▒▒▒▒▒▒▒▒▒▒ UI PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl UiStrings {
	pub fn print_newline(&self) {println!("{}", self.s["newline"]);}
	pub fn print_sep1(&self) {println!("{}", self.s["separator1"]);}
	pub fn print_sep2(&self) {println!("{}", self.s["separator2"]);}
}

//▒▒▒▒▒▒▒▒▒▒ UI RETURN ▒▒▒▒▒▒▒▒▒▒▒▒▒
//#[rustfmt::skip]
impl UiStrings {
	pub fn newline(&self) -> &str {
		&self.s["newline"]
	}
	pub fn bul1(&self) -> &str {
		&self.s["bullet1"]
	}
	pub fn sep1(&self) -> &str {
		&self.s["separator1"]
	}
	pub fn sep2(&self) -> &str {
		&self.s["separator2"]
	}
	pub fn prompt1(&self) -> &str {
		&self.s["prompt1"]
	}
	pub fn prompt2(&self) -> &str {
		&self.s["prompt2"]
	}
}
