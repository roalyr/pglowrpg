use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "menu_strings";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub mn1: String,
	pub mn2: String,
	pub mn3: String,
	pub mn4: String,
	pub mn5: String,
	pub mn6: String,
	pub mn7: String,
	pub mn8: String,
	pub mn9: String,
	pub mn10: String,
	pub mn11: String,
	pub mn12: String,
	pub mn13: String,
	pub mn14: String,
	pub mn15: String,
	pub mn16: String,
	pub mn17: String,
	pub mn18: String,
	pub mn19: String,
	pub mn20: String,
	pub mn21: String,
	pub mn22: String,
	pub mn23: String,
	pub mn24: String,
	pub mn25: String,
	pub mn26: String,
	pub mn27: String,
	pub mn28: String,
	pub mn29: String,
	pub mn30: String,
	pub mn31: String,
	pub mn32: String,
	pub mn33: String,
	pub mn34: String,
	pub mn35: String,
	pub mn36: String,
	pub mn37: String,
	pub mn38: String,
	pub mn39: String,
	pub mn40: String,
	pub mn41: String,
	pub mn42: String,
	pub mn43: String,
	pub mn44: String,
	pub mn45: String,
	pub mn46: String,
	pub mn47: String,
	pub mn48: String,
	pub mn49: String,
	pub mn50: String,
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
