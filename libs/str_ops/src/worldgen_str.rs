use io_ops::toml::{options, strings};
use lazy_static::lazy_static;

lazy_static! {
	pub static ref LOC: Locale = initiate();
}

pub struct Locale {
	s: strings::worldgen_strings::Stuff,
	u: strings::ui_elements::Stuff,
	_p: strings::panic_strings::Stuff,
}

fn initiate() -> Locale {
	let input_locale = options::get().locale;
	Locale {
		s: strings::worldgen_strings::get(&input_locale),
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
	pub fn print_intro(&self) {println!("{}", self.s.wg1);}
	pub fn print_no_input_preset(&self) {println!("{}", self.s.wg28);}
	pub fn print_seed_rand(&self) {println!("{}", self.s.wg4);}
	pub fn print_prep_topog(&self) {println!("{}", self.s.wg7);}
	pub fn print_prep_climate(&self) {println!("{}", self.s.wg9);}
	pub fn print_prep_wmask(&self) {println!("{}", self.s.wg13);}
	pub fn print_prep_rmap(&self) {println!("{}", self.s.wg17);}
	pub fn print_prep_biome(&self) {println!("{}", self.s.wg19);}
	pub fn print_prep_georeg(&self) {println!("{}", self.s.wg21);}
	pub fn print_write_data(&self) {println!("{}", self.s.wg25);}
	pub fn print_write_no_data(&self) {println!("{}", self.s.wg26);}
	pub fn print_write_color(&self) {println!("{}", self.s.wg15);}
	pub fn print_write_raw(&self) {println!("{}", self.s.wg16);}
	pub fn print_done_worldgen(&self) {println!("{}", self.s.wg23);}
	//With args
	pub fn print_world_num(&self, x : &usize) {println!("{}\"{}\"", self.s.wg6, x);}
	pub fn print_seed_used(&self, x : &usize) {println!("{}\"{}\"", self.s.wg5, x);}
}

//▒▒▒▒▒▒▒▒▒▒ STRINGS RETURN ▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl Locale {
	pub fn sel_preset(&self) -> &str {&self.s.wg3}
	pub fn seed_rand(&self) -> &str {&self.s.wg2}
	pub fn world_num(&self) -> &str {&self.s.wg24}
}
