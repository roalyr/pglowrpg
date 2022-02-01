use colored::*;
use lib_constants::generic as cg;
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

	// Copied to codec.
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
	let mut uid_plant_type: u16 = cg::UID_MIN_U16;

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

	// TEST
	// Making a non-spatial cache table to put specific creatures
	// onto the map in specific locations.
	// Cache table is related to map size.
	println!("{}", "CREATING ENTRIES IN THE LOCATION\n".green());
	let cachemap_size = (world_size * 5) as usize;

	// U32 is Ind (position) and this way by it we can index the respective
	// vector, containing plant groups.
	let mut data_from_cachemap: HashMap<u32, Vec<PlantGroup>> =
		HashMap::with_capacity(cachemap_size);

	// let mut data_from_cachemap...
	// let mut items_cache...

	// Location (within entity generator loop)
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Creating instances.
	data_from_cachemap.insert(
		ind,
		vec![
			PlantGroup {
				type_uid: 1,
				quantity: 25,
			},
			PlantGroup {
				type_uid: 1,
				quantity: 255,
			},
			PlantGroup {
				type_uid: 2,
				quantity: 2550,
			},
		],
	);

	println!(
		"Total number of plant groups in the world: {}",
		data_from_cachemap.len()
	);

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:
	println!("{}", "CHECKING ENTRIES IN THE LOCATION\n".green());
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	let plant_groups = &data_from_cachemap[&ind];
	for group in plant_groups.iter() {
		// Destruct the entity. What should this return? How and when?
		// Entity is a PlantGroup.
		println!("---------------------");
		println!("type UID: {}", group.type_uid);
		println!("quantity: {}", group.quantity);
		// Use type_uid to get type data from types.
		let plant_type_data = &plant_types[&group.type_uid];
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

	// Implement I/O functions for the cache map.
}
