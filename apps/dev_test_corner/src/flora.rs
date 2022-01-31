use colored::*;
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

	// Where to store all these enums and structs?
	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct MaterialProperties {
		property: String,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	enum Material {
		Plant(MaterialProperties),
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	// From the forest structure by height, from highest to lowest.
	// Simplified names for uniformity.
	enum PlantLevel {
		TallCanopy,
		MediumCanopy,
		ShortCanopy,
		Shrub,
		Ground,
		Underground,
		Underwater,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct PlantProperties {
		plant_level: PlantLevel,
		local_max_quantity: u32,
		materials: Vec<Material>,
		native_biomes: Vec<String>,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	enum EntityType {
		Plant(PlantProperties),
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct EntityData {
		entity_codename: String,
		entity_type: EntityType,
	}

	// Not needed for plants and groups here because it copies too much
	// non-unique data.
	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct UniqueEntity {
		uid: u32,
		x: u32,
		y: u32,
		data: EntityData,
	}

	#[derive(Debug, Clone, Serialize, Deserialize)]
	struct PlantGroup {
		type_uid: u16, // links to EntityData directly.
		quantity: u16, // Can be u16 due to rounding (u8 = 1 byte -> 1 +2)
	}

	//Check sizes of structures.
	println!("----------------------");
	println!(
		"PlantGroup size (bytes): {:?}",
		std::mem::size_of::<PlantGroup>()
	);

	// Total number of entities in world.
	// For 4k world, estimation:
	let plant_types_on_level = cw::PLANT_LEVEL_TYPES_NUMBER_MAX;
	let map_sizes = [256, 512, 1024, 2048, 4096];
	for map_size in map_sizes {
		let world_size: u32 = map_size * map_size;
		// Let's assume 50% of world area is underwater, and there is only
		// one level: 'Underwater".
		let num_water = world_size / 2 * plant_types_on_level as u32;
		// For 10% of ground we assume there is all 6 levels:
		let num_all_levels = world_size / 10 * plant_types_on_level as u32 * 6;
		// For rest 40% of the world we assume 3 levels are occupied:
		let num_sparse = world_size / 10 * 4 * plant_types_on_level as u32 * 3;

		let plant_groups_cap = num_sparse + num_all_levels + num_water;
		let plant_layer_mem =
			std::mem::size_of::<PlantGroup>() as u32 * plant_groups_cap / 1_000_000;
		println!("\nMap size (one side): {:?}", map_size);
		println!("World size (cells): {:?}", world_size);
		println!(
			"Estimated number of plant groups in world: {:?}",
			plant_groups_cap
		);
		println!(
			"Estimated memory taken by flora layer (MB): {:?}\n",
			plant_layer_mem
		);
		//let items_cap = 1_000_000;
		//...

		println!("----------------------");
	}

	// Just out of loop vals.
	let map_size = 4096;
	let world_size = map_size * map_size;
	let plant_groups_cap = 100000;

	println!("{}", "\nREADING DATA FILE\n".green());

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
						// Plant level along with world constant and scarcity
						// will define the "mix" of plants in given location at
						// given plant level.
						plant_level: Ground,
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
						plant_level: Ground,
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
						plant_level: Ground,
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

	// This should be a hashmap that connects type_UIDs with type data.
	// Type UIDs u16 is enough.
	let mut plant_types: HashMap<u16, EntityData> = HashMap::with_capacity(1000); // some plant type cap.
																																							// Codenames list needed to check for duplicates only.
	let mut plant_types_codenames = Vec::new();
	let mut uid_plant_type: u16 = 1;

	for entity_from_file in entities_from_file {
		if !plant_types_codenames.contains(&entity_from_file.entity_codename) {
			match &entity_from_file.entity_type {
				EntityType::Plant(_) => {
					plant_types_codenames.push(entity_from_file.entity_codename.clone());
					println!(
						"Plant type loaded: {}",
						entity_from_file.entity_codename.clone()
					);
					plant_types.insert(uid_plant_type, entity_from_file);

					// Increment UID every successfull load.
					uid_plant_type = uid_plant_type
						.checked_add(1)
						.expect("ERROR: overflow at type UID increment.");
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

	// This stores unique entities..
	// This approach is wrong for flora - it stores too much non-unique data
	// Which can be just linked by type ID.
	//let mut unique_plant_groups: HashMap<u32, UniqueEntity> =
	//	HashMap::with_capacity(plant_groups_cap);

	// This approach is better.
	// UID, [type UID, quantity].
	// UID can be a lot, because of world size.
	// Type UID can be u16 at most.
	// Quantity u8.
	// One cell is 8 bytes.
	// List of all groups of plants in world, with their quantity.
	let mut unique_plant_groups: HashMap<u32, PlantGroup> =
		HashMap::with_capacity(plant_groups_cap as usize);

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
	let cache_table_size = (world_size) as usize;

	// CACHE TABLE.
	// Created at worldgen.
	// ~ 70 MB at 4k world
	// Index, [UIDs].
	let mut plants_cache: HashMap<u32, Vec<u32>> =
		HashMap::with_capacity(cache_table_size);

	// let mut plants_cache...
	// let mut items_cache...

	// This is the record of UIDs of creatures in the local coords.
	// Temporary value, stores entities at given location.
	let mut uids_plant_groups_local = Vec::new();

	// Location (within entity generator loop)
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Creating instances.
	uids_plant_groups_local.push(1);
	unique_plant_groups.insert(
		1, // u32 UID
		PlantGroup {
			type_uid: 1,   //u16
			quantity: 255, //u8
		},
	);

	uids_plant_groups_local.push(2);
	unique_plant_groups.insert(
		2, // u32 UID
		PlantGroup {
			type_uid: 2,
			quantity: 255,
		},
	);

	// PUTTING A BATCH OF ENTITIES ON MAP.
	plants_cache.insert(ind, uids_plant_groups_local);
	println!(
		"Total number of plant patches in the world: {}",
		unique_plant_groups.len()
	);
	// MAKE DESTRUCTORS FOR SPEIFIC UNIQUE ENTITY TYPES.

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:
	let local_entities = &plants_cache[&ind];
	println!("{}", "CHECKING ENTRIES IN THE LOCATION\n".green());
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	for entity_id in local_entities.iter() {
		// Destruct the entity. What should this return? How and when?
		// Check if entity exists.
		match unique_plant_groups.get_mut(entity_id) {
			Some(entity) => {
				// Entity is a PlantGroup.
				println!("---------------------");
				println!("type UID: {}", entity.type_uid);
				// Use type_uid to get type data from types.
				let plant_type_data = &plant_types[&entity.type_uid];
				// This is the link to actual data that is later destructed.
				match &plant_type_data.entity_type {
					// Have different destructors for different types.
					EntityType::Plant(properties) => {
						println!("Type: {:?}", &plant_type_data.entity_codename);
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
							PlantLevel::Ground => {
								println!("GROUND LEVEL")
							}
							PlantLevel::Underground => {
								println!("UNDERGROUND LEVEL")
							}
							PlantLevel::Underwater => {
								println!("UNDERWATER LEVEL")
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
