#![allow(non_snake_case)]
use io_ops::readron::presets;
use serde::{Deserialize, Serialize};
use unit_systems::coords::Index;

//▒▒▒▒▒▒▒▒▒▒▒▒ LAYER PACK ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[derive(Serialize, Deserialize, Debug)]
pub struct LayerPack {
	pub index: Index,
	pub wi: presets::presets_worldgen::Stuff,

	pub layer_vec_len: u32,
	pub noisemap_vec_len: u32,

	pub biomes: BitLayerBiomes,
	pub rivers_id: BitLayerRiversID,
	pub georeg_id: BitLayerGeoregID,
	pub topography: BitLayerTopography,
	pub climate: BitLayerClimate,
	pub rivers: BitLayerRivers,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ Layers ▒▒▒▒▒▒▒▒▒▒▒▒▒
#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerTopography {
	pub data: Vec<u16>,
	//Masks
	pub TERRAIN: u16,
	pub WATERMASK: u16,
	pub _placeholder: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerClimate {
	pub data: Vec<u16>,
	//Masks
	pub TEMPERATURE: u16,
	pub RAINFALL: u16,
}

#[derive(Serialize, Deserialize, Debug)]
pub struct BitLayerRivers {
	pub data: Vec<u16>,
	//Masks
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
pub struct BitLayerGeoregID {
	pub data: Vec<u32>,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ METHODS ▒▒▒▒▒▒▒▒▒▒▒▒▒
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

//▒▒▒▒▒▒▒▒ BIT LAYER STORAGE ▒▒▒▒▒▒▒▒▒▒▒▒
// Credits to ZippyMagician from "One Lone Coder" community
// for initial draft of this macro.
// This macro initiates a bit layer structure.
#[macro_export]
macro_rules! bit_layer {
	//general case with masks
	($layer_val_type: ty, $length: expr,
	[$($mask_name: ident : $mask_val: expr), *]) => {
	{
		#[derive(Debug)]
		struct Masks {
			$($mask_name : $layer_val_type), *
		}
		#[derive(Debug)]
		struct BitLayer {
			data: Vec<$layer_val_type>,
			masks: Masks,
		}
		impl BitLayer {
			fn new(
				length: usize
			) -> Self {
				Self {
					data: vec![0; length],
					masks: Masks{ $($mask_name : $mask_val), * },
				}
			}
			pub fn write(
				&mut self,
				val: $layer_val_type,
				mask: $layer_val_type,
				index: usize,
			) {
				let zeros = mask.trailing_zeros();
				//overflow guard
				if val > (mask >> zeros) {
					panic!("bit layer value overflow for mask {:#0b}", mask)
				}
				self.data[index] |= ((mask >> zeros) & val) << zeros;
			}
			pub fn read(
				&self,
				mask: $layer_val_type,
				index: usize,
			) -> $layer_val_type {
				(self.data[index] & mask) >> mask.trailing_zeros()
			}
		}
		BitLayer::new($length)
		}
	};
	//whole-value case
	($layer_val_type: ty, $length: expr) => {
	{
		#[derive(Debug)]
		struct BitLayer {
			data: Vec<$layer_val_type>,
		}
		impl BitLayer {
			fn new(
				length: usize
			) -> Self {
				Self {
					data: vec![0; length],
				}
			}
			pub fn write(
				&mut self,
				val: $layer_val_type,
				index: usize,
			) {
				//overflow guard
				let max_val = <$layer_val_type>::MAX;
				if val > max_val {
					panic!("bit layer value overflow for type max value {}", max_val)
				}
				self.data[index] = val;
			}
			pub fn read(
				&self,
				index: usize,
			) -> $layer_val_type {
				self.data[index]
			}
		}
		BitLayer::new($length)
		}
	};
}

//▒▒▒▒▒▒▒▒▒▒▒▒ INIT ▒▒▒▒▒▒▒▒▒▒▒▒▒
// Set up laysers which have the masks to i/o different kinds of data.
impl_with_masks!(
	BitLayerRivers, u16;
	BitLayerClimate, u16;
	BitLayerTopography, u16;
);

// Simple data layers without masks.
impl_without_masks!(
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
