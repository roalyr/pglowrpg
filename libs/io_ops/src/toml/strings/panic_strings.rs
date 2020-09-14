use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "panic_strings";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub map_size_1: String,
	pub map_size_2: String,
	pub noisemap_size_1: String,
	pub noisemap_size_2: String,
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
