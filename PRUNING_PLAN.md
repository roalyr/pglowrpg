# Codebase Pruning Plan

**Date:** 2024-12-13
**Status:** ✅ COMPLETED
**Purpose:** Simplify the codebase to a barebones foundation for the reimagined pGLOWrpg (per GDD-PGLOWRPG-REIMAGINED.md)

---

## Summary of Changes

### Removed
- `apps/dev_test_corner/` - Entire directory deleted
- `apps/game/src/commands_ops/` - Entire directory deleted
- `apps/game/src/commands_ops.rs` - Deleted
- `apps/game/src/formatting_ops.rs` - Deleted
- `apps/game/src/printing_ops.rs` - Deleted
- `locales/en/commands_strings.ron` - Deleted (no longer used)

### Simplified
- `apps/game/src/lib.rs` - Reduced from ~65 lines to ~35 lines (stub)
- `apps/game/src/data_ops.rs` - Reduced from ~95 lines to ~55 lines
- `apps/game/src/struct_ops.rs` - Reduced from ~110 lines to ~25 lines
- `apps/game/Cargo.toml` - Removed 5 unnecessary dependencies
- `src/menu.rs` - Simplified menu from 5 options to 3
- `Cargo.toml` - Removed dev_test_corner dependency

### Added Headers (per AI_PRIMING.md)
- `src/main.rs`
- `src/menu.rs`
- `apps/game/src/lib.rs`
- `apps/game/src/data_ops.rs`
- `apps/game/src/struct_ops.rs`

### Fixed
- `apps/worldgen/src/layer_ops/flora_mapping.rs` - Fixed unused variable warnings

---

## Final State

### Code Metrics (Approximate)
| Component | Before | After | Change |
|-----------|--------|-------|--------|
| `apps/game/` total | ~600 lines | ~115 lines | -81% |
| Menu options | 5 | 3 | -40% |
| Game dependencies | 9 | 5 | -44% |

### What Remains
- **Worldgen:** Fully intact and functional
- **Game stub:** Loads worlds, displays basic info
- **All libs:** Unchanged (needed by worldgen)
- **Data files:** Locales, palettes, presets unchanged

### Build Status
```
cargo build
Finished `dev` profile [optimized] target(s)
```
No errors, no warnings.

---

## Next Steps

The codebase is now ready for GDD implementation. Per GDD-PGLOWRPG-REIMAGINED.md Phase 1:

1. **Character System** - Stats, resources, vows
2. **Move Engine** - Action roll resolution (2d10 vs 2d10)
3. **Oracle System** - Table lookups, random generation
4. **Location Abstraction** - Significant points on world map
5. **Basic CLI Interface** - Journey view, action prompts

See [GDD-PGLOWRPG-REIMAGINED.md](GDD-PGLOWRPG-REIMAGINED.md) Section 7 (Development Phases) and Section 8 (Technical Architecture) for details.
