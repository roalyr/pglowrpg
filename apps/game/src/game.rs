use io_ops::toml::{options, strings};
use ui::prompt;

pub fn start(
	options: &options::Stuff,
	gm_str: &strings::game_strings::Stuff,
	panic_str: &strings::panic_strings::Stuff,
	ui_el: &strings::ui_elements::Stuff,
) {
	//Intro message
	println!("{}", &gm_str.gm1);

	//main loop
	loop {
		let mut input = prompt::new_line_io("", &ui_el);

		if input.is_empty() {
			continue;
		}

		if (input == "q") || (input == "Q") {
			return;
		}
	}
}
