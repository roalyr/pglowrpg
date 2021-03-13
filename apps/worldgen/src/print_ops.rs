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
	pub fn print_sep1(&self) {println!("{}", self.u.separator1);}
	pub fn print_sep2(&self) {println!("{}", self.u.separator2);}
	//Texts and messages
	pub fn print_intro(&self) {println!("{}", self.s.wg1);}
}

//▒▒▒▒▒▒▒▒▒▒▒▒ RETURN ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[rustfmt::skip]
impl WgLocale {
	//UI symbols (elements and separators)
	pub fn ui_bul1(&self) -> &str {&self.u.bullet1}
	pub fn ui_sep1(&self) -> &str {&self.u.separator1}
	pub fn ui_sep2(&self) -> &str {&self.u.separator2}
	//prompt symbols
	pub fn ui_ps1(&self) -> &str {&self.u.prompt1}
	pub fn ui_ps2(&self) -> &str {&self.u.prompt2}
	//strings
	pub fn str_sel_preset(&self) -> &str {&self.s.wg3}
	pub fn str_seed_rand(&self) -> &str {&self.s.wg2}
	pub fn str_world_num(&self) -> &str {&self.s.wg24}
	pub fn str_worlds_to_gen(&self) -> &str {&self.s.wg6}
	pub fn str_seed_used(&self) -> &str {&self.s.wg5}
}
