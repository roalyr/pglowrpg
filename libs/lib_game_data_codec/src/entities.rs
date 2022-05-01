use serde::{Deserialize, Serialize};

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// GENERIC ENTITIES
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum EntityType {
	Plant(PlantProperties),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityData {
	pub entity_codename: String,
	pub entity_type: EntityType,
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// MATERIALS
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
	pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
	Plant(MaterialProperties),
}

//▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒▒
// FLORA
#[derive(Debug, Clone, Serialize, Deserialize)]
// From the forest structure by height, from highest to lowest.
// Simplified names for uniformity.

// TODO: move to constants?
pub enum PlantLevel {
	TallCanopy,
	MediumCanopy,
	ShortCanopy,
	Shrub,
	Ground,
	Underground,
	Underwater,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantProperties {
	pub plant_level: PlantLevel,
	pub local_max_quantity: u16, // u16 should be enough.
	pub materials: Vec<Material>,
	pub native_biomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantGroup {
	// Rounding to 4 bytes.
	pub type_uid: u16, // links to EntityData directly.
	pub quantity: u16, // Can be u16 due to rounding.
}
