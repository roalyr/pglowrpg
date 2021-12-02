use colored::*;
use deepsize::DeepSizeOf;
use lib_unit_systems::coords::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

pub fn start() {
	// MATERIALS GENERAL
	// Properties of a material, that define it.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct MaterialProperties {
		property: String, 
		// density
		// strength
		// color
	}
	
	// INDIVIDUAL MATERIALS
	// All existing individual materials.
	// TODO: keep materials hardcoded?
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum Material {
		Bone(MaterialProperties),
		Flesh(MaterialProperties),
		Skin(MaterialProperties),
		Hair(MaterialProperties),
	}

	// ENTITY KIND GENERAL
	// Commin for all creatures.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct CreatureProperties {
		materials: Vec<Material>,
		// behavior
		// size
		// structure
		// etc.
	}
	// Commin for all plants.
	//#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	//struct PlantProperties {
		//materials: Vec<Material>,
		// size
		// structure
		// etc.
	}
	
	// UNIQUE ENTITY TYPES.
	// All possible entities.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum EntityType {
		Creature(CreatureProperties),
		// Plant(PlantProperties),
		// Item(ItemProperties),
	}
	
	// UNIQUE ENTITY DATA PACKAGE.
	// This is a container for all the data attached to an entity in 
	// game space.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct EntityData {
		// Codename is an internal name to match against respective
		// langauge strings with display name and descriptions.
		entity_codename: String,
		entity_type: EntityType,
	}
	
	// UNIQUE ENTITY ITSELF.
	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct UniqueEntity {
		uid: u32,
		x: u32,
		y: u32,
		// Unique values for given entity.
		// room or location
		// state
		// age
		// etc.
		data: EntityData,
		// Unique attached data links for given entity.
		// Associated items (inventory cache ID), etc.
		// Thoughts.
	}

	// ENTITY PRESET FILE.
	// One entity per file.
	let data = r#"
			EntityData(
				entity_codename : "rabbit",
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


	// LOAD AND PARSE ON STARTUP.
	// Parse presets and store data in type vectors.
	// Loop through all presets and preset folders.
	// This data is loaded once on worldgen, and then it should be
	// saved to the game data package.
	let entity_from_file: EntityData = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}", e.to_string());
			std::process::exit(0);
		}
	};
	// Those are types not unique entities.
	let mut creature_types = Vec::new();
	//let mut plant_types = Vec::new();
	//let mut item_types = Vec::new();
	//...
	// Sort entities into respective type vectors.
	match &entity_from_file.entity_type {
		EntityType::Creature(_) => {
			creature_types.push(entity_from_file);
		} 
		//EntityType::Plant...
		//EntityType::Item...
	}

	// MAKING NON-SPATIAL GLOBAL TABLE.
	// Make total number cap. Defines hasmap size.
	// Must be less than max ID value which will be U32 max.
	let creatures_cap = 1_000_000;
	let test_creatures_num = 3;
	//let items_cap = 1_000_000;
	//...
	let mut unique_creatures: HashMap<u32, UniqueEntity> =
		HashMap::with_capacity(creatures_cap);
	//let mut unique_items....

	// GENERATING UNIQUE ENTITIES.
	//This is done one worldgen stage.
	// -------------------------------------------------------------
	// The relations and data embedding is like this:
	//
	// Blob of entities on map:
	// Map cell (x, y) -> [Cache_ID]
	//
	// What entities are in what blob:
	// Entities cache table -> <Cache_ID, Vec<Entity_UID>>
	//
	// Which entiry is of what kind and has which data.
	// Entities table - HashMap<Entity_UID, UniqueEntity>,
	// -------------------------------------------------------------

	// TEST
	// Making a non-spatial cache table to put specific creatures
	// onto the map in specific locations.
	// Cache table is related to map size.
	let map_size: u32 = 4096;
	let cache_table_size = (map_size * map_size) as usize;
	
	// CACHE TABLE.
	// Created at worldgen.
	let mut creatures_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);
		
	// let mut plants_cache...
	// let mut items_cache...


	// UID here is personal ID, not a type ID.
	// Starting value.
	let mut uid_creature: u32 = 1;
	
	// This is the record of UIDs of creatures in the local coords.
	// Temporary value, stores entities at given location.
	let mut uids_creatures_local = Vec::new();
	
	// Location (within entity generator loop)
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Use find to locate the required entity type and load its data.
	// This will be called by "creatures" layer generator.
	// The specific string will correspond to whatever is required from list.
	for _ in 0..test_creatures_num {
		uids_creatures_local.push(uid_creature);
		if let Some(creature_type) = creature_types
			.iter()
			.find(|EntityData { entity_codename: x, .. }| *x == "rabbit")
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

	// PUTTING A BATCH OF ENTITIES ON MAP.
	creatures_cache.insert(ind, uids_creatures_local);

	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// TEST. READING AND PARSING.
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
						println!("Type: {:?}", &entity.data.entity_codename);
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
