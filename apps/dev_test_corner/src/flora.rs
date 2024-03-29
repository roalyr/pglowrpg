use dep::colored::*;
use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_game_data_codec as gdc;
use lib_io_ops::readron::presets::presets_flora;
use lib_unit_systems::coords::Index;
use std::collections::HashMap;

// Plants should be described in patches or groups, that have
// UIDs, and region of origin (assigned at worldgen).
// This allows for unified entity system to be applied.

pub fn start() {
	// Initial values which are required.
	let map_size = 4096;
	let world_size = map_size * map_size;

	// Read presets from files.
	let entities_from_file = presets_flora::get();
	let plant_types_num = entities_from_file.len();
	
	// Create an UID- type hasmap.
	let mut plant_types: HashMap<u16, gdc::entities::EntityData> =
		HashMap::with_capacity(plant_types_num);

	// Codenames list needed to check for duplicates upon loading.
	let mut plant_types_codenames = Vec::new();
	
	// Initiate UID.
	let mut uid_plant_type: u16 = cg::UID_MIN_U16;

	// Store plant types from presets into hashmap.
	for entity_from_file in entities_from_file {
		if !plant_types_codenames.contains(&entity_from_file.entity_codename) {
			match &entity_from_file.entity_type {
				gdc::entities::EntityType::Plant(_) => {
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
				"WARNING: same entity already loaded",
				entity_from_file.entity_codename
			);
		}
	}

	// WORLDGEN
	// Making a non-spatial cache table to put specific entities in the world.
	// Approximate number of entities per location.
	let groups_per_loc = 5; 
	// Cache table is related to map size.
	let cachemap_size = (world_size * groups_per_loc) as usize;

	// U32 is Ind (position), allows to index the respective entity vector.
	let mut flora_cachemap: HashMap<u32, Vec<gdc::entities::PlantGroup>> =
		HashMap::with_capacity(cachemap_size);

	// Loop through the map, match by biomes, randomly drop in plants.
	let x: u32 = 10;
	let y: u32 = 110;
	let index = Index { map_size };
	let ind = index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Creating instances.
	flora_cachemap.insert(
		ind,
		vec![
			gdc::entities::PlantGroup {
				type_uid: 1,
				quantity: 25,
			},
			gdc::entities::PlantGroup {
				type_uid: 1,
				quantity: 255,
			},
			gdc::entities::PlantGroup {
				type_uid: 2,
				quantity: 2550,
			},
		],
	);

	println!(
		"Total number of plant groups in the world: {}",
		flora_cachemap.len()
	);

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:
	println!("{}", "CHECKING ENTRIES IN THE LOCATION\n".green());
	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	let plant_groups = &flora_cachemap[&ind];
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
			gdc::entities::EntityType::Plant(properties) => {
				println!("Type: {:?}", &plant_type_data.entity_codename);
				// Move it into plant destructor.
				for material in properties.materials.iter() {
					// Move into material destructor.
					match material {
						gdc::entities::Material::Plant(properties) => {
							println!("{:?}", properties);
						}
					}
				}
				match properties.plant_level {
					gdc::entities::PlantLevel::TallCanopy => {
						println!("TALL PLANT CANOPY LEVEL")
					}
					gdc::entities::PlantLevel::MediumCanopy => {
						println!("MEDIUM PLANT CANOPY LEVEL")
					}
					gdc::entities::PlantLevel::ShortCanopy => {
						println!("SHORT PLANT CANOPY LEVEL")
					}
					gdc::entities::PlantLevel::Shrub => {
						println!("SRUB LEVEL")
					}
					gdc::entities::PlantLevel::Ground => {
						println!("GROUND LEVEL")
					}
					gdc::entities::PlantLevel::Underground => {
						println!("UNDERGROUND LEVEL")
					}
					gdc::entities::PlantLevel::Underwater => {
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
