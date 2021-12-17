use colored::*;
use deepsize::DeepSizeOf;
use lib_unit_systems::coords::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// Plants should be described in patches or groups, that have
// UIDs, and region of origin (assigned at worldgen).
// This allows for unified entity system to be applied.

pub fn start() {
	//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
	// All this should go into dedicated entities lib.
	// It will be used by worldgen to read presets and by game to
	// retrieve data.
	// Entity part of the system. Things solely related to individual
	// objects, later used by the DISTRIBUTION system.

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct MaterialProperties {
		property: String,
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum Material {
		Plant(MaterialProperties),
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct PlantProperties {
		materials: Vec<Material>,
		max_quantity: u8,
		origin_region_id: u32,
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	enum EntityType {
		Plant(PlantProperties),
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct EntityData {
		entity_codename: String,
		entity_type: EntityType,
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct UniqueEntity {
		uid: u32,
		x: u32,
		y: u32,
		data: EntityData,
	}

	// ENTITY PRESET FILE.
	// All presets data are used in worldgen and are stored in save.
	let data = r#"
			EntityData(
				entity_codename : "grass_foxtail_patch",
				entity_type : Plant(
					PlantProperties(
						materials: [
							Plant(
								MaterialProperties(
									property: "Green fiber"
								)
							)
						],
						max_quantity: 255,
						origin_region_id: 0,
					)
				)
			)
		"#;

	// LOAD AND PARSE ON STARTUP.
	// In worldgen.
	let entity_from_file: EntityData = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("ERROR: {}", e.to_string());
			std::process::exit(0);
		}
	};

	let mut plant_types = Vec::new();
	match &entity_from_file.entity_type {
		EntityType::Plant(_) => {
			plant_types.push(entity_from_file);
		}
	}

	let plant_patches_cap = 1_000_000;
	let test_plant_patches_num = 3;
	//let items_cap = 1_000_000;
	//...
	let mut unique_plant_patches: HashMap<u32, UniqueEntity> =
		HashMap::with_capacity(plant_patches_cap);
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
	let mut plant_patch_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);

	// let mut plants_cache...
	// let mut items_cache...

	// UID here is personal ID, not a type ID.
	// Starting value.
	let mut uid_plant_patch: u32 = 1;

	// This is the record of UIDs of creatures in the local coords.
	// Temporary value, stores entities at given location.
	let mut uids_plant_patches_local = Vec::new();

	// Location (within entity generator loop)
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Use find to locate the required entity type and load its data.
	// This will be called by "creatures" layer generator.
	// The specific string will correspond to whatever is required from list.
	for _ in 0..test_plant_patches_num {
		uids_plant_patches_local.push(uid_plant_patch);
		if let Some(plant_type) = plant_types.iter().find(
			|EntityData {
			   entity_codename: x, ..
			 }| *x == "grass_foxtail_patch",
		) {
			unique_plant_patches.insert(
				uid_plant_patch,
				UniqueEntity {
					uid: uid_plant_patch,
					x,
					y,
					data: plant_type.clone(),
				},
			);
		}
		// Increment UID every call.
		uid_plant_patch = uid_plant_patch
			.checked_add(1)
			.expect("ERROR: overflow at  UID increment.");
	}
	println!(
		"Total number of plant patches: {}",
		unique_plant_patches.len()
	);

	// PUTTING A BATCH OF ENTITIES ON MAP.
	plant_patch_cache.insert(ind, uids_plant_patches_local);

	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:
	let local_entities = &plant_patch_cache[&ind];
	println!("{}", "\nCHECKING ENTRIES IN THE LOCATION".green());
	println!("Putting entities: {:?}", local_entities);
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	for entity_id in local_entities.iter() {
		// Destruct the entity. What should this return? How and when?
		match unique_plant_patches.get_mut(entity_id) {
			Some(entity) => {
				println!("UID: {}", entity.uid);
				match &entity.data.entity_type {
					// Have different destructors for different types.
					EntityType::Plant(properties) => {
						println!("Type: {:?}", &entity.data.entity_codename);
						// Move it into plant destructor.
						for material in properties.materials.iter() {
							// Move into material destructor.
							match material {
								Material::Plant(properties) => {
									println!("{:?}", properties);
								}
							}
						}
						println!("Max quantity {:?}", properties.max_quantity);
						println!(
							"Originates from region ID {:?}",
							properties.origin_region_id
						);
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
