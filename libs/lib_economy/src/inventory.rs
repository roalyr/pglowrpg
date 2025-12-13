// [Revision: v0.1] [Path: libs/lib_economy/src/inventory.rs] [Date: 2025-12-13]
// Description: Inventory management system

use crate::item::{Item, ItemType, EquipmentSlot};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;

/// A stack of items in inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ItemStack {
    /// The item being stacked.
    pub item: Item,
    /// Current quantity.
    pub quantity: u32,
}

impl ItemStack {
    /// Create a new item stack.
    pub fn new(item: Item, quantity: u32) -> Self {
        Self { item, quantity }
    }
    
    /// Get total weight of this stack.
    pub fn total_weight(&self) -> f32 {
        self.item.weight * self.quantity as f32
    }
    
    /// Get total value of this stack.
    pub fn total_value(&self) -> u32 {
        self.item.value() * self.quantity
    }
    
    /// Check if more can be added to this stack.
    pub fn can_add(&self, amount: u32) -> bool {
        self.quantity + amount <= self.item.max_stack
    }
    
    /// Space remaining in stack.
    pub fn space_remaining(&self) -> u32 {
        self.item.max_stack.saturating_sub(self.quantity)
    }
}

/// Character inventory.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Inventory {
    /// Items stored by slot index.
    slots: Vec<Option<ItemStack>>,
    /// Maximum number of slots.
    max_slots: usize,
    /// Maximum weight capacity.
    max_weight: f32,
    /// Currently equipped items by slot.
    equipment: HashMap<EquipmentSlot, ItemStack>,
    /// Currency (gold).
    pub gold: u32,
}

impl Default for Inventory {
    fn default() -> Self {
        Self::new(20, 100.0)
    }
}

impl Inventory {
    /// Create a new inventory.
    pub fn new(max_slots: usize, max_weight: f32) -> Self {
        Self {
            slots: vec![None; max_slots],
            max_slots,
            max_weight,
            equipment: HashMap::new(),
            gold: 0,
        }
    }
    
    /// Get current weight of all items.
    pub fn current_weight(&self) -> f32 {
        let inventory_weight: f32 = self.slots.iter()
            .filter_map(|s| s.as_ref())
            .map(|s| s.total_weight())
            .sum();
        
        let equipment_weight: f32 = self.equipment.values()
            .map(|s| s.total_weight())
            .sum();
        
        inventory_weight + equipment_weight
    }
    
    /// Get remaining weight capacity.
    pub fn weight_remaining(&self) -> f32 {
        self.max_weight - self.current_weight()
    }
    
    /// Check if inventory is over encumbered.
    pub fn is_encumbered(&self) -> bool {
        self.current_weight() > self.max_weight
    }
    
    /// Get number of used slots.
    pub fn slots_used(&self) -> usize {
        self.slots.iter().filter(|s| s.is_some()).count()
    }
    
    /// Get number of free slots.
    pub fn slots_free(&self) -> usize {
        self.max_slots - self.slots_used()
    }
    
    /// Add an item to inventory.
    /// Returns the number of items that could not be added.
    pub fn add_item(&mut self, item: Item, quantity: u32) -> u32 {
        let mut remaining = quantity;
        
        // First, try to stack with existing items
        if item.is_stackable() {
            for slot in &mut self.slots {
                if remaining == 0 { break; }
                
                if let Some(stack) = slot {
                    if stack.item.id == item.id {
                        let can_add = stack.space_remaining().min(remaining);
                        stack.quantity += can_add;
                        remaining -= can_add;
                    }
                }
            }
        }
        
        // Then, use empty slots for remainder
        while remaining > 0 {
            if let Some(empty_slot) = self.slots.iter_mut().find(|s| s.is_none()) {
                let stack_amount = remaining.min(item.max_stack);
                *empty_slot = Some(ItemStack::new(item.clone(), stack_amount));
                remaining -= stack_amount;
            } else {
                // No more empty slots
                break;
            }
        }
        
        remaining
    }
    
    /// Remove items from inventory.
    /// Returns the number of items actually removed.
    pub fn remove_item(&mut self, item_id: &str, quantity: u32) -> u32 {
        let mut removed = 0;
        let mut remaining = quantity;
        
        for slot in &mut self.slots {
            if remaining == 0 { break; }
            
            if let Some(stack) = slot {
                if stack.item.id == item_id {
                    let to_remove = remaining.min(stack.quantity);
                    stack.quantity -= to_remove;
                    removed += to_remove;
                    remaining -= to_remove;
                    
                    if stack.quantity == 0 {
                        *slot = None;
                    }
                }
            }
        }
        
        removed
    }
    
