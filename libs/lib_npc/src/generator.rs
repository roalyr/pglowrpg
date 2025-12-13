// [Revision: v0.1] [Path: libs/lib_npc/src/generator.rs] [Date: 2025-12-13]
// Description: NPC generation using oracle tables

use crate::npc::{Npc, NpcDisposition, NpcRole};
use lib_oracle::Oracle;
use lib_pseudo_rng::Rng;

/// Names for NPC generation.
const FIRST_NAMES: &[&str] = &[
    "Aric", "Bran", "Cade", "Dara", "Elara", "Finn", "Gwen", "Holt",
    "Ivy", "Jace", "Kira", "Lark", "Mira", "Nash", "Ora", "Penn",
    "Quinn", "Rook", "Seren", "Thane", "Una", "Vale", "Wren", "Xara",
    "Yara", "Zane", "Ash", "Bryn", "Cora", "Drake", "Ember", "Flint",
    "Gale", "Haven", "Iris", "Jade", "Kael", "Luna", "Moss", "Nyx",
];

const EPITHETS: &[&str] = &[
    "the Bold", "the Quiet", "the Wise", "the Swift", "the Strong",
    "One-Eye", "Ironhand", "Shadowfoot", "Fireheart", "Stormborn",
    "the Wanderer", "the Healer", "the Hunter", "the Smith", "the Elder",
    "Blackwood", "Riverborn", "Stoneheart", "Wildmane", "Duskwalker",
];

/// Generator for creating NPCs using oracle tables.
pub struct NpcGenerator {
    oracle: Oracle,
    next_id: u32,
}

impl NpcGenerator {
    /// Create a new NPC generator.
    pub fn new() -> Self {
        Self {
            oracle: Oracle::new(),
            next_id: 1,
        }
    }
    
    /// Generate a random NPC.
    pub fn generate(&mut self, rng: &mut Rng) -> Npc {
        let id = self.next_id;
        self.next_id += 1;
        
        // Generate name
        let name = self.generate_name(rng);
        
        // Generate role
        let role = self.generate_role(rng);
        
        // Get disposition from oracle
        let disposition_str = self.oracle.npc_disposition(rng);
        let disposition = NpcDisposition::from_str(disposition_str);
        
        // Get goal from oracle
        let goal = self.oracle.npc_goal(rng).to_string();
        
        Npc::new(id, name, role)
            .with_disposition(disposition)
            .with_goal(goal)
    }
    
    /// Generate a random name.
    pub fn generate_name(&self, rng: &mut Rng) -> String {
        let first_idx = rng.rand_range(0, FIRST_NAMES.len() as u32) as usize;
        let first = FIRST_NAMES[first_idx];
        
        // 30% chance of epithet
        if rng.rand_range(0, 100) < 30 {
            let epithet_idx = rng.rand_range(0, EPITHETS.len() as u32) as usize;
            format!("{} {}", first, EPITHETS[epithet_idx])
        } else {
            first.to_string()
        }
    }
    
    /// Generate a random role.
    pub fn generate_role(&self, rng: &mut Rng) -> NpcRole {
        let roll = rng.rand_range(1, 101);
        match roll {
            1..=25 => NpcRole::Commoner,
            26..=35 => NpcRole::Merchant,
            36..=50 => NpcRole::Warrior,
            51..=60 => NpcRole::Mystic,
            61..=70 => NpcRole::Outlaw,
            71..=75 => NpcRole::Noble,
            76..=85 => NpcRole::Traveler,
            86..=90 => NpcRole::Artisan,
            91..=95 => NpcRole::Scholar,
            _ => NpcRole::Hunter,
        }
    }
    
    /// Generate NPC for a specific context.
    pub fn generate_for_location(&mut self, rng: &mut Rng, location_id: u32) -> Npc {
        let mut npc = self.generate(rng);
        npc.location_id = Some(location_id);
        npc
    }
    
    /// Generate hostile NPC (for combat encounters).
    pub fn generate_hostile(&mut self, rng: &mut Rng) -> Npc {
        let mut npc = self.generate(rng);
        npc.disposition = NpcDisposition::Hostile;
        // Hostile NPCs are more likely to be outlaws/warriors
        npc.role = if rng.rand_range(0, 100) < 60 {
            NpcRole::Outlaw
        } else {
            NpcRole::Warrior
        };
        npc
    }
}

impl Default for NpcGenerator {
    fn default() -> Self {
        Self::new()
    }
}

/// Collection of NPCs in the world.
#[derive(Debug, Clone, Default)]
pub struct NpcRegistry {
    npcs: Vec<Npc>,
}

impl NpcRegistry {
    /// Create a new NPC registry.
    pub fn new() -> Self {
        Self { npcs: Vec::new() }
    }
    
    /// Add an NPC to the registry.
    pub fn add(&mut self, npc: Npc) {
        self.npcs.push(npc);
    }
    
    /// Get NPC by ID.
    pub fn get(&self, id: u32) -> Option<&Npc> {
        self.npcs.iter().find(|n| n.id == id)
    }
    
    /// Get mutable NPC by ID.
    pub fn get_mut(&mut self, id: u32) -> Option<&mut Npc> {
        self.npcs.iter_mut().find(|n| n.id == id)
    }
    
    /// Get all NPCs at a location.
    pub fn at_location(&self, location_id: u32) -> Vec<&Npc> {
        self.npcs.iter()
            .filter(|n| n.location_id == Some(location_id) && n.alive)
            .collect()
    }
    
    /// Get all met NPCs.
    pub fn met(&self) -> Vec<&Npc> {
        self.npcs.iter().filter(|n| n.met && n.alive).collect()
    }
    
    /// Get all NPCs.
    pub fn all(&self) -> &[Npc] {
        &self.npcs
    }
}
