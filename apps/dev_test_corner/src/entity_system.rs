use colored::*;
use deepsize::DeepSizeOf;
use lib_unit_systems::coords::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn start() {
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
		entity_name: String,
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
	// One entity per file.
	let data = r#"
			EntityData(
				entity_name : "rabbit",
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
							),
							Skin(
								MaterialProperties(
									property: "Smooth"
								)
							),
							Hair(
								MaterialProperties(
									property: "Fluffy"
								)
							)
						]
					)
				)
			)
		"#;

	// Parse presets and store data in type vectors.
	// Loop through all presets and preset folders.
	let entity_from_file: EntityData = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}", e.to_string());
			std::process::exit(0);
		}
	};
	let mut creature_types = Vec::new();
	//let mut item_types = Vec::new();
	//...
	// Sort entities into respective type vectors.
	match &entity_from_file.entity_type {
		EntityType::Creature(_) => {
			creature_types.push(entity_from_file);
		} //EntityType::Item...
	}

	// MAKING NON-SPATIAL GLOBAL TABLE.
	// Make init cap and total cap. Defines hasmap size.
	// Must be less than max ID values which will be U32 max.
	let creatures_cap = 1_000_000;
	let test_creatures_num = 3;
	//let items_cap = 1_000_000;
	//...
	let mut unique_creatures: HashMap<u32, UniqueEntity> =
		HashMap::with_capacity(creatures_cap);
	//let mut unique_items....

	// GENERATING UNIQUE ENTITIES.
	// Map - Vec<Cache_ID>,
	// Entities cache table - HashMap<Cache_ID, Vec<Entity_UID>,
	// Entities table - HashMap<Entity_UID, Entity>,

	// Making a non-spatial cache table to put specific creatures
	// onto the map in specific locations.
	// Cache table is related to map size.
	let map_size: u32 = 4096;
	let cache_table_size = (map_size * map_size) as usize;
	// CACHE MAP
	let mut creatures_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);

	// UID is personal ID, not a type ID.
	let mut uid_creature: u32 = 1;
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;
	// This is the record of UIDs of creatures in the local coords.
	let mut uids_creatures_local = Vec::new();

	//let mut uid_item: u32 = 1;
	//...

	// Use find to locate the required entity type and load its data.
	// This will be called by "creatures" layer generator.
	// The specific string will correspond to whatever is required from list.
	for _ in 0..test_creatures_num {
		uids_creatures_local.push(uid_creature);
		if let Some(creature_type) = creature_types
			.iter()
			.find(|EntityData { entity_name: x, .. }| *x == "rabbit")
		{
			unique_creatures.insert(
				uid_creature,
				UniqueEntity {
					uid: uid_creature,
					x,
					y,
					data: creature_type.clone(),
				},
			);
		}
		// Increment UID every call.
		uid_creature = uid_creature
			.checked_add(1)
			.expect("ERROR: overflow at  UID increment.");
	}
	println!("Total number of creatures: {}", unique_creatures.len());

	// Put an entity into the cache immediately.
	creatures_cache.insert(ind, uids_creatures_local);

	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// Now access the creatures in the given location:
	let local_entities = &creatures_cache[&ind];
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
						println!("Type: {:?}", &entity.data.entity_name);
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
}
