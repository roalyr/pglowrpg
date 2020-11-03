use io_ops::toml::{options, strings};
use ui::prompts;

pub fn start() {
	//Load UI locale, must be here
	let input_locale = options::get().locale;

	//Load UI strings
	let mn_str: strings::menu_strings::Stuff =
		strings::menu_strings::get(&input_locale);

	let ui_el: strings::ui_elements::Stuff =
		strings::ui_elements::get(&input_locale);

	//Intro message
	println!("{}", mn_str.mn1);
	println!("{}", mn_str.mn2);

	//Menu loop
	loop {
		let input = prompts::new_line_io("", &ui_el.prompt1);

		if input.is_empty() {
			continue;
		}

		if (input == "w") || (input == "W") {
			println!("{}", mn_str.mn2);
			worldgen::start(
				&options::get(),
				&strings::worldgen_strings::get(&input_locale),
				&strings::panic_strings::get(&input_locale),
				&strings::ui_elements::get(&input_locale),
			);
			//Intro message repeated
			println!("{}", mn_str.mn2);
		}

		if (input == "g") || (input == "G") {
			println!("{}", mn_str.mn3);
			game::start(
				//Let thus app take its own copy of those, for convenience
				options::get(),
				strings::game_strings::get(&input_locale),
				strings::panic_strings::get(&input_locale),
				strings::ui_elements::get(&input_locale),
			);
			//Intro message repeated
			println!("{}", mn_str.mn2);
		}

		if (input == "q") || (input == "Q") {
			return;
		}
	}
}
