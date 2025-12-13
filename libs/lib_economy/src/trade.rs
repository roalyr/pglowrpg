// [Revision: v0.1] [Path: libs/lib_economy/src/trade.rs] [Date: 2025-12-13]
// Description: Trade and commerce mechanics

use crate::inventory::Inventory;
use crate::item::Item;
use serde::{Deserialize, Serialize};

/// Price modifier based on relationship/circumstances.
#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum PriceModifier {
    /// Hostile pricing (150% buy, 50% sell).
    Hostile,
    /// Unfriendly pricing (125% buy, 75% sell).
    Unfriendly,
    /// Neutral pricing (100% buy, 80% sell).
    Neutral,
    /// Friendly pricing (90% buy, 90% sell).
    Friendly,
    /// Allied pricing (80% buy, 100% sell).
    Allied,
}

impl Default for PriceModifier {
    fn default() -> Self {
        PriceModifier::Neutral
    }
}

impl PriceModifier {
    /// Multiplier when buying from merchant.
    pub fn buy_multiplier(&self) -> f32 {
        match self {
            PriceModifier::Hostile => 1.5,
            PriceModifier::Unfriendly => 1.25,
            PriceModifier::Neutral => 1.0,
            PriceModifier::Friendly => 0.9,
            PriceModifier::Allied => 0.8,
        }
    }
    
    /// Multiplier when selling to merchant.
    pub fn sell_multiplier(&self) -> f32 {
        match self {
            PriceModifier::Hostile => 0.5,
            PriceModifier::Unfriendly => 0.75,
            PriceModifier::Neutral => 0.8,
            PriceModifier::Friendly => 0.9,
            PriceModifier::Allied => 1.0,
        }
    }
}

/// An offer in a trade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeOffer {
    /// Items being offered.
    pub items: Vec<(Item, u32)>,
    /// Gold being offered.
    pub gold: u32,
}

impl TradeOffer {
    /// Create an empty offer.
    pub fn new() -> Self {
        Self {
            items: Vec::new(),
            gold: 0,
        }
    }
    
    /// Add items to offer.
    pub fn with_item(mut self, item: Item, quantity: u32) -> Self {
        self.items.push((item, quantity));
        self
    }
    
    /// Add gold to offer.
    pub fn with_gold(mut self, amount: u32) -> Self {
        self.gold = amount;
        self
    }
    
    /// Calculate total value of offer.
    pub fn total_value(&self) -> u32 {
        self.items.iter()
            .map(|(item, qty)| item.value() * qty)
            .sum::<u32>()
            + self.gold
    }
    
    /// Check if offer is empty.
    pub fn is_empty(&self) -> bool {
        self.items.is_empty() && self.gold == 0
    }
}

impl Default for TradeOffer {
    fn default() -> Self {
        Self::new()
    }
}

/// A trade deal between two parties.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TradeDeal {
    /// What the buyer offers.
    pub buyer_offer: TradeOffer,
    /// What the seller offers.
    pub seller_offer: TradeOffer,
    /// Price modifier for this trade.
    pub price_modifier: PriceModifier,
}

impl TradeDeal {
    /// Create a new trade deal.
    pub fn new() -> Self {
        Self {
            buyer_offer: TradeOffer::new(),
            seller_offer: TradeOffer::new(),
            price_modifier: PriceModifier::default(),
        }
    }
    
    /// Set buyer offer.
    pub fn buyer_offers(mut self, offer: TradeOffer) -> Self {
        self.buyer_offer = offer;
        self
    }
    
    /// Set seller offer.
    pub fn seller_offers(mut self, offer: TradeOffer) -> Self {
        self.seller_offer = offer;
        self
    }
    
    /// Set price modifier.
    pub fn with_modifier(mut self, modifier: PriceModifier) -> Self {
        self.price_modifier = modifier;
        self
    }
    
    /// Check if deal is fair (buyer value >= seller value with modifier).
    pub fn is_fair(&self) -> bool {
        let buyer_value = self.buyer_offer.total_value();
        let seller_value = (self.seller_offer.total_value() as f32 
            * self.price_modifier.buy_multiplier()) as u32;
        buyer_value >= seller_value
    }
    
    /// Calculate how much more the buyer needs to offer.
    pub fn deficit(&self) -> i32 {
        let buyer_value = self.buyer_offer.total_value() as i32;
        let seller_value = (self.seller_offer.total_value() as f32 
            * self.price_modifier.buy_multiplier()) as i32;
        seller_value - buyer_value
    }
}

impl Default for TradeDeal {
    fn default() -> Self {
        Self::new()
    }
}

