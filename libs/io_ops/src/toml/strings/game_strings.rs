use constants_app::*;
use serde::{Deserialize, Serialize};
use std::path::Path;

const FILENAME: &str = "game_strings";

#[derive(Serialize, Deserialize)]
pub struct Stuff {
	pub gm1: String,
	pub gm2: String,
	pub gm3: String,
	pub gm4: String,
	pub gm5: String,
	pub gm6: String,
	pub gm7: String,
	pub gm8: String,
	pub gm9: String,
	pub gm10: String,
	pub gm11: String,
	pub gm12: String,
	pub gm13: String,
	pub gm14: String,
	pub gm15: String,
	pub gm16: String,
	pub gm17: String,
	pub gm18: String,
	pub gm19: String,
	pub gm20: String,
	pub gm21: String,
	pub gm22: String,
	pub gm23: String,
	pub gm24: String,
	pub gm25: String,
	pub gm26: String,
	pub gm27: String,
	pub gm28: String,
	pub gm29: String,
	pub gm30: String,
	pub gm31: String,
	pub gm32: String,
	pub gm33: String,
	pub gm34: String,
	pub gm35: String,
	pub gm36: String,
	pub gm37: String,
	pub gm38: String,
	pub gm39: String,
	pub gm40: String,
	pub gm41: String,
	pub gm42: String,
	pub gm43: String,
	pub gm44: String,
	pub gm45: String,
	pub gm46: String,
	pub gm47: String,
	pub gm48: String,
	pub gm49: String,
	pub gm50: String,
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