    /// Get total quantity of a specific item.
    pub fn count_item(&self, item_id: &str) -> u32 {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item.id == item_id)
            .map(|s| s.quantity)
            .sum()
    }
    
    /// Check if inventory contains at least the specified quantity.
    pub fn has_item(&self, item_id: &str, quantity: u32) -> bool {
        self.count_item(item_id) >= quantity
    }
    
    /// Get all items of a specific type.
    pub fn items_by_type(&self, item_type: ItemType) -> Vec<&ItemStack> {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item.item_type == item_type)
            .collect()
    }
    
    /// Get all items with a specific tag.
    pub fn items_by_tag(&self, tag: &str) -> Vec<&ItemStack> {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .filter(|s| s.item.has_tag(tag))
            .collect()
    }
    
    /// Get all non-empty slots.
    pub fn all_items(&self) -> Vec<&ItemStack> {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .collect()
    }
    
    /// Equip an item from inventory.
    /// Returns the previously equipped item if any.
    pub fn equip(&mut self, slot_index: usize) -> Option<Item> {
        // Get the item from inventory
        let stack = self.slots.get_mut(slot_index)?.take()?;
        
        // Check if it's equipment
        let equipment = stack.item.equipment.as_ref()?;
        let equip_slot = equipment.slot;
        
        // Remove old equipment if any
        let old_item = self.equipment.remove(&equip_slot).map(|s| s.item);
        
        // Equip the new item
        self.equipment.insert(equip_slot, ItemStack::new(stack.item, 1));
        
        // If there were more in the stack, put them back
        if stack.quantity > 1 {
            self.slots[slot_index] = Some(ItemStack {
                item: self.equipment[&equip_slot].item.clone(),
                quantity: stack.quantity - 1,
            });
        }
        
        // Put old item back in inventory
        if let Some(old) = &old_item {
            self.add_item(old.clone(), 1);
        }
        
        old_item
    }
    
    /// Unequip an item to inventory.
    /// Returns true if successful.
    pub fn unequip(&mut self, slot: EquipmentSlot) -> bool {
        if let Some(stack) = self.equipment.remove(&slot) {
            self.add_item(stack.item, stack.quantity) == 0
        } else {
            false
        }
    }
    
    /// Get equipped item in a slot.
    pub fn equipped(&self, slot: EquipmentSlot) -> Option<&Item> {
        self.equipment.get(&slot).map(|s| &s.item)
    }
    
    /// Get all equipped items.
    pub fn all_equipped(&self) -> Vec<(&EquipmentSlot, &Item)> {
        self.equipment.iter().map(|(slot, stack)| (slot, &stack.item)).collect()
    }
    
    /// Calculate total armor from equipment.
    pub fn total_armor(&self) -> i8 {
        self.equipment.values()
            .filter_map(|s| s.item.equipment.as_ref())
            .map(|e| e.armor)
            .sum()
    }
    
    /// Calculate total damage bonus from equipment.
    pub fn total_damage_bonus(&self) -> i8 {
        self.equipment.values()
            .filter_map(|s| s.item.equipment.as_ref())
            .map(|e| e.damage)
            .sum()
    }
    
    /// Calculate total value of inventory (excluding equipped).
    pub fn total_value(&self) -> u32 {
        self.slots.iter()
            .filter_map(|s| s.as_ref())
            .map(|s| s.total_value())
            .sum::<u32>()
            + self.gold
    }
    
    /// Add gold.
    pub fn add_gold(&mut self, amount: u32) {
        self.gold = self.gold.saturating_add(amount);
    }
    
    /// Remove gold.
    /// Returns true if successful (had enough gold).
    pub fn remove_gold(&mut self, amount: u32) -> bool {
        if self.gold >= amount {
            self.gold -= amount;
            true
        } else {
            false
        }
    }
    
    /// Get inventory summary string.
    pub fn summary(&self) -> String {
        format!(
            "Inventory: {}/{} slots | {:.1}/{:.1} weight | {} gold",
            self.slots_used(),
            self.max_slots,
            self.current_weight(),
            self.max_weight,
            self.gold
        )
    }
}
