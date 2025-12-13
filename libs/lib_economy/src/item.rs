// [Revision: v0.1] [Path: libs/lib_economy/src/item.rs] [Date: 2025-12-13]
// Description: Item definitions and types

use serde::{Deserialize, Serialize};

/// Category of item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ItemType {
    /// Weapons for combat (swords, axes, bows, etc.).
    Weapon,
    /// Armor and protective gear.
    Armor,
    /// Consumable items (potions, food, medicine).
    Consumable,
    /// Raw materials for crafting or trade.
    Material,
    /// Quest-related items that cannot be sold/dropped.
    Quest,
    /// Tools and utility items.
    Tool,
    /// Trinkets, jewelry, and valuables.
    Treasure,
}

impl ItemType {
    /// Display name for item type.
    pub fn name(&self) -> &'static str {
        match self {
            ItemType::Weapon => "Weapon",
            ItemType::Armor => "Armor",
            ItemType::Consumable => "Consumable",
            ItemType::Material => "Material",
            ItemType::Quest => "Quest Item",
            ItemType::Tool => "Tool",
            ItemType::Treasure => "Treasure",
        }
    }
    
    /// Whether this item type can be sold.
    pub fn can_sell(&self) -> bool {
        !matches!(self, ItemType::Quest)
    }
    
    /// Whether this item type can be dropped.
    pub fn can_drop(&self) -> bool {
        !matches!(self, ItemType::Quest)
    }
}

/// Rarity tier of an item.
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize, Deserialize)]
pub enum ItemRarity {
    /// Common items found everywhere.
    Common,
    /// Uncommon items requiring some effort to find.
    Uncommon,
    /// Rare items found in dangerous places.
    Rare,
    /// Epic items of legendary quality.
    Epic,
    /// Unique items, only one exists.
    Unique,
}

impl ItemRarity {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            ItemRarity::Common => "Common",
            ItemRarity::Uncommon => "Uncommon",
            ItemRarity::Rare => "Rare",
            ItemRarity::Epic => "Epic",
            ItemRarity::Unique => "Unique",
        }
    }
    
    /// Value multiplier based on rarity.
    pub fn value_multiplier(&self) -> f32 {
        match self {
            ItemRarity::Common => 1.0,
            ItemRarity::Uncommon => 2.0,
            ItemRarity::Rare => 5.0,
            ItemRarity::Epic => 15.0,
            ItemRarity::Unique => 50.0,
        }
    }
}

/// Equipment slot for wearable items.
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum EquipmentSlot {
    /// Main hand weapon.
    MainHand,
    /// Off-hand weapon or shield.
    OffHand,
    /// Head armor (helmet, hood).
    Head,
    /// Body armor (mail, leather).
    Body,
    /// Hand armor (gloves, gauntlets).
    Hands,
    /// Foot armor (boots).
    Feet,
    /// Cloak or cape.
    Back,
    /// Amulet or necklace.
    Neck,
    /// Rings (can have multiple).
    Ring,
}

impl EquipmentSlot {
    /// Display name.
    pub fn name(&self) -> &'static str {
        match self {
            EquipmentSlot::MainHand => "Main Hand",
            EquipmentSlot::OffHand => "Off Hand",
            EquipmentSlot::Head => "Head",
            EquipmentSlot::Body => "Body",
            EquipmentSlot::Hands => "Hands",
            EquipmentSlot::Feet => "Feet",
            EquipmentSlot::Back => "Back",
            EquipmentSlot::Neck => "Neck",
            EquipmentSlot::Ring => "Ring",
        }
    }
}

/// Equipment properties for wearable items.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Equipment {
    /// Which slot this equipment occupies.
    pub slot: EquipmentSlot,
    /// Armor bonus (damage reduction).
    pub armor: i8,
    /// Weapon damage bonus.
    pub damage: i8,
    /// Stat modifiers when equipped.
    pub stat_bonuses: Vec<(String, i8)>,
}

impl Equipment {
    /// Create new equipment.
    pub fn new(slot: EquipmentSlot) -> Self {
        Self {
            slot,
            armor: 0,
            damage: 0,
            stat_bonuses: Vec::new(),
        }
    }
    
    /// Set armor value.
    pub fn with_armor(mut self, armor: i8) -> Self {
        self.armor = armor;
        self
    }
    
    /// Set damage value.
    pub fn with_damage(mut self, damage: i8) -> Self {
        self.damage = damage;
        self
    }
    
    /// Add stat bonus.
    pub fn with_stat_bonus(mut self, stat: impl Into<String>, bonus: i8) -> Self {
        self.stat_bonuses.push((stat.into(), bonus));
        self
    }
}

/// An item that can be carried, used, or traded.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Item {
    /// Unique identifier for item type.
    pub id: String,
    /// Display name.
    pub name: String,
    /// Description text.
    pub description: String,
    /// Category.
    pub item_type: ItemType,
    /// Rarity tier.
    pub rarity: ItemRarity,
    /// Base value in currency units.
    pub base_value: u32,
    /// Weight in arbitrary units.
    pub weight: f32,
    /// Maximum stack size (1 for unstackable).
    pub max_stack: u32,
    /// Equipment properties if this is equipment.
    pub equipment: Option<Equipment>,
    /// Special tags for filtering/effects.
    pub tags: Vec<String>,
}

