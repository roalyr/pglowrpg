use crate::print_ops::*;
use ui::prompts;

pub fn start() {
	let l = locale_load();
	l.print_intro();
	//Menu loop
	loop {
		let input = prompts::new_line_io("", &l.u.prompt1);
		if input.is_empty() {
			continue;
		}
		match input.as_str() {
			"w" => worldgen::start(),
			"g" => game::start(),
			"q" => return,
			_ => {}
		}
	}
}
