#![allow(non_snake_case)]
pub mod entities;
use lib_constants::world as cw;
use lib_unit_systems::coords::Index;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

// TODO: move all "Stuff" data structs that correspond to .ron
// presets into this lib?

#[derive(Serialize, Deserialize, Debug)]
pub struct WorldgenPreset {
	pub seed: u32,
	pub abs_elev_min: u32,
	pub abs_elev_max: u32,
	pub abs_rain_min: u32,
	pub abs_rain_max: u32,
	pub abs_temp_min: i32,
	pub abs_temp_max: i32,
	pub waterlevel: u32,
	pub topog_scope: f32,
	pub topog_continuity: f32,
	pub topog_concentrator: f32,
	pub topog_filter: u32,
	pub topog_erosion_factor: f32,
	pub topog_noise_size: f32,
	pub topog_noise_weight: f32,
	pub temp_mode: cw::TempGrad,
	pub temp_noise_size: f32,
	pub temp_noise_weight: f32,
	pub rain_noise_size: f32,
	pub rain_noise_weight: f32,
	pub river_source_density: f32,
	pub river_heuristic_factor: f32,
	pub river_noise_size1: f32,
	pub river_noise_size2: f32,
	pub river_noise_blend: f32,
	pub river_noise_weight: f32,
	pub river_min_length: u32,
	pub river_attr_pool_size_pow: u16,
	pub river_sink_min_pool_size_pow: u16,
	pub river_erosion_width: u32,
	pub river_erosion_smooth: f32,
	pub map_size: u32,
	pub noisemap_size: u32,

	pub magic: f32,
	pub magic1: f32,
	pub magic2: f32,
	pub magic3: f32,
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// WORLD DATA STRUCTURE
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerPack {
	pub index: Index,
	pub wi: WorldgenPreset,

	pub layer_vec_len: u32,
	pub noisemap_vec_len: u32,

	// Layers. Multi-value data storages in primitive types.
	// ID values occupy whole primitive, so no masks.
	pub biomes: BitLayerBiomes,
	pub rivers_id: BitLayerRiversID,
	pub georeg_id: BitLayerGeoregID,
	pub bioreg_id: BitLayerBioregID,
	pub topography: BitLayerTopography,
	pub climate: BitLayerClimate,
	pub rivers: BitLayerRivers,

	// Cachemaps. Basically, data is stored in HashMaps instead
	// of vectors. Naturally, indexed by "ind" position respective to x, y.
	// TODO: add methods (read, write)? Should methods be unique?
	// implementation can be done at the bottom section.
	pub flora: CacheLayerFlora,
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// LAYER TYPES
// Bit layers.
#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerTopography {
	pub data: Vec<u16>,
	// Masks.
	pub TERRAIN: u16,
	pub WATERMASK: u16,
	pub _placeholder: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerClimate {
	pub data: Vec<u16>,
	// Masks.
	pub TEMPERATURE: u16,
	pub RAINFALL: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerRivers {
	pub data: Vec<u16>,
	// Masks.
	pub ELEMENT: u16,
	pub WIDTH: u16,
	pub UPSTREAM: u16,
	pub DOWNSTREAM: u16,
	pub _placeholder: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerBiomes {
	pub data: Vec<u8>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerRiversID {
	pub data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerBioregID {
	pub data: Vec<u32>,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerGeoregID {
	pub data: Vec<u32>,
}

// Cachemaps
#[derive(Serialize, Deserialize, Debug)]
pub struct CacheLayerFlora {
	pub data: HashMap<u32, Vec<entities::PlantGroup>>,
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// METHODS
// Those macros implement write and read methods for the
// bit layer structs.
macro_rules! impl_with_masks {
    ($($struct_type:ty, $val_type:ty);*;) => {
        $(impl $struct_type {
		pub fn write(
				&mut self,
				val: $val_type,
				mask: $val_type,
				index: usize,
			) {
				let zeros = mask.trailing_zeros();
				// Overflow guard.
				if val > (mask >> zeros) {
					panic!("ERROR: bit layer value overflow for mask {:#0b}", mask)
				}
				// Flush then write.
				self.data[index] = (self.data[index] & !mask) | (((mask >> zeros) & val) << zeros);
			}
			pub fn read(
				&self,
				mask: $val_type,
				index: usize,
			) -> $val_type {
				(self.data[index] & mask) >> mask.trailing_zeros()
			}
			// I don't remember why was this made. Maybe for debug.
			pub fn expose(
				&self,
				index: usize,
			) {
				println!("{:#016b}", self.data[index]);
			}
        })*
    }
}

macro_rules! impl_without_masks {
    ($($struct_type: ty, $val_type: ty);*;) => {
        $(impl $struct_type {
		pub fn write(
				&mut self,
				val: $val_type,
				index: usize,
			) {
				// Overflow guard.
				let max_val = <$val_type>::MAX;
				if val > max_val {
					panic!("ERROR: bit layer value overflow for type max value {}", max_val)
				}
				self.data[index] = val;
			}
			pub fn read(
				&self,
				index: usize,
			) -> $val_type {
				self.data[index]
			}
        })*
    }
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// INITIALIZE BITLAYERS WITH MACROS
// Set up laysers which have the masks to i/o different kinds of data.
impl_with_masks!(
	BitLayerRivers, u16;
	BitLayerClimate, u16;
	BitLayerTopography, u16;
);

// Simple data layers without masks.
impl_without_masks!(
	BitLayerBioregID, u32;
	BitLayerGeoregID, u32;
	BitLayerRiversID, u32;
	BitLayerBiomes, u8;
);

//For table maps
// vector of vectors filled with different things

//For cache maps
// vector of vectors with IDs

//that's just queue
//use Vec::with_capacity
//expand it in chunks.

//need a queue for every layer

//caches not linked to maps and tables need a queue to track freed IDs
