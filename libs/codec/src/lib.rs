use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize)]
//▒▒▒▒▒▒▒▒▒▒▒▒ Masks ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub struct MasksTopography {
	pub terrain: u16,
	pub watermask: u16,
	pub _placeholder: u16,
}

pub struct MasksClimate {
	pub temperature: u16,
	pub rainfall: u16,
}

pub struct MasksRivers {
	pub element: u16,
	pub width: u16,
	pub upstream: u16,
	pub downstream: u16,
	pub _placeholder: u16,
}

//▒▒▒▒▒▒▒▒▒▒▒▒ Layers ▒▒▒▒▒▒▒▒▒▒▒▒▒
pub struct BitLayerTopography {
	pub data: Vec<u16>,
	pub masks: MasksTopography,
}

pub struct BitLayerClimate {
	pub data: Vec<u16>,
	pub masks: MasksClimate,
}

pub struct BitLayerRivers {
	pub data: Vec<u16>,
	pub masks: MasksRivers,
}

pub struct BitLayerBiomes {
	pub data: Vec<u8>,
}

pub struct BitLayerRiversID {
	pub data: Vec<u16>,
}

pub struct BitLayerGeoregID {
	pub data: Vec<u16>,
}

macro_rules! impl_with_masks {
    ($($struct_type:ty, $val_type:ty);*) => {
        $(impl $struct_type {
		pub fn write(
				&mut self,
				val: $val_type,
				mask: $val_type,
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
				mask: $val_type,
				index: usize,
			) -> $val_type {
				(self.data[index] & mask) >> mask.trailing_zeros()
			}
        })*
    }
}

macro_rules! impl_without_masks {
    ($($struct_type: ty, $val_type: ty);*) => {
        $(impl $struct_type {
		pub fn write(
				&mut self,
				val: $val_type,
				index: usize,
			) {
				//overflow guard
				let max_val = <$val_type>::MAX;
				if val > max_val {
					panic!("bit layer value overflow for type max value {}", max_val)
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

impl_with_masks!(
	BitLayerRivers, u16; 
	BitLayerClimate, u16; 
	BitLayerTopography, u16);

impl_without_masks!(
	BitLayerGeoregID, u16; 
	BitLayerRiversID, u16; 
	BitLayerBiomes, u8);

//▒▒▒▒▒▒▒▒ BIT LAYER STORAGE ▒▒▒▒▒▒▒▒▒▒▒▒
//credits to ZippyMagician from "One Lone Coder" community
//for initial draft of this macro.

#[macro_export]
macro_rules! bit_layer {

	//general case with masks
	($layer_val_type: ty,
	$length: expr,
	[$($mask_name: ident : $mask_val: expr), *]
	) => {{

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
		}};

	//whole-value case
	($layer_val_type: ty,
	$length: expr
	) => {{

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
		}};

}

//For table maps
// vector of vectors filled with different things

//For cache maps
// vector of vectors with IDs

//that's just queue
//use Vec::with_capacity
//expand it in chunks.

//need a queue for every layer

//caches not linked to maps and tables need a queue to track freed IDs
