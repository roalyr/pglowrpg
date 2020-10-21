use io_ops::toml::{options, strings};
use ui::prompts;

pub fn start() {
	//Load options, must be here
	let options: options::Stuff = options::get();

	//Load UI locale, must be here
	let input_locale = options.locale.clone();

	//Load UI strings
	let ui_el: strings::ui_elements::Stuff =
		strings::ui_elements::get(&input_locale);

	let mn_str: strings::menu_strings::Stuff =
		strings::menu_strings::get(&input_locale);

	let wg_str: strings::worldgen_strings::Stuff =
		strings::worldgen_strings::get(&input_locale);

	let gm_str: strings::game_strings::Stuff =
		strings::game_strings::get(&input_locale);

	let panic_str: strings::panic_strings::Stuff =
		strings::panic_strings::get(&input_locale);

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
			worldgen::start(&options, &wg_str, &panic_str, &ui_el);
			//Intro message repeated
			println!("{}", mn_str.mn2);
		}

		if (input == "g") || (input == "G") {
			println!("{}", mn_str.mn3);
			game::start(&options, &gm_str, &panic_str, &ui_el);
			//Intro message repeated
			println!("{}", mn_str.mn2);
		}

		if (input == "q") || (input == "Q") {
			return;
		}
	}
}
