use colored::*;
use deepsize::DeepSizeOf;
use std::collections::HashMap;
use unit_systems::coords::Index;

//use slots::consts::*;
//use slots::slots::Slots;
//use constants::app::*;
//use game_data_codec::LayerPack;
//use game_options::OPTIONS;
//use io_ops::decompress_to_memory;
//use io_ops::readron::palettes::biomes;
//use io_ops::readron::strings;
//use io_ops::writepng::from_hex;
//use std::path::Path;
//use text_ops::{prompt_input, GS};
//use unit_systems::translate;

pub fn start() {
	println!("{}", "DEV TESTING CORNER".red());
	println!("{}", "START\n".blue());
	//▒▒▒▒▒▒▒▒▒▒▒▒ START ▒▒▒▒▒▒▒▒▒▒▒▒▒

	// DEFINING OBJECTS
	// This is a standard set of parameters for a type.
	#[derive(Debug, Clone, DeepSizeOf)]
	struct MaterialProperties {
		property: String, // Standard stuff such as material density, strength, etc.
	}
	// This is all possible variations of the Material type.
	#[derive(Debug, Clone, DeepSizeOf)]
	enum Material {
		Bone(MaterialProperties),
		Flesh(MaterialProperties),
		Skin(MaterialProperties),
		Hair(MaterialProperties),
	}

	// Another pair of definitions.
	#[derive(Debug, Clone, DeepSizeOf)]
	struct CreatureProperties {
		materials: Vec<Material>,
		// Behavior, size, structure, etc.
	}
	#[derive(Debug, Clone, DeepSizeOf)]
	enum CreatureType {
		//Fish(CreatureProperties),
		//Bird(CreatureProperties),
		Animal(CreatureProperties),
	}
	// Keep generic data in the header of the struct for ease of access.
	#[derive(Debug, Clone, DeepSizeOf)]
	struct Creature {
		uid: u32,
		x: u32,
		y: u32,
		creature_type: CreatureType,
		// Associated items (inventory cache ID), etc.
		// Thoughts.
	}

	// MAKING NON-SPATIAL GLOBAL TABLE.
	// Make init cap and total cap. Defines hasmap size.
	// Must be less than max ID values which will be U32 max.
	let creature_cap = 1_000_00;
	// ENTITY TABLE
	let mut unique_creatures: HashMap<u32, Creature> =
		HashMap::with_capacity(creature_cap);

	// Filling up creatures object from preset.
	for uid in 0..creature_cap {
		unique_creatures
			// Here we can try to insert result from parting .ron.
			.insert(
				uid as u32,
				Creature {
					uid: uid as u32,
					x: 0,
					y: 0,
					creature_type: CreatureType::Animal(CreatureProperties {
						materials: vec![
							Material::Bone(MaterialProperties {
								property: "Tough".to_string(),
							}),
							Material::Flesh(MaterialProperties {
								property: "Meaty".to_string(),
							}),
							Material::Skin(MaterialProperties {
								property: "Pale".to_string(),
							}),
							Material::Hair(MaterialProperties {
								property: "Long".to_string(),
							}),
						], //mats
					}), //type
				}, //creature
			); //insert
	}

	// Check stuff.
	let single_creature = unique_creatures[&123].clone();
	let entry_size = single_creature.deep_size_of();
	println!("{}", "SINGLE ENTRY CHECK".green());
	println!("Entry size: {:?}", entry_size);
	println!("Entry content: {:?}", single_creature);

	// WORLD-RELATED DATA TABLES AND CACHES.
	// Map - Vec<Cache_ID>,
	// Creatures cache table - HashMap<Cache_ID, Vec<Creature_ID>,
	// Creatures table - HashMap<Creature_ID, Creature>,

	// Making a non-spatial cache table to put specific creatures
	// onto the map in specific locations.
	// Cache table is related to map size.
	let map_size = 4096;
	let cache_table_size = map_size * map_size;
	// CACHE MAP
	let mut creatures_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);

	// Now put the creatures at x, y coords:
	let x = 10;
	let y = 110;
	let queue = vec![123, 667, 9986];
	let xy = Index { map_size };
	let index = xy.ind(x, y) as u32;
	creatures_cache.insert(index, queue.clone());
	// Update coords in their headers.
	for creature_id in queue.iter() {
		unique_creatures.get_mut(creature_id).unwrap().x = x as u32;
		unique_creatures.get_mut(creature_id).unwrap().y = y as u32;
	}

	// Now access the creatures in the given location:
	let local_creatures = &creatures_cache[&index];
	println!("{}", "\nCHECKING ENTRIES IN THE LOCATION".green());
	println!("Putting creatures: {:?}", local_creatures);
	println!("At x: {}, y: {}, index: {}\n", x, y, index);
	for creature_id in local_creatures.iter() {
		println!("{:?}", unique_creatures[creature_id]);
	}

	// Then parsethe Creature object...

	// Implement I/O functions for the cache map.

	//▒▒▒▒▒▒▒▒▒▒▒▒ END ▒▒▒▒▒▒▒▒▒▒▒▒▒
	println!("{}", "\nEND".blue());
}
