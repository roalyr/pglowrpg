use lib_constants::generic as cg;
use lib_constants::world as cw;
use lib_game_data_codec as gdc;
use lib_game_data_codec::LayerPack;
use lib_io_ops::readron::presets::presets_flora;
use lib_unit_systems::translate;
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

		// Init
		let mut flora_batch = Vec::new();

		for (flora_type_uid, flora_type_data) in lp.flora.flora_types.iter() {
			// Get flora type codename from UID.
			let flora_codename = &lp.flora.flora_codenames[flora_type_uid];

			// Makes sure we are destructing flora type.
			match &flora_type_data.entity_type {
				gdc::entities::EntityType::Plant(properties) => {
					// Destruct data.
					if properties.native_biomes.contains(&biome_codename) {
						// This vector of structures is used for conventience.
						// It has all the data needed for worldgen and will be sorted.
						flora_batch.push(gdc::entities::FloraBatch {
							entity_codename: flora_codename.to_string(),
							type_uid: flora_type_uid.clone(),
							scarcity: properties.scarcity,
							local_max_quantity: properties.local_max_quantity,
						});
					}
				}
			}
		}

		// Sort the flora list by scarcity.
		flora_batch.sort_by_key(|entry| entry.scarcity);

		//println!("{}", biome_codename );
		//println!("{:?}", flora_batch);
		//println!("-----", );

		// Put up a hashmap of biome : flora list kind.
		lp.flora
			.flora_sorted
			.insert(biome_codename.to_string(), flora_batch);
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
		let rain_rel = translate::get_rel(
			lp.climate.read(lp.climate.RAINFALL, ind) as f32,
			cg::ONE_F32,
			cg::ZERO_F32,
			cg::VAL_255_F32,
		);
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
		// Get the list of flora codenames suitable for this biome.
		// Vector of structs.
		let flora_sorted_this_biome =
			lp.flora.flora_sorted.get(&biome_codename).unwrap();

		// Flora generation happens here.
		// Now for each world cell we have a respective biome codename.
		// We also know what plants are allowed to be spawned there.
		// Now it boils down to populating the 'data' field of the cachemap.
		let mut local_flora_batch = Vec::new();
		let mut local_flora_batch_uids = Vec::new();
		let mut iteration: usize = 0;

		while iteration < cw::PLANT_SPAWNING_ITERATIONS {
			iteration += 1;

			for entry in flora_sorted_this_biome {
				let flora_codename = entry.entity_codename.clone();
				let flora_type_uid = entry.type_uid.clone();
				let flora_scarcity = (entry.scarcity.clone() as f32) / 100.0;
				let flora_max_quantity = entry.local_max_quantity.clone();
				let random = lib_pseudo_rng::get(
					0.0,
					1.0,
					lp.wi.seed + (flora_type_uid as u32),
					iteration + (ind as usize),
				);
				let quantity_variation = lib_pseudo_rng::get(
					0.7,
					1.3,
					lp.wi.seed + (flora_type_uid as u32) + 1,
					iteration + (ind as usize) + 1,
				) * rain_rel;
				// println!("{:?}", rain_rel);
				// Filter out duplicates.
				if !local_flora_batch_uids.contains(&flora_type_uid)
					&& random < flora_scarcity
				{
					local_flora_batch.push(gdc::entities::PlantGroup {
						type_uid: flora_type_uid, // Find the proper uid from the lp.flora.types
						quantity: ((flora_max_quantity as f32) * quantity_variation) as u16,
					});
					local_flora_batch_uids.push(flora_type_uid);
				}
			}
		}

		//println!("{:?}", local_flora_batch);

		// Write down generated cell data.
		if !local_flora_batch.is_empty() {
			lp.flora.data.insert(ind as u32, local_flora_batch);
		}
	}

	println!(
		"Total number of plant groups in the world: {}",
		lp.flora.data.len()
	);

	// Implement I/O functions for the cache map.
}
