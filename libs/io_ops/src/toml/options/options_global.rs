use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "global";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub locale: String,
}

pub fn get() -> Stuff {
	let path = Path::new(PATH_OPTIONS)
		.join(FILENAME)
		.with_extension(EXTENSION_OPTION);

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
