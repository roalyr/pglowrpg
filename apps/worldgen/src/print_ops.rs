use io_ops::toml::{options, strings};

pub struct WgLocale {
	s: strings::worldgen_strings::Stuff,
	u: strings::ui_elements::Stuff,
	p: strings::panic_strings::Stuff,
}

pub fn locale_load() -> WgLocale {
	let input_locale = options::get().locale;
	WgLocale {
		s: strings::worldgen_strings::get(&input_locale),
		u: strings::ui_elements::get(&input_locale),
		p: strings::panic_strings::get(&input_locale),
	}
}

//All the print calls from within the apps should be via
//this interface. Both for printing and yielding strings.
//▒▒▒▒▒▒▒▒▒▒▒▒ PRINT ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl WgLocale {
	//UI symbols (elements and separators)
	pub fn print_newline(&self) {println!("{}", self.u.newline);}
	pub fn print_sep1(&self) {println!("{}", self.u.separator1);}
	pub fn print_sep2(&self) {println!("{}", self.u.separator2);}
	//Texts and messages
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

//▒▒▒▒▒▒▒▒▒▒▒▒ RETURN ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl WgLocale {
	//UI symbols (elements and separators)
	pub fn ui_newline(&self) -> &str {&self.u.newline}
	pub fn ui_bul1(&self) -> &str {&self.u.bullet1}
	pub fn ui_sep1(&self) -> &str {&self.u.separator1}
	pub fn ui_sep2(&self) -> &str {&self.u.separator2}
	//prompt symbols
	pub fn ui_ps1(&self) -> &str {&self.u.prompt1}
	pub fn ui_ps2(&self) -> &str {&self.u.prompt2}
	//strings (prefer print_ methods above)
	pub fn str_sel_preset(&self) -> &str {&self.s.wg3}
	pub fn str_seed_rand(&self) -> &str {&self.s.wg2}
	pub fn str_world_num(&self) -> &str {&self.s.wg24}
}
