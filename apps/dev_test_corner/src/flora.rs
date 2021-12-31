use colored::*;
use deepsize::DeepSizeOf;
use lib_constants::world as cw;
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
	// From the forest structure by height, from highest to lowest.
	// Simplified names for uniformity.
	enum PlantLevel {
		TallCanopy,
		MediumCanopy,
		ShortCanopy,
		Shrub,
		Grass,
		Root,
	}

	#[derive(Debug, Clone, Serialize, Deserialize, DeepSizeOf)]
	struct PlantProperties {
		plant_level: PlantLevel,
		local_max_quantity: u32,
		materials: Vec<Material>,
		native_biomes: Vec<String>,
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

	println!("{}", "READING DATA FILE\n".green());

	// ENTITY PRESET FILE.
	// All presets data are used in worldgen and are stored in save.
	let data = r#"
		[ // Start from vector that contains entities.
			EntityData(
				entity_codename : "grass_foxtail",
				entity_type : Plant(
					PlantProperties(
						// Plants be described as "patches" or "groups"
						// of population individuals at given place, so
						// a quantity parameter is given.
						// For instance, if there are 10 grass sprouts per 1 sq.m
						// Then for a place (300x300 m) the quantity will be like
						// 900000, thus taking u32 init.
						plant_level: Grass,
						local_max_quantity: 1000000,
						materials: [
							Plant(
								MaterialProperties(
									property: "Green fiber"
								)
							)
						],
						native_biomes: [
							"test_invalid_biome",
							"biome_boreal_grassland",
							"biome_temperate_grassland",
							"biome_tropical_grassland",
							"biome_boreal_woodland",
							"biome_temperate_woodland",
							"biome_tropical_woodland",
							"biome_boreal_forest",
							"biome_temperate_forest",
							"biome_tropical_forest",
						],
					)
				)
			),
			EntityData(
				entity_codename : "grass_barley",
				entity_type : Plant(
					PlantProperties(
						plant_level: Grass,
						local_max_quantity: 1000000,
						materials: [
							Plant(
								MaterialProperties(
									property: "Green fiber"
								)
							)
						],
						native_biomes: [
							"biome_temperate_grassland",
							"biome_tropical_grassland",
							"biome_boreal_woodland",
							"biome_temperate_woodland",
							"biome_tropical_woodland",
							"biome_boreal_shrubland",
							"biome_temperate_shrubland",
							"biome_tropical_shrubland",
						],
					)
				)
			),
			EntityData(
				entity_codename : "grass_foxtail",
				entity_type : Plant(
					PlantProperties(
						plant_level: Grass,
						local_max_quantity: 1000000,
						materials: [
							Plant(
								MaterialProperties(
									property: "Green fiber"
								)
							)
						],
						native_biomes: [
							"biome_boreal_grassland",
							"biome_temperate_grassland",
							"biome_tropical_grassland",
							"biome_boreal_woodland",
							"biome_temperate_woodland",
							"biome_tropical_woodland",
							"biome_boreal_forest",
							"biome_temperate_forest",
							"biome_tropical_forest",
							"biome_boreal_swamp",
							"biome_temperate_swamp",
							"biome_tropical_swamp",
							"biome_boreal_shrubland",
							"biome_temperate_shrubland",
							"biome_tropical_shrubland",
							"biome_boreal_alpine_grassland",
							"biome_temperate_alpine_grassland",
							"biome_tropical_alpine_grassland",
						],
					)
				)
			),
		]
		"#;

	// LOAD AND PARSE ON STARTUP.
	// In worldgen.
	let path_placeholder = "./preset/some_preset...";
	let entities_from_file: Vec<EntityData> = match ron::from_str(&data) {
		Ok(f) => f,
		Err(e) => {
			println!("{}: {}", "ERROR: ".red(), e.to_string().red());
			println!("Flie: {}", path_placeholder);
			println!("Check missing commas in preset.");
			println!("Check if all option names are valid.");
			std::process::exit(0);
		}
	};

	let mut plant_types = Vec::new();
	let mut plant_types_codenames = Vec::new();
	for entity_from_file in entities_from_file {
		if !plant_types_codenames.contains(&entity_from_file.entity_codename) {
			match &entity_from_file.entity_type {
				EntityType::Plant(_) => {
					plant_types_codenames.push(entity_from_file.entity_codename.clone());
					println!(
						"Plant type loaded: {}",
						entity_from_file.entity_codename.clone()
					);
					plant_types.push(entity_from_file);
				}
			}
		} else {
			println!(
				"{} : {}",
				"WARNING: same entity already loaded".yellow(),
				entity_from_file.entity_codename.yellow()
			);
		}
	}

	println!("Loaded plant types: {:?}", plant_types_codenames);

	let plant_patches_cap = 1_000_000;
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
	println!("{}", "CREATING ENTRIES IN THE LOCATION\n".green());
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
	// Just put everything out from plant tyoes.
	for plant_type in plant_types {
		uids_plant_patches_local.push(uid_plant_patch);
		unique_plant_patches.insert(
			uid_plant_patch,
			UniqueEntity {
				uid: uid_plant_patch,
				x,
				y,
				data: plant_type.clone(), // EntityData
			},
		);

		// Increment UID every call.
		uid_plant_patch = uid_plant_patch
			.checked_add(1)
			.expect("ERROR: overflow at  UID increment.");
	}

	// PUTTING A BATCH OF ENTITIES ON MAP.
	plant_patch_cache.insert(ind, uids_plant_patches_local);
	println!(
		"Total number of plant patches in the world: {}",
		unique_plant_patches.len()
	);
	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:
	let local_entities = &plant_patch_cache[&ind];
	println!("{}", "CHECKING ENTRIES IN THE LOCATION\n".green());
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	for entity_id in local_entities.iter() {
		// Destruct the entity. What should this return? How and when?
		// Check if entity exists.
		match unique_plant_patches.get_mut(entity_id) {
			Some(entity) => {
				println!("---------------------");
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
						match properties.plant_level {
							PlantLevel::TallCanopy => {
								println!("TALL PLANT CANOPY LEVEL")
							}
							PlantLevel::MediumCanopy => {
								println!("MEDIUM PLANT CANOPY LEVEL")
							}
							PlantLevel::ShortCanopy => {
								println!("SHORT PLANT CANOPY LEVEL")
							}
							PlantLevel::Shrub => {
								println!("SRUB LEVEL")
							}
							PlantLevel::Grass => {
								println!("GRASS LEVEL")
							}
							PlantLevel::Root => {
								println!("ROOT LEVEL")
							}
						}
						println!("Max local quantity {}", properties.local_max_quantity);
						// Store biome data into some vec. Sort out duplicates.
						for b in &properties.native_biomes {
							if cw::BIOMES_CODENAMES.contains_key(&b.clone()) {
								let id = cw::BIOMES_CODENAMES[&b.clone()];
								println!("Native biome (id) {}: {}", id, &b.clone());
							} else {
								// Make a proper warning prompt later on.
								println!(
									"{}: {}",
									"WARNING: unknown native biome assigned".yellow(),
									b.yellow()
								);
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
