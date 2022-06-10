use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_game_data_codec as gdc;
use lib_game_data_codec::LayerPack;
use lib_io_ops::readron::presets::presets_flora;
use std::collections::HashMap;

// Plants should be described in patches or groups, that have
// UIDs, and region of origin (assigned at worldgen).
// This allows for unified entity system to be applied.

pub fn get(lp: &mut LayerPack) {
	// Read presets from files.
	let entities_from_file = presets_flora::get();
	let plant_types_num = entities_from_file.len();

	// Create an UID-type hasmap.
	lp.flora.types = HashMap::with_capacity(plant_types_num);

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
					
					// This is a read-only table, not meant to be changed.
					lp.flora.types.insert(uid_plant_type, entity_from_file);
					//lp.flora.types.insert(uid_plant_type, entity_from_file);
					
					// TODO: Make additional hashmap "keyword" -> "uid"
					// this way one can either search by uid or keyword
					// without going for the search.
					
					// Increment UID every successfull load.
					uid_plant_type = uid_plant_type
						.checked_add(1)
						.expect("ERROR: overflow at type UID increment.");
				}
			}
		} else {
			println!(
				"{} : {}",
				"WARNING: same entity already loaded", entity_from_file.entity_codename
			);
		}
	}

	// WORLDGEN
	// Making a non-spatial cache table to put specific entities in the world.
	// Approximate number of entities per location.
	let groups_per_loc = 5;
	// Cache table is related to map size.
	let cachemap_size = (lp.layer_vec_len * groups_per_loc) as usize;

	// U32 is Ind (position), allows to index the respective entity vector.
	lp.flora.data = HashMap::with_capacity(cachemap_size);

	// Loop through the map, match by biomes, randomly drop in plants.
	let x: u32 = 128;
	let y: u32 = 128;
	let ind = lp.index.get(x, y) as u32;

	// LOADING AND WRITING ENTITIES.
	// Creating instances.
	lp.flora.data.insert(
		ind,
		vec![
			gdc::entities::PlantGroup {
				type_uid: 1, // Find the proper uid from the lp.flora.types
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
		lp.flora.data.len()
	);

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:

	// TODO: make a look-up for strings descripiti, name) in
	// locales and if there are none: give an error.

	println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	let plant_groups = &lp.flora.data[&ind];
	for group in plant_groups.iter() {
		// Destruct the entity. What should this return? How and when?
		// Entity is a PlantGroup.
		println!("---------------------");
		println!("type UID: {}", group.type_uid);
		println!("quantity: {}", group.quantity);
		// Use type_uid to get type data from types.
		let plant_type_data = &lp.flora.types[&group.type_uid];
		match &plant_type_data.entity_type {
			// Have different destructors for different types.
			gdc::entities::EntityType::Plant(properties) => {
				println!("Type: {:?}", &plant_type_data.entity_codename);
				println!("Structure: {:?}", &properties.plant_components);
				match properties.plant_level {
					cw::PlantLevel::TallCanopy => {
						println!("TALL PLANT CANOPY LEVEL")
					}
					cw::PlantLevel::MediumCanopy => {
						println!("MEDIUM PLANT CANOPY LEVEL")
					}
					cw::PlantLevel::ShortCanopy => {
						println!("SHORT PLANT CANOPY LEVEL")
					}
					cw::PlantLevel::Shrub => {
						println!("SRUB LEVEL")
					}
					cw::PlantLevel::Ground => {
						println!("GROUND LEVEL")
					}
					cw::PlantLevel::Underground => {
						println!("UNDERGROUND LEVEL")
					}
					cw::PlantLevel::Underwater => {
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
						println!("{}: {}", "WARNING: unknown native biome assigned", b);
					}
				}
			}
		}
	}

	// Implement I/O functions for the cache map.
}