impl Item {
    /// Create a new item.
    pub fn new(
        id: impl Into<String>,
        name: impl Into<String>,
        item_type: ItemType,
    ) -> Self {
        Self {
            id: id.into(),
            name: name.into(),
            description: String::new(),
            item_type,
            rarity: ItemRarity::Common,
            base_value: 1,
            weight: 1.0,
            max_stack: 1,
            equipment: None,
            tags: Vec::new(),
        }
    }
    
    /// Set description.
    pub fn with_description(mut self, desc: impl Into<String>) -> Self {
        self.description = desc.into();
        self
    }
    
    /// Set rarity.
    pub fn with_rarity(mut self, rarity: ItemRarity) -> Self {
        self.rarity = rarity;
        self
    }
    
    /// Set base value.
    pub fn with_value(mut self, value: u32) -> Self {
        self.base_value = value;
        self
    }
    
    /// Set weight.
    pub fn with_weight(mut self, weight: f32) -> Self {
        self.weight = weight;
        self
    }
    
    /// Set max stack size.
    pub fn with_stack_size(mut self, size: u32) -> Self {
        self.max_stack = size;
        self
    }
    
    /// Set equipment properties.
    pub fn with_equipment(mut self, equipment: Equipment) -> Self {
        self.equipment = Some(equipment);
        self
    }
    
    /// Add a tag.
    pub fn with_tag(mut self, tag: impl Into<String>) -> Self {
        self.tags.push(tag.into());
        self
    }
    
    /// Get actual value (base * rarity multiplier).
    pub fn value(&self) -> u32 {
        (self.base_value as f32 * self.rarity.value_multiplier()) as u32
    }
    
    /// Check if item can stack.
    pub fn is_stackable(&self) -> bool {
        self.max_stack > 1
    }
    
    /// Check if item is equipment.
    pub fn is_equipment(&self) -> bool {
        self.equipment.is_some()
    }
    
    /// Check if item has a specific tag.
    pub fn has_tag(&self, tag: &str) -> bool {
        self.tags.iter().any(|t| t == tag)
    }
}

// === Predefined Item Templates ===

/// Create a basic sword.
pub fn sword() -> Item {
    Item::new("sword", "Iron Sword", ItemType::Weapon)
        .with_description("A reliable iron sword.")
        .with_value(10)
        .with_weight(3.0)
        .with_equipment(Equipment::new(EquipmentSlot::MainHand).with_damage(2))
}

/// Create a basic shield.
pub fn shield() -> Item {
    Item::new("shield", "Wooden Shield", ItemType::Armor)
        .with_description("A sturdy wooden shield.")
        .with_value(8)
        .with_weight(4.0)
        .with_equipment(Equipment::new(EquipmentSlot::OffHand).with_armor(1))
}

/// Create leather armor.
pub fn leather_armor() -> Item {
    Item::new("leather_armor", "Leather Armor", ItemType::Armor)
        .with_description("Light leather armor.")
        .with_value(15)
        .with_weight(5.0)
        .with_equipment(Equipment::new(EquipmentSlot::Body).with_armor(1))
}

/// Create chainmail armor.
pub fn chainmail() -> Item {
    Item::new("chainmail", "Chainmail", ItemType::Armor)
        .with_description("Heavy chainmail armor.")
        .with_value(50)
        .with_weight(12.0)
        .with_rarity(ItemRarity::Uncommon)
        .with_equipment(Equipment::new(EquipmentSlot::Body).with_armor(2))
}

/// Create a health potion.
pub fn health_potion() -> Item {
    Item::new("health_potion", "Health Potion", ItemType::Consumable)
        .with_description("Restores health when consumed.")
        .with_value(5)
        .with_weight(0.5)
        .with_stack_size(10)
        .with_tag("healing")
}

/// Create rations.
pub fn rations() -> Item {
    Item::new("rations", "Rations", ItemType::Consumable)
        .with_description("Dried food for travel.")
        .with_value(2)
        .with_weight(1.0)
        .with_stack_size(20)
        .with_tag("food")
}

/// Create iron ore.
pub fn iron_ore() -> Item {
    Item::new("iron_ore", "Iron Ore", ItemType::Material)
        .with_description("Raw iron ore for smelting.")
        .with_value(3)
        .with_weight(2.0)
        .with_stack_size(50)
}

/// Create gold coins.
pub fn gold_coins() -> Item {
    Item::new("gold_coins", "Gold Coins", ItemType::Treasure)
        .with_description("Currency of the realm.")
        .with_value(1)
        .with_weight(0.01)
        .with_stack_size(9999)
}

/// Create a torch.
pub fn torch() -> Item {
    Item::new("torch", "Torch", ItemType::Tool)
        .with_description("Provides light in dark places.")
        .with_value(1)
        .with_weight(0.5)
        .with_stack_size(5)
        .with_tag("light")
}

/// Create a rope.
pub fn rope() -> Item {
    Item::new("rope", "Hemp Rope", ItemType::Tool)
        .with_description("50 feet of sturdy rope.")
        .with_value(3)
        .with_weight(5.0)
        .with_stack_size(1)
}
