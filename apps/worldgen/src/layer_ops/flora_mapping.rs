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

	// Codenames list needed to check for duplicates upon loading.
	let mut plant_types_codenames = Vec::new();

	// Initiate UID.
	let mut flora_type_uid: u16 = cg::UID_MIN_U16;

	// Store plant types from presets into hashmap.
	for entity_from_file in entities_from_file {
		// If not previously loaded - proceed, otherwise raise a warning.
		if !plant_types_codenames.contains(&entity_from_file.entity_codename) {
			match &entity_from_file.entity_type {
				// Make sure there are no non-plant types.
				gdc::entities::EntityType::Plant(_) => {
					let flora_codename = entity_from_file.entity_codename.clone();
					plant_types_codenames.push(flora_codename.clone());
					println!("Plant type loaded: {}", flora_codename);

					// Those are read-only tables, not meant to be changed.
					// Types connect UID with data, while codenames - UID with name.
					lp.flora
						.flora_types
						.insert(flora_type_uid, entity_from_file);
					lp.flora
						.flora_codenames
						.insert(flora_type_uid, flora_codename);

					// Increment UID every successfull load.
					flora_type_uid = flora_type_uid
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

	// We need to know which plants can be spawned in this biome type.
	// This means that we need to make a hashmap
	// k: biome_codename, v: Vec<plant_codenames>
	// In order to do so we need to iterate over all biome codenames and
	// match them with loaded plant presets.
	// This is what field 'lp.flora.flora_native_to_biomes' does.
	for (_, biome_codename) in cw::BIOMES_CODENAMES.iter() {
		//println!("{:?}", codename)

		// Gather all flora that is native to this specific biome.
		let mut native_flora = Vec::new();
		for (flora_type_uid, flora_type_data) in lp.flora.flora_types.iter() {
			// Get flora type codename from UID.
			let flora_codename = &lp.flora.flora_codenames[flora_type_uid];

			// Makes sure we are destructing flora type.
			match &flora_type_data.entity_type {
				gdc::entities::EntityType::Plant(properties) => {
					// Destruct data.
					if properties.native_biomes.contains(&biome_codename) {
						native_flora.push(flora_codename.to_string());
					}
				}
			}
		}

		// Put up a hashmap of biome : flora list kind.
		lp.flora
			.flora_native_to_biomes
			.insert(biome_codename.to_string(), native_flora);
	}

	//for (k, v) in lp.flora.flora_native_to_biomes.iter() {
	//println!("{:?}, {:?}", k, v );
	//}

	// WORLDGEN
	// Making a non-spatial cache table to put specific entities in the world.
	// Approximate number of entities per location per world size.
	let groups_per_loc = 5;
	let cachemap_size = (lp.layer_vec_len * groups_per_loc) as usize;
	lp.flora.data = HashMap::with_capacity(cachemap_size);

	// U32 is Ind (position), allows to index the respective entity vector.
	// Loop through the map, match by biomes, randomly drop in plants.
	for ind in 0..lp.layer_vec_len as usize {
		let biome_id = lp.biomes.read(ind);
		// Retrieve biome codename from the ID.
		// TODO: make it a method for 'biomes' layer?
		let mut biome_codename = String::new();
		if cw::BIOMES_CODENAMES.contains_key(&biome_id.clone()) {
			biome_codename = cw::BIOMES_CODENAMES[&biome_id.clone()].clone();
		} else {
			println!("{}: {}", "ERROR: unknown biome parsed", biome_id);
			panic!()
		}

		// Now for each world cell we have a respective biome codename.
		// We also know what plants are allowed to be spawned there.
		// Now it boils down to populating the 'data' field of the cachemap.
		let mut local_flora_batch = Vec::new();

		// TODO: flora generation happens here.
		local_flora_batch.push(gdc::entities::PlantGroup {
			type_uid: 1, // Find the proper uid from the lp.flora.types
			quantity: 25,
		});

		// Write down generated cell data.
		lp.flora.data.insert(ind as u32, local_flora_batch);
	}

	println!(
		"Total number of plant groups in the world: {}",
		lp.flora.data.len()
	);

	// TEST. READING AND PARSING.
	// Now access the creatures in the given location:

	// TODO: make a look-up for strings descripiti, name) in
	// locales and if there are none: give an error.

	//println!("At x: {}, y: {}, index: {}\n", x, y, ind);
	let ind = 1000;
	let plant_groups = &lp.flora.data[&(ind as u32)];
	for group in plant_groups.iter() {
		// Destruct the entity. What should this return? How and when?
		// Entity is a PlantGroup.
		println!("---------------------");
		println!("type UID: {}", group.type_uid);
		println!("quantity: {}", group.quantity);
		// Use type_uid to get type data from types.
		let plant_type_data = &lp.flora.flora_types[&group.type_uid];
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
					if cw::BIOMES_IDS.contains_key(&b.clone()) {
						let id = cw::BIOMES_IDS[&b.clone()];
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
