# Development Plan for pGLOWrpg Reimagined

**Date:** 2025-12-13  
**Version:** 0.7.0  
**Reference:** [GDD-PGLOWRPG-REIMAGINED.md](GDD-PGLOWRPG-REIMAGINED.md)

---

## Current State Summary

### Codebase Health ✅
- **Build Status:** Clean (minor warnings, 0 errors)
- **Test Status:** 138 unit tests passing
- **Rust Version:** 1.92.0, Edition 2024
- **All source files:** Standardized headers applied
- **TODOs/FIXMEs:** 0 remaining in Rust code

### Components
| Component | Status | Tests | Notes |
|-----------|--------|-------|-------|
| **Worldgen** | ✅ Functional | N/A | Terrain, climate, biomes, rivers, flora |
| **Game** | ✅ Playable | 17 | Save/load, journal, combat, vows |
| **lib_character** | ✅ Complete | 20 | Stats, resources, vows, inventory |
| **lib_moves** | ✅ Complete | 14 | Action rolls, momentum, progress |
| **lib_oracle** | ✅ Complete | 2 | Yes/No, tables, random events |
| **lib_combat** | ✅ Complete | 7 | Combat state, enemies, actions |
| **lib_soul** | ✅ Complete | 9 | Reincarnation, chronicle, marks |
| **lib_location** | ✅ Complete | 4 | Regions, locations, map display |
| **lib_travel** | ✅ Complete | 3 | Journey progress, supply |
| **lib_npc** | ✅ Complete | 4 | NPCs, factions, relationships |
| **lib_economy** | ✅ Complete | 8 | Items, inventory, trade |

### File Structure
```
pglowrpg/
├── src/           # Entry point + menu
├── apps/
│   ├── worldgen/  # World generation (fully functional)
│   └── game/      # Game app (playable loop)
├── libs/          # 15 internal libraries
├── dep/           # External dependency re-exports
├── locales/       # Localized strings
├── palettes/      # Color definitions
├── presets_default/  # World & entity presets
└── options/       # Game options
```

---

## Implementation Roadmap

### Phase 1: Core Systems Foundation ✅ COMPLETE
**Target:** Basic gameplay loop without worldgen integration

#### 1.1 Character System ✅
- [x] Create `libs/lib_character/` module
- [x] Implement stat blocks (Edge, Heart, Iron, Shadow, Wits)
- [x] Resource tracking (Health, Spirit, Supply, Momentum)
- [x] Vow/Bond data structures
- [x] Character serialization (save/load) ✅

#### 1.2 Move Engine ✅
- [x] Create `libs/lib_moves/` module
- [x] Implement 2d10 vs 2d10 action roll
- [x] Strong/Weak Hit/Miss resolution
- [x] Momentum system (burn, reset)
- [x] Progress track mechanics

#### 1.3 Oracle System ✅
- [x] Create `libs/lib_oracle/` module
- [x] D100 table lookup system
- [x] Weighted random selection (Yes/No with likelihood)
- [x] Action/Theme/Complication tables
- [ ] Load tables from RON files (future extension)

#### 1.4 Basic CLI Interface ✅
- [x] Refactor `apps/game/` with proper command loop
- [x] Status display (character, resources, vows)
- [x] Action selection menu
- [x] Move resolution output
- [x] Journal/log system ✅ (13 tests)
- [x] Save/load game ✅ (6 tests)

### Phase 2: World Integration ✅ COMPLETE
**Target:** Connect worldgen output to gameplay

#### 2.1 Location Abstraction ✅
- [x] Create `libs/lib_location/` module
- [x] Location types (Settlement, Ruin, Wilderness, Sacred, Landmark)
- [x] Location rank system (Troublesome to Epic)
- [x] Region management
  - River confluences/sources
  - Biome boundaries
  - Coastlines
  - Mountain passes
- [x] Location metadata (name, type, features)

#### 2.2 Journey System ✅
- [x] Create `libs/lib_travel/` module
- [x] Travel between locations
- [x] Terrain-based difficulty modifiers
- [x] Supply consumption on travel
- [x] Random encounter triggers (via Oracle + exploration)

#### 2.3 Map Interface ✅
- [x] ASCII region map display
- [x] Current location marker
- [x] Known locations overlay
- [ ] Path visualization (future)

### Phase 3: Content & Polish ✅ COMPLETE
**Target:** Full gameplay experience

#### 3.1 NPC System ✅
- [x] Faction definitions
- [x] NPC generation from oracle
- [x] Relationship tracking
- [ ] Dialogue stubs (future)

#### 3.2 Combat System ✅
- [x] Enter the Fray / Strike / Clash moves
- [x] Enemy stat blocks (lib_combat)
- [x] Combat state machine

#### 3.3 Economy ✅
- [x] Item definitions (lib_economy)
- [x] Inventory management
- [x] Trade mechanics

#### 3.4 Soul System (GDD Section 12) ✅
- [x] Reincarnation triggers
- [x] Memory persistence (Soul marks)
- [x] Legacy effects (Chronicle, Fate threads)

---

## Phase 4: Advanced Features (Future)
**Target:** Enhanced gameplay depth

### 4.1 Flora & Fauna Integration
- [ ] Connect worldgen flora data to game
- [ ] Resource gathering mechanics
- [ ] Crafting system

### 4.2 Settlement Development
- [ ] Settlement growth mechanics
- [ ] Building construction
- [ ] Population management

### 4.3 Procedural Narrative
- [ ] Dynamic quest generation
- [ ] Story arc system
- [ ] Consequence chains

---

## Technical Debt & Improvements

### Low Priority (Future)
- [ ] Consider macro for BIOMES_IDS/BIOMES_CODENAMES dual hashmaps
- [ ] Optimize flora capacity estimation in worldgen
- [ ] Add progress bars to long worldgen operations
- [x] Add unit tests for core libraries ✅ (138 tests)

### Code Quality
- [ ] Add `#[must_use]` attributes where appropriate
- [ ] Consider `thiserror` for error handling
- [ ] Document public APIs with rustdoc

### Performance
- [ ] Profile worldgen with large map sizes
- [ ] Consider rayon for parallel river pathfinding
- [ ] Lazy loading of world data sections

---

## Suggested Next Steps

### Immediate (This Session) ✅ COMPLETE
1. ~~Create `libs/lib_character/` with basic stat structure~~ ✅
2. ~~Create `libs/lib_moves/` with action roll implementation~~ ✅
3. ~~Update `apps/game/` to use new systems~~ ✅

### Short Term (Next Session)
1. Polish UI/UX - better formatting, clearer menus
2. Balance game mechanics - test vow progression
3. Add more oracle tables variety

### Medium Term (1-2 months)
1. Complete Phase 4 advanced features
2. Integrate generated world data into gameplay
3. Add more content (locations, NPCs, items)

---

## Reference Links

- [GDD-PGLOWRPG-REIMAGINED.md](GDD-PGLOWRPG-REIMAGINED.md) - Full design document
- [PRUNING_PLAN.md](PRUNING_PLAN.md) - Recent cleanup documentation
- [AI_PRIMING.md](AI_PRIMING.md) - AI assistant context

---

*This document should be updated as development progresses.*
