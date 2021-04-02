use constants_app::*;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::Path;

const FILENAME: &str = "ui_strings";

pub fn get(input: &str) -> HashMap<String, String> {
	let path = Path::new(PATH_LOCALES)
		.join(&input)
		.join(FILENAME)
		.with_extension(EXTENSION_LOCALE);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: HashMap<String, String> = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
