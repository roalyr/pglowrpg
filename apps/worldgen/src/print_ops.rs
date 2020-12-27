use io_ops::toml::{options, strings};

pub struct WgLocale {
	pub s: strings::worldgen_strings::Stuff,
	pub u: strings::ui_elements::Stuff,
	pub p: strings::panic_strings::Stuff,
}

impl WgLocale {
	pub fn print_intro(&self) {
		println!("{}", self.s.wg1);
	}
}

pub fn locale_load() -> WgLocale {
	let input_locale = options::get().locale;
	WgLocale {
		s: strings::worldgen_strings::get(&input_locale),
		u: strings::ui_elements::get(&input_locale),
		p: strings::panic_strings::get(&input_locale),
	}
}