/// Calculate buy price for an item.
pub fn buy_price(item: &Item, modifier: PriceModifier) -> u32 {
    (item.value() as f32 * modifier.buy_multiplier()) as u32
}

/// Calculate sell price for an item.
pub fn sell_price(item: &Item, modifier: PriceModifier) -> u32 {
    (item.value() as f32 * modifier.sell_multiplier()) as u32
}

/// Execute a simple purchase (player buys from merchant).
/// Returns true if successful.
pub fn execute_purchase(
    buyer: &mut Inventory,
    item: Item,
    quantity: u32,
    modifier: PriceModifier,
) -> bool {
    let total_cost = buy_price(&item, modifier) * quantity;
    
    // Check if buyer has enough gold
    if buyer.gold < total_cost {
        return false;
    }
    
    // Check if buyer can carry the items
    let weight = item.weight * quantity as f32;
    if buyer.weight_remaining() < weight && buyer.slots_free() == 0 {
        return false;
    }
    
    // Execute transaction
    buyer.remove_gold(total_cost);
    buyer.add_item(item, quantity);
    true
}

/// Execute a simple sale (player sells to merchant).
/// Returns the gold received if successful, None otherwise.
pub fn execute_sale(
    seller: &mut Inventory,
    item_id: &str,
    quantity: u32,
    modifier: PriceModifier,
) -> Option<u32> {
    // Get the item to calculate price
    let item = seller.all_items()
        .iter()
        .find(|s| s.item.id == item_id)?
        .item.clone();
    
    // Check if item can be sold
    if !item.item_type.can_sell() {
        return None;
    }
    
    // Check if seller has enough
    if !seller.has_item(item_id, quantity) {
        return None;
    }
    
    // Calculate sale value
    let gold_received = sell_price(&item, modifier) * quantity;
    
    // Execute transaction
    seller.remove_item(item_id, quantity);
    seller.add_gold(gold_received);
    
    Some(gold_received)
}

