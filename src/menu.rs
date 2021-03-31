use str_ops::menu_str::LOC;
use ui::prompts;

pub fn start() {
	LOC.print_intro();

	loop {
		LOC.print_menu();
		let input = prompts::new_line_io("", LOC.prompt1());
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
