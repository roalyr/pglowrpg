use io_ops::toml::{options, strings};
use ui::prompt;

pub fn start() {
	//Load options
	let options_worldgen: options::options_worldgen::Stuff =
		options::options_worldgen::get();

	let options_global: options::options_global::Stuff =
		options::options_global::get();

	let options_debug: options::options_debug::Stuff =
		options::options_debug::get();

	//Load UI locale
	let input_locale = options_global.locale.clone();

	//Load UI strings
	let wg_str: strings::worldgen_strings::Stuff =
		strings::worldgen_strings::get(&input_locale);

	let panic_str: strings::panic_strings::Stuff =
		strings::panic_strings::get(&input_locale);

	//Main menu screen and options will be here later
	worldgen::worldgen::start(
		options_worldgen,
		options_global,
		options_debug,
		wg_str,
		panic_str,
	);
}
