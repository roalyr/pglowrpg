use rand::prelude::*;
//use std::io::{self, Write};

fn main() {
	let mut rng = rand::thread_rng();

	//Strings
	//arrival verbs
	let arrival_verbs = [
		"arrive to new place",
		"stop by this place",
		"decide to make a camp at this site",
		"take a small break here",
		"pass by this place",
	];
	//walking verbs
	let walk_verb = [
		"keep moving on",
		"resume your travel",
		"go on",
		"depart",
		"go away",
	];
	//examine verbs
	let explore_verb = [
		"explore",
		"look around",
		"examine",
		"decide to not to explore",
		"curb your curiosity, avoid approaching",
	];
	//spot verbs
	let spot_verb = ["spot", "see", "notice", "can see"];
	//immediate proximity
	let prox_word = [
		"here",
		"around",
		"at this place",
		"around you",
		"at this site",
	];

	//terrain
	let terrain_p = 0.1;
	let terrain_type = [
		"rocky",
		"muddy",
		"sandy",
		"dusty",
		"covered in gravel",
		"mostly clay",
	];
	let terrain_word = ["ground", "terrain", "soil", "land"];

	//vegetation
	let vegetation_p = 0.2;
	let vegetation_word = ["lush", "sparse", "dense", "green"];
	let vegetation_type = [
		"trees",
		"shrubs",
		"grasses",
		"trees and shrubs",
		"shrubs and grasses",
	];

	//constructions
	let constructions_p = 0.05;
	let constructions_word = ["old", "abandoned", "new", ""];
	let constructions_type = [
		"house",
		"hut",
		"wall remains",
		"castle remains",
		"well",
		"road",
		"tower",
	];
	//near field

	//far field

	loop {
		let mut iter = 0;
		let mut terrain_prev = 0;
		let mut constructions_prev = 0;
		let mut vegetation_prev = 0;
		let mut time = 0; //mins
		let mut day = 0;
		let mut day_prev = 0;

		//inp
		//print!("Input the seed >> ");
		//let _ = io::stdout().flush();
		//let mut inp = String::new();
		//io::stdin().read_line(&mut inp).unwrap();
		//let seed : usize = inp.trim().parse().expect("number 1, please");
		//let seed = 1;

		loop {
			//constructions
			//probability trigger
			let constructions_t: f32 = rng.gen();
			//event
			if constructions_t < constructions_p {
				//pick random line
				let i = rng.gen_range(0, spot_verb.len()) as usize;
				let j = rng.gen_range(0, constructions_type.len())
					as usize;
				let k = rng.gen_range(0, constructions_word.len())
					as usize;
				let l = rng.gen_range(0, explore_verb.len()) as usize;
				let m = rng.gen_range(0, walk_verb.len()) as usize;
				//print if changed
				if i != constructions_prev {
					//describe the new scenery
					println!(
						"It is currently {} o'clock.",
						time / 60,
					);
					println!(
						"You {} {} {}.",
						spot_verb[i],
						constructions_word[k],
						constructions_type[j],
					);
					println!(
						"You {} the {} and {}",
						explore_verb[l],
						constructions_type[j],
						walk_verb[m],
					);
					println!("...");
				}
				constructions_prev = i;
			}
			//end constructions

			//terrain and veg
			//probability trigger
			let terrain_t: f32 = rng.gen();
			//event
			if terrain_t < terrain_p {
				//pick random line
				let i = rng.gen_range(0, terrain_type.len()) as usize;
				let j = rng.gen_range(0, terrain_word.len()) as usize;
				let k = rng.gen_range(0, prox_word.len()) as usize;
				let ii =
					rng.gen_range(0, vegetation_type.len()) as usize;
				let jj =
					rng.gen_range(0, vegetation_word.len()) as usize;
				let m = rng.gen_range(0, walk_verb.len()) as usize;
				let n =
					rng.gen_range(0, arrival_verbs.len()) as usize;
				//print if changed
				if i != terrain_prev {
					println!("You {}.", arrival_verbs[n]);
					//describe the new scenery
					println!(
						"The {} is {}, {} are {} {}.",
						terrain_word[j],
						terrain_type[i],
						vegetation_type[ii],
						vegetation_word[jj],
						prox_word[k]
					);
					println!("You {}.", walk_verb[m],);
					println!("...");
				}
				terrain_prev = i;
			}
			//end terrain and veg

			//day
			if day != day_prev {
				println!("======== day {} ========", day);
				day_prev = day;
			}
			//day end

			iter = iter + 1;
			time = time + 5; //mins
			if time >= 1440 {
				day = day + 1;
				time = 0;
			}
		}
	}
}
