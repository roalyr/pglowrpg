use lib_constants::world as cw;
use serde::{Deserialize, Serialize};

////////////////////////////////////////////////////////////////////////////////
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

////////////////////////////////////////////////////////////////////////////////
// MATERIALS
// TODO: generalize materials and define them via preset too.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct MaterialProperties {
	pub property: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum Material {
	Wood,
	Fiber,
}

////////////////////////////////////////////////////////////////////////////////
// FLORA
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlantComponentType {
	Root,
	Trunk,
	Stem,
	Branch,
	Leaf,
	Flower,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlantChildComponent {
	Components(Vec<Box<PlantComponent>>),
	None,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum PlantComponentQuantity {
	Single,
	Few,
	Many,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantComponent {
	pub component: PlantComponentType,
	pub quantity: PlantComponentQuantity,
	pub material: Material,
	pub child_components: PlantChildComponent,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantProperties {
	pub plant_level: cw::PlantLevel,
	pub local_max_quantity: u16, // u16 should be enough.
	//days to grow
	//dimensions when fully grown (heights, width)
	//dimension variation
	//volume
	pub plant_components: PlantComponent,
	pub native_biomes: Vec<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlantGroup {
	// Rounding to 4 bytes.
	pub type_uid: u16, // links to EntityData directly.
	pub quantity: u16, // Can be u16 due to rounding.
}
