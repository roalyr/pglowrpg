pub mod commands;

// Returns string file contents which is hashmap.
#[macro_export]
macro_rules! get_strings_hash {
	($locale: expr, $filename: expr) => {{
		use io_ops::file_to_string;
		let path = Path::new(PATH_LOCALES)
			.join(&$locale)
			.join($filename)
			.with_extension(EXTENSION_LOCALE);

		let data = file_to_string(&vec![path.clone()]);

		let stuff: HashMap<String, String> = match ron::from_str(&data) {
			Ok(f) => f,
			Err(e) => {
				println!("{}: {}", e.to_string(), path.to_str().unwrap());
				std::process::exit(0);
			}
		};
		stuff
	}};
}
