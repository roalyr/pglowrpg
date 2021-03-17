use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "ui_elements";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub newline: String,
	pub prompt1: String,
	pub prompt2: String,
	pub bullet1: String,
	pub separator1: String,
	pub separator2: String,
}

pub fn get(input: &str) -> Stuff {
	let path = Path::new(PATH_LOCALES)
		.join(&input)
		.join(FILENAME)
		.with_extension(EXTENSION_LOCALE);

	let data = crate::file_to_string(&vec![path.clone()]);

	let stuff: Stuff = match toml::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", e.to_string(), path.to_str().unwrap());
			std::process::exit(0);
		}
	};
	stuff
}
