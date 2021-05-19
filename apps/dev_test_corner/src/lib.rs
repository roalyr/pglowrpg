use colored::*;
use deepsize::DeepSizeOf;
use serde::{Deserialize, Serialize};
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
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct MaterialProperties {
		property: String, // Standard stuff such as material density, strength, etc.
	}
	// This is all possible variations of the Material type.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum Material {
		Bone(MaterialProperties),
		Flesh(MaterialProperties),
		Skin(MaterialProperties),
		Hair(MaterialProperties),
	}

	// Another pair of definitions.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct CreatureProperties {
		materials: Vec<Material>,
		// Behavior, size, structure, etc.
	}
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum EntityType {
		//Plant(PlantProperties),
		Creature(CreatureProperties),
	}
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct EntityData {
		// Everything that must be written in .ron
		name: String,
		entity_type: EntityType,
	}
	// Keep generic data in the header of the struct for ease of access.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct UniqueEntity {
		uid: u32,
		x: u32,
		y: u32,
		data: EntityData,
		// Associated items (inventory cache ID), etc.
		// Thoughts.
	}

	// RON stuff. Basically a string from the preset file.
	// Should match the EntityType enum above.
	let data = r#"
			EntityData(
				name : "Creature",
				entity_type : Creature(
					CreatureProperties(
						materials: [
							Bone(
								MaterialProperties(
									property: "Strong"
								)
							),
							Flesh(
								MaterialProperties(
									property: "Meaty"
								)
							)
						]
					)
				)
			)
		"#;

	let mut files = Vec::new();
	// Search all directories for files.
	let entity_from_file: EntityData = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}", e.to_string());
			std::process::exit(0);
		}
	};
	files.push(entity_from_file);

	// MAKING NON-SPATIAL GLOBAL TABLE.
	// Make init cap and total cap. Defines hasmap size.
	// Must be less than max ID values which will be U32 max.
	let entity_cap = 1_000_00;
	// ENTITY TABLE
	let mut unique_creatures: HashMap<u32, UniqueEntity> =
		HashMap::with_capacity(entity_cap);

	// Make sure uids are only assigned here.
	// UIDs as usize?
	let mut uid: u32 = 0;
	for entity_from_file in files.iter() {
		match &entity_from_file.entity_type {
			EntityType::Creature(_) => {
				unique_creatures.insert(
					uid,
					UniqueEntity {
						uid,
						x: 0,
						y: 0,
						data: files[0].clone(),
					},
				);
			}
		}
		// Make this capped at specific entity cap
		uid.checked_add(1).expect("Overflow at  uid += 1");
	}

	// WORLD-RELATED DATA TABLES AND CACHES.
	// Map - Vec<Cache_ID>,
	// Entitys cache table - HashMap<Cache_ID, Vec<Entity_ID>,
	// Entitys table - HashMap<Entity_ID, Entity>,

	// Making a non-spatial cache table to put specific creatures
	// onto the map in specific locations.
	// Cache table is related to map size.
	let map_size = 4096;
	let cache_table_size = map_size * map_size;
	// CACHE MAP
	let mut entities_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);

	// Now put the creatures at x, y coords:
	let x = 10;
	let y = 110;
	let queue = vec![123, 667, 9986];
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;
	entities_cache.insert(ind, queue.clone());
	// Update coords in their headers.
	// Make proper match for error
	for entity_id in queue.iter() {
		unique_creatures.get_mut(entity_id).unwrap().x = x as u32;
		unique_creatures.get_mut(entity_id).unwrap().y = y as u32;
	}

	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// Now access the creatures in the given location:
	let local_entities = &entities_cache[&ind];
	println!("{}", "\nCHECKING ENTRIES IN THE LOCATION".green());
	println!("Putting entities: {:?}", local_entities);
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	for entity_id in local_entities.iter() {
		// Destruct the entity. What should this return? How and when?
		match unique_creatures.get_mut(entity_id) {
			Some(entity) => {
				println!("UID: {}", entity.uid);
				match &entity.data.entity_type {
					// Have different destructors for different types.
					EntityType::Creature(properties) => {
						println!("Type: {:?}", &entity.data.name);
						// Move it into creature destructor.
						for material in properties.materials.iter() {
							// Move into material destructor.
							match material {
								Material::Bone(properties) => {
									println!("{:?}", properties);
								}
								Material::Flesh(properties) => {
									println!("{:?}", properties);
								}
								Material::Skin(properties) => {
									println!("{:?}", properties);
								}
								Material::Hair(properties) => {
									println!("{:?}", properties);
								}
							}
						}
					}
				}
			}
			None => {
				println!("ERROR: No entity by uid: {}", entity_id);
			}
		}
	}

	// Implement I/O functions for the cache map.

	//▒▒▒▒▒▒▒▒▒▒▒▒ END ▒▒▒▒▒▒▒▒▒▒▒▒▒
	println!("{}", "\nEND".blue());
}
