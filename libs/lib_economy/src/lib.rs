// [Revision: v0.1] [Path: libs/lib_economy/src/lib.rs] [Date: 2025-12-13]
// Description: Economy system - items, inventory, and trade

pub mod item;
pub mod inventory;
pub mod trade;

pub use item::{Item, ItemType, ItemRarity, Equipment, EquipmentSlot};
pub use inventory::Inventory;
pub use trade::{TradeDeal, TradeOffer};
