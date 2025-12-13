# Development Plan for pGLOWrpg Reimagined

**Date:** 2025-12-13  
**Version:** 0.1.0  
**Reference:** [GDD-PGLOWRPG-REIMAGINED.md](GDD-PGLOWRPG-REIMAGINED.md)

---

## Current State Summary

### Codebase Health ✅
- **Build Status:** Clean (0 warnings, 0 errors)
- **Rust Version:** 1.92.0, Edition 2024
- **All source files:** Standardized headers applied
- **TODOs/FIXMEs:** 0 remaining in Rust code
- **Unused code:** Pruned (gaussian_blur, hexagonify, dev_test_corner)

### Components
| Component | Status | Lines | Notes |
|-----------|--------|-------|-------|
| **Worldgen** | ✅ Functional | ~4,500 | Terrain, climate, biomes, rivers, flora |
| **Game** | 🟡 Stub | ~115 | Loads world data, displays info |
| **Libraries** | ✅ Functional | ~3,000 | IO, constants, codecs, image ops |

### File Structure
```
pglowrpg/
├── src/           # Entry point + menu
├── apps/
│   ├── worldgen/  # World generation (fully functional)
│   └── game/      # Game stub (to be implemented)
├── libs/          # 10 internal libraries
├── dep/           # External dependency re-exports
├── locales/       # Localized strings
├── palettes/      # Color definitions
├── presets_default/  # World & entity presets
└── options/       # Game options
```

---

## Implementation Roadmap

### Phase 1: Core Systems Foundation
**Target:** Basic gameplay loop without worldgen integration

#### 1.1 Character System ✅
- [x] Create `libs/lib_character/` module
- [x] Implement stat blocks (Edge, Heart, Iron, Shadow, Wits)
- [x] Resource tracking (Health, Spirit, Supply, Momentum)
- [x] Vow/Bond data structures
- [ ] Character serialization (save/load)

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
- [ ] Simple journal/log system

### Phase 2: World Integration
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
- [ ] Location metadata (name, type, features)

#### 2.2 Journey System ✅
- [x] Create `libs/lib_travel/` module
- [x] Travel between locations
- [x] Terrain-based difficulty modifiers
- [x] Supply consumption on travel
- [ ] Random encounter triggers (partially via Oracle)

#### 2.3 Map Interface
- [ ] ASCII region map display
- [ ] Current location marker
- [ ] Known locations overlay
- [ ] Path visualization

### Phase 3: Content & Polish
**Target:** Full gameplay experience

#### 3.1 NPC System
- [ ] Faction definitions
- [ ] NPC generation from oracle
- [ ] Relationship tracking
- [ ] Dialogue stubs

#### 3.2 Combat System
- [ ] Enter the Fray / Strike / Clash moves
- [ ] Enemy stat blocks
- [ ] Combat state machine

#### 3.3 Economy
- [ ] Item definitions
- [ ] Inventory management
- [ ] Trade mechanics

#### 3.4 Soul System (GDD Section 12)
- [ ] Reincarnation triggers
- [ ] Memory persistence
- [ ] Legacy effects

---

## Technical Debt & Improvements

### Low Priority (Future)
- [ ] Consider macro for BIOMES_IDS/BIOMES_CODENAMES dual hashmaps
- [ ] Optimize flora capacity estimation in worldgen
- [ ] Add progress bars to long worldgen operations
- [ ] Add unit tests for core libraries

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

### Immediate (This Session)
1. Create `libs/lib_character/` with basic stat structure
2. Create `libs/lib_moves/` with action roll implementation
3. Update `apps/game/` to use new systems

### Short Term (1-2 weeks)
1. Complete Phase 1 core systems
2. Playable character creation flow
3. Basic move resolution working

### Medium Term (1-2 months)
1. Complete Phase 2 world integration
2. Journey between generated world locations
3. Meaningful encounters from oracle tables

---

## Reference Links

- [GDD-PGLOWRPG-REIMAGINED.md](GDD-PGLOWRPG-REIMAGINED.md) - Full design document
- [PRUNING_PLAN.md](PRUNING_PLAN.md) - Recent cleanup documentation
- [AI_PRIMING.md](AI_PRIMING.md) - AI assistant context

---

*This document should be updated as development progresses.*
