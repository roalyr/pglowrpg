use io_ops::toml::{options, strings};

pub struct Locale {
	pub s: strings::menu_strings::Stuff,
	pub u: strings::ui_elements::Stuff,
}

impl Locale {
	pub fn print_intro(&self) {
		println!("{}", self.s.mn1);
		println!("{}", self.s.mn2);
	}

	pub fn print_menu(&self) {
		println!("{}", self.s.mn2);
	}
}

pub fn locale_load() -> Locale {
	let input_locale = options::get().locale;
	Locale {
		s: strings::menu_strings::get(&input_locale),
		u: strings::ui_elements::get(&input_locale),
	}
}