// ============================================================================
// UNIT TESTS
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;
    use crate::item::{Item, ItemType, ItemRarity};

    // Helper to create test items
    fn test_item(id: &str, base_value: u32) -> Item {
        Item::new(id.to_string(), id.to_string(), ItemType::Material)
            .with_rarity(ItemRarity::Common)
            .with_value(base_value)
    }

    // -------------------------------------------------------------------------
    // PriceModifier tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_price_modifier_default() {
        assert_eq!(PriceModifier::default(), PriceModifier::Neutral);
    }

    #[test]
    fn test_price_modifier_buy_multipliers() {
        assert_eq!(PriceModifier::Hostile.buy_multiplier(), 1.5);
        assert_eq!(PriceModifier::Unfriendly.buy_multiplier(), 1.25);
        assert_eq!(PriceModifier::Neutral.buy_multiplier(), 1.0);
        assert_eq!(PriceModifier::Friendly.buy_multiplier(), 0.9);
        assert_eq!(PriceModifier::Allied.buy_multiplier(), 0.8);
    }

    #[test]
    fn test_price_modifier_sell_multipliers() {
        assert_eq!(PriceModifier::Hostile.sell_multiplier(), 0.5);
        assert_eq!(PriceModifier::Unfriendly.sell_multiplier(), 0.75);
        assert_eq!(PriceModifier::Neutral.sell_multiplier(), 0.8);
        assert_eq!(PriceModifier::Friendly.sell_multiplier(), 0.9);
        assert_eq!(PriceModifier::Allied.sell_multiplier(), 1.0);
    }

    // -------------------------------------------------------------------------
    // TradeOffer tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_trade_offer_new() {
        let offer = TradeOffer::new();
        assert!(offer.items.is_empty());
        assert_eq!(offer.gold, 0);
        assert!(offer.is_empty());
    }

    #[test]
    fn test_trade_offer_with_gold() {
        let offer = TradeOffer::new().with_gold(100);
        assert_eq!(offer.gold, 100);
        assert!(!offer.is_empty());
    }

    #[test]
    fn test_trade_offer_with_item() {
        let item = test_item("ore", 10);
        let offer = TradeOffer::new().with_item(item, 5);
        
        assert_eq!(offer.items.len(), 1);
        assert!(!offer.is_empty());
    }

    #[test]
    fn test_trade_offer_total_value() {
        let item = test_item("ore", 10);
        let offer = TradeOffer::new()
            .with_item(item, 5)  // 5 * 10 = 50
            .with_gold(30);       // +30 = 80
        
        assert_eq!(offer.total_value(), 80);
    }

    // -------------------------------------------------------------------------
    // TradeDeal tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_trade_deal_new() {
        let deal = TradeDeal::new();
        assert!(deal.buyer_offer.is_empty());
        assert!(deal.seller_offer.is_empty());
        assert_eq!(deal.price_modifier, PriceModifier::Neutral);
    }

    #[test]
    fn test_trade_deal_is_fair_equal_value() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(100))
            .seller_offers(TradeOffer::new().with_item(item, 1));
        
        assert!(deal.is_fair());
    }

    #[test]
    fn test_trade_deal_is_fair_buyer_overpays() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(150))
            .seller_offers(TradeOffer::new().with_item(item, 1));
        
        assert!(deal.is_fair());
    }

    #[test]
    fn test_trade_deal_is_unfair_buyer_underpays() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(50))
            .seller_offers(TradeOffer::new().with_item(item, 1));
        
        assert!(!deal.is_fair());
    }

    #[test]
    fn test_trade_deal_hostile_modifier_increases_price() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(100))
            .seller_offers(TradeOffer::new().with_item(item, 1))
            .with_modifier(PriceModifier::Hostile);
        
        // Item worth 100, but hostile = 150% = 150 gold needed
        assert!(!deal.is_fair());
    }

    #[test]
    fn test_trade_deal_allied_modifier_decreases_price() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(80))
            .seller_offers(TradeOffer::new().with_item(item, 1))
            .with_modifier(PriceModifier::Allied);
        
        // Item worth 100, but allied = 80% = 80 gold needed
        assert!(deal.is_fair());
    }

    #[test]
    fn test_trade_deal_deficit() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(60))
            .seller_offers(TradeOffer::new().with_item(item, 1));
        
        // Need 100, have 60 = deficit of 40
        assert_eq!(deal.deficit(), 40);
    }

    #[test]
    fn test_trade_deal_surplus() {
        let item = test_item("gem", 100);
        
        let deal = TradeDeal::new()
            .buyer_offers(TradeOffer::new().with_gold(150))
            .seller_offers(TradeOffer::new().with_item(item, 1));
        
        // Need 100, have 150 = surplus of -50
        assert_eq!(deal.deficit(), -50);
    }

    // -------------------------------------------------------------------------
    // Price calculation tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_buy_price() {
        let item = test_item("sword", 100);
        
        assert_eq!(buy_price(&item, PriceModifier::Neutral), 100);
        assert_eq!(buy_price(&item, PriceModifier::Hostile), 150);
        assert_eq!(buy_price(&item, PriceModifier::Allied), 80);
    }

    #[test]
    fn test_sell_price() {
        let item = test_item("sword", 100);
        
        assert_eq!(sell_price(&item, PriceModifier::Neutral), 80);
        assert_eq!(sell_price(&item, PriceModifier::Hostile), 50);
        assert_eq!(sell_price(&item, PriceModifier::Allied), 100);
    }

    // -------------------------------------------------------------------------
    // Transaction tests
    // -------------------------------------------------------------------------

    #[test]
    fn test_execute_purchase_success() {
        let mut buyer = Inventory::new(10, 100.0);
        buyer.add_gold(200);
        
        let item = test_item("potion", 50);
        
        assert!(execute_purchase(&mut buyer, item, 2, PriceModifier::Neutral));
        assert_eq!(buyer.gold, 100); // 200 - (50 * 2)
        assert!(buyer.has_item("potion", 2));
    }

    #[test]
    fn test_execute_purchase_insufficient_gold() {
        let mut buyer = Inventory::new(10, 100.0);
        buyer.add_gold(50);
        
        let item = test_item("expensive", 100);
        
        assert!(!execute_purchase(&mut buyer, item, 1, PriceModifier::Neutral));
        assert_eq!(buyer.gold, 50); // Unchanged
    }

    #[test]
    fn test_execute_sale_success() {
        let mut seller = Inventory::new(10, 100.0);
        let item = test_item("gem", 100);
        seller.add_item(item, 3);
        
        let result = execute_sale(&mut seller, "gem", 2, PriceModifier::Neutral);
        
        assert!(result.is_some());
        assert_eq!(result.unwrap(), 160); // 2 * 80 (neutral sell = 80%)
        assert!(seller.has_item("gem", 1)); // 1 remaining
    }

    #[test]
    fn test_execute_sale_insufficient_quantity() {
        let mut seller = Inventory::new(10, 100.0);
        let item = test_item("gem", 100);
        seller.add_item(item, 1);
        
        let result = execute_sale(&mut seller, "gem", 5, PriceModifier::Neutral);
        
        assert!(result.is_none());
    }

    #[test]
    fn test_execute_sale_item_not_found() {
        let mut seller = Inventory::new(10, 100.0);
        
        let result = execute_sale(&mut seller, "nonexistent", 1, PriceModifier::Neutral);
        
        assert!(result.is_none());
    }
}
