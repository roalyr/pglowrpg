# pGLOWrpg - Game Design Document (Reimagined)

**Version:** 0.1 (Draft)
**Date:** December 13, 2025
**Author:** Roal-Yr

---

# 0. Introduction

## 0.1 Game Identity

* **Game Title:** pGLOWrpg (Procedurally Generated Living Open World RPG)
* **Logline:** A solo narrative RPG where you guide a wanderer through a procedurally generated world, making meaningful choices that shape both your story and the land itself.
* **Genre:** Narrative Roguelike, Solo RPG, Text Adventure, World Simulation
* **Theme:** The lone traveler finding purpose in a vast, indifferent world. Stories emerge from the intersection of player choice and world state.
* **Target Audience:** Fans of Ironsworn, Dwarf Fortress adventure mode, classic roguelikes, and players who enjoy emergent narrative through procedural systems.
* **Platform:** 
    * Primary: CLI (Rust) - The "authentic" experience
    * Secondary: Analogue (Tabletop solo RPG using generated world maps)
* **Unique Selling Points:**
    * Procedurally generated worlds with geological, climatic, and ecological coherence
    * Ironsworn-inspired narrative mechanics adapted for digital play
    * A world that exists and evolves beyond the player
    * Classical fantasy made personal through procedural storytelling
    * Play the same world digitally or on paper

## 0.2 Design Philosophy

> **"The world was here before you arrived. It will be here after you're gone. What mark will you leave?"**

The game is not about saving the world. It is about finding your place in it.

### Core Principles

1. **The World is Real:** The procedural generation creates a world with internal logic—geography shapes climate, climate shapes biomes, biomes shape what lives there. This coherence makes exploration meaningful.

2. **You Are Small:** The player is one person in a vast world. You cannot conquer kingdoms or slay gods. You can help a village, explore ruins, make enemies, find allies, and leave a legacy.

3. **Stories Emerge:** We do not write plots. We create systems that generate situations. The narrative emerges from the player's choices within those situations.

4. **Simple Rules, Deep Play:** A unified resolution mechanic (inspired by Ironsworn) handles all uncertain outcomes. Complexity comes from context, not from rule exceptions.

5. **Analog Compatibility:** If a mechanic cannot be explained with dice and paper, it's too complicated. The CLI is an oracle, not a hidden black box.

6. **Death Ends a Life, Not a Story:** Characters die permanently—their skills, relationships, and progress are lost. But souls persist. Through reincarnation, echoes of past lives create new story seeds, emotional continuity, and a soft landing that distinguishes this from punishing roguelikes while maintaining meaningful stakes.

---

# 1. Core Mechanics

## 1.1 The Action Roll

All uncertain actions use a single resolution mechanic.

### The Roll
```
Roll 2d10 + Modifier vs. Challenge Dice (1d10 + 1d10)
```

* **Action Dice:** 2d10, summed, plus your relevant stat modifier (+1 to +3)
* **Challenge Dice:** 2d10 rolled separately (not summed)

### Outcomes

| Result | Name | Meaning |
|--------|------|---------|
| Action > Both Challenge | **Strong Hit** | You succeed fully. Gain momentum or bonus. |
| Action > One Challenge | **Weak Hit** | You succeed, but at a cost or with a complication. |
| Action ≤ Both Challenge | **Miss** | You fail. Face a consequence. The world pushes back. |

### Special Results

* **Match (Challenge dice show same number):** The outcome is pointed and dramatic. On a hit, an unexpected opportunity. On a miss, a serious escalation.

*This is the Ironsworn engine, proven for solo narrative play.*

## 1.2 Character Stats

Five stats define your capabilities. Each ranges from +1 to +3.

| Stat | Governs |
|------|---------|
| **Edge** | Speed, agility, precision, ranged combat |
| **Heart** | Courage, willpower, social connection, empathy |
| **Iron** | Strength, endurance, melee combat, physical labor |
| **Shadow** | Stealth, deception, cunning, subterfuge |
| **Wits** | Knowledge, perception, crafting, survival |

### Starting Array
New characters distribute the array: +3, +2, +2, +1, +1

## 1.3 Core Resources

### Momentum (0 to +10, resets to +2)
* Gained on strong hits and through certain moves
* Can be "burned" to improve a weak hit to a strong hit (if momentum > highest challenge die)
* Resets to +2 after burning

### Health (0 to 5)
* Physical condition
* At 0: You are wounded and must **Endure Harm** or face death

### Spirit (0 to 5)
* Mental/emotional condition
* At 0: You are broken and must **Endure Stress** or face despair

### Supply (0 to 5)
* Abstract representation of food, gear, ammunition, trade goods
* Depletes through travel and hardship
* At 0: You are desperate and must **Face Deprivation**

### Experience (XP)
* Gained by completing Vows and reaching Milestones
* Spent to improve stats or gain Assets

## 1.4 Vows (Goals)

Vows are the engine of play. They give direction and meaning.

### Vow Ranks

| Rank | Difficulty | Progress per Milestone | XP on Completion |
|------|------------|------------------------|------------------|
| Troublesome | Easy, local scope | 3 boxes (of 10) | 1 XP |
| Dangerous | Significant effort | 2 boxes | 2 XP |
| Formidable | Major undertaking | 1 box | 3 XP |
| Extreme | Life-defining | 0.5 boxes | 5 XP |
| Epic | Legendary | 0.25 boxes | 8 XP |

### Swearing a Vow
When you commit to a goal, you **Swear an Iron Vow**:
* State the vow clearly
* Roll +Heart
* On a hit: The path is clear, gain momentum
* On a weak hit: The way is uncertain but possible
* On a miss: Dark portents or immediate obstacles

### Fulfilling a Vow
When you believe you've achieved your goal, **Fulfill Your Vow**:
* Roll the progress track (count filled boxes as your roll)
* Compare against challenge dice
* Strong hit: Complete success, gain XP, narrative closure
* Weak hit: Success with a loose end or cost
* Miss: The vow is unfulfilled in a meaningful way

## 1.5 Assets

Assets are special abilities, companions, or items that provide mechanical benefits.

### Asset Types

* **Companion:** A loyal ally (animal or person) who aids you
* **Path:** A background or profession granting specific moves
* **Combat Talent:** Fighting styles or weapon expertise
* **Ritual:** Supernatural abilities (if magic exists in your world)

Each asset has up to 3 upgrade tiers. Gaining an asset costs 3 XP. Upgrading costs 2 XP.

---

# 2. The World

## 2.1 World Generation (Existing System)

The existing pGLOWrpg worldgen remains the foundation. It produces:

* **Terrain:** Elevation maps with mountains, plains, valleys, coastlines
* **Climate:** Temperature and rainfall derived from terrain and latitude
* **Biomes:** ~50 biome types based on climate/elevation intersection
* **Rivers:** Pathfound drainage systems with erosion
* **Regions:** Bioregions (ecological) and Georegions (geological)
* **Flora:** Plant distribution based on biome suitability

### New: Settlement & Feature Generation (Phase 2+)

Layered on top of the natural world:
* **Settlements:** Villages, towns, ruins placed based on habitability
* **Routes:** Paths and roads connecting settlements
* **Points of Interest:** Ruins, shrines, dangerous lairs, resource sites
* **Factions:** Groups with territory, goals, and relationships

## 2.2 World Scale

* **Map Cell:** ~300m × 300m (a "location" you can explore)
* **Region:** ~50-200 cells forming a coherent area
* **World:** 256×256 to 4096×4096 cells (scale chosen at generation)

### Travel Abstraction

The world is too large to traverse cell-by-cell in real time. Travel is abstracted:

* **Journey:** Moving from one significant location to another
* **Waypoint:** A cell where something notable exists or happens
* **Passage:** The cells between waypoints (handled narratively)

## 2.3 Locations

### Location Types

| Type | Description | Gameplay |
|------|-------------|----------|
| **Settlement** | Village, town, or city | Social moves, resupply, vows, rumors |
| **Waystation** | Inn, trading post, shrine | Rest, limited resupply, encounters |
| **Wilderness** | Untamed biome area | Survival, exploration, hunting |
| **Ruin** | Abandoned structure | Delving, danger, treasure |
| **Landmark** | Notable natural feature | Navigation, discovery, sometimes sacred |
| **Lair** | Creature den or enemy camp | Combat, stealth, potential resources |

### Location Generation

Each significant location is generated with:
* **Biome context** (from worldgen)
* **Descriptor tags** (e.g., "forested," "riverside," "abandoned")
* **Threat level** (safe / unsafe / dangerous / deadly)
* **Features** (what can be interacted with)
* **NPCs** (if inhabited)

---

# 3. Gameplay Modules

## 3.1 Journey Module

Travel is not free. The world is vast and indifferent.

### Undertake a Journey

When you travel through dangerous or unfamiliar lands:

1. Set your destination and determine journey length (in waypoints)
2. Roll +Wits (or +appropriate stat for terrain)
3. **Strong Hit:** Reach a waypoint, mark progress, journey continues smoothly
4. **Weak Hit:** Reach a waypoint but face a cost (suffer -1 supply, hazard, or delay)
5. **Miss:** A crisis occurs—loss, danger, or you are waylaid

### Journey Progress Track

Like vows, journeys have a progress track (10 boxes). Each successful waypoint fills boxes based on journey difficulty.

### Reach Your Destination

When the progress track is sufficiently filled, you may attempt to **Reach Your Destination**:
* Roll progress vs. challenge dice
* Strong hit: Arrive safely with an advantage
* Weak hit: Arrive but fatigued or with a complication
* Miss: You are lost, delayed, or arrive at the wrong moment

### Journey Encounters

Misses and weak hits trigger encounters from the **Journey Oracle**:
* Environmental hazards (weather, terrain)
* Creature encounters (hostile, neutral, or potential ally)
* Human encounters (travelers, bandits, patrols)
* Discoveries (ruins, resources, mysteries)

## 3.2 Combat Module

Combat is dangerous. Even victory has costs.

### Combat Flow

1. **Enter the Fray:** Roll to establish your initial position
2. **Exchange Blows:** Take turns making moves until resolved
3. **End the Fight:** When your foe's progress track is full, attempt to end it

### Combat Moves

| Move | Trigger | Roll |
|------|---------|------|
| **Strike** | Attack in melee | +Iron |
| **Clash** | Fight defensively while attacking | +Iron (less damage but safer) |
| **Shoot** | Ranged attack | +Edge |
| **Face Danger** | React to immediate threat | +Appropriate stat |
| **Secure an Advantage** | Gain upper hand | +Appropriate stat |
| **Turn the Tide** | Desperate reversal (burn momentum) | Special |

### Harm & Progress

* **Dealing Harm:** Hits fill the enemy's progress track (amount based on weapon)
* **Taking Harm:** Misses or enemy actions reduce your Health
* **Armor:** Absorbs harm but may be damaged

### End the Fight

When enemy progress is sufficient:
* Roll progress vs. challenge dice
* Strong hit: Decisive victory, you dictate terms
* Weak hit: Victory at a cost (harm, resource loss, complication)
* Miss: The fight turns against you or enemy escapes

### Lethality

Combat should feel weighty. Death is possible. The **Pay the Price** move applies when things go wrong—and sometimes the price is a character's life.

## 3.3 Social Module

Relationships matter. You cannot survive alone.

### Connections

NPCs you've built a relationship with become **Connections**. Each has:
* **Name and role**
* **Relationship track** (0 to 40, representing bond strength)
* **Disposition** (friendly / neutral / wary / hostile)

### Social Moves

| Move | Trigger | Roll |
|------|---------|------|
| **Compel** | Persuade, threaten, or seduce | +Heart or +Shadow |
| **Gather Information** | Ask questions, investigate | +Wits |
| **Aid Your Ally** | Help a connection with their goal | +Heart |
| **Forge a Bond** | Solidify a meaningful relationship | +Heart |
| **Test Your Relationship** | When the bond is strained | Roll relationship progress |

### Building Relationships

* Help NPCs with their problems
* Share hardship on journeys
* Keep promises
* Honor local customs

### Faction Standing

Beyond individual NPCs, you have standing with **Factions**:
* +3 (Honored): They actively help you
* +2 (Friendly): Favorable treatment
* +1 (Liked): Benefit of the doubt
* 0 (Neutral): No special treatment
* -1 (Disliked): Suspicion and higher prices
* -2 (Hated): Hostility and exclusion
* -3 (Hunted): Active enemies

## 3.4 Exploration Module

Discovering what the world holds.

### Explore a Waypoint

When you search a location for secrets or resources:
* Roll +Wits
* Strong hit: Discover something valuable, mark progress on exploration
* Weak hit: Discovery, but it attracts attention or costs time
* Miss: Danger finds you first

### Delve the Depths

When exploring a ruin, cave, or dangerous interior:
1. Determine the site's **Threat** (like vow rank)
2. Make progress by overcoming obstacles and dangers
3. **Locate Your Objective** when progress is sufficient

This is essentially a dungeon-crawl subsystem with progress-based resolution.

### Discovery Types

| Type | Examples |
|------|----------|
| **Resource** | Ore deposit, herb patch, fresh water |
| **Treasure** | Coins, artifacts, trade goods |
| **Knowledge** | Map fragment, historical record, rumor |
| **Shelter** | Defensible position, hidden camp, safe haven |
| **Mystery** | Strange phenomenon, unexplained ruin, riddle |

## 3.5 Survival Module

The world does not care if you live.

### Make Camp

When you rest in the wild:
* Roll +Supply
* Strong hit: Recover health or spirit, resupply holds
* Weak hit: Recover, but supply decreases or danger approaches
* Miss: Trouble finds you—no rest, or worse

### Resupply

When you forage, hunt, or scavenge:
* Roll +Wits
* Strong hit: +2 Supply
* Weak hit: +1 Supply, but at a cost (time, noise, risk)
* Miss: You find nothing, and something finds you

### Face Deprivation

When Supply hits 0:
* Roll +Heart
* Strong hit: You persevere, but must resupply soon
* Weak hit: Suffer -1 Health or Spirit (your choice)
* Miss: Suffer both, and face further hardship

### Environmental Hazards

Biomes impose conditions:
* **Extreme Cold:** Suffer harm without shelter/fire
* **Extreme Heat:** Increased supply consumption
* **Toxic Swamp:** Ongoing spirit or health drain
* **Thin Air (Mountains):** Reduced effectiveness

---

# 4. World Simulation

## 4.1 The Time Clock

Time passes. The world moves.

### Time Units (TU)

* Actions, travel, and rest consume **Time Units**
* Major actions: 1 TU
* Short journeys: 1-3 TU
* Long journeys: 5-10+ TU
* Rest/recovery: 1-2 TU

### World Event Tick

When accumulated TU reaches a threshold (e.g., 20 TU):
1. **Season advances** (if applicable)
2. **Faction actions resolve** (Phase 3+)
3. **World events trigger** (from event tables)
4. **NPC goals progress** (connections may achieve or fail their objectives)
5. **Time clock resets**

This creates a world that exists beyond the player's immediate actions.

## 4.2 The Chronicle

A log of significant events, both player actions and world events.

### Chronicle Entry Types

* **Vows:** Sworn, progressed, fulfilled, forsaken
* **Journeys:** Destinations reached, discoveries made
* **Bonds:** Connections forged, tested, broken
* **Conflicts:** Battles fought, enemies made, truces reached
* **World Events:** Seasonal changes, faction movements, disasters

### Chronicle Uses

* Provides context for play (what happened before)
* Seeds future events (consequences catch up)
* Creates legacy (your story, recorded)

## 4.3 World Events (Phase 2+)

The world generates events independent of player action.

### Event Categories

| Category | Examples |
|----------|----------|
| **Natural** | Harsh winter, drought, plague, wildfire |
| **Faction** | War declared, trade route opened, leader dies |
| **Local** | Monster sighting, bandit activity, festival |
| **Mysterious** | Strange lights, wandering stranger, ancient artifact unearthed |

### Event Resolution

Events are generated and placed on the world map. They:
* Affect local conditions (travel difficulty, prices, danger level)
* Create opportunities (quests, trade, exploration)
* Evolve over time (unaddressed threats grow)

---

# 5. The Classical Fantasy Setting

## 5.1 Setting Philosophy

> **Familiar tropes, personal scale.**

The setting uses classical fantasy elements because they require no explanation:
* Everyone knows what a tavern is
* Everyone knows what goblins are
* Everyone knows the general shape of "medieval-ish society"

This shared vocabulary lets us skip exposition and focus on emergent personal stories.

### What This Setting Is

* **Low Fantasy:** Magic exists but is rare, dangerous, or costly
* **Grounded:** People need food, shelter, warmth. Economies exist.
* **Local:** The player's scope is a region, not a continent
* **Morally Gray:** No cosmic good vs. evil. People have reasons.

### What This Setting Is NOT

* **Epic:** You are not the chosen one
* **High Magic:** No wizard schools or casual teleportation
* **World-Ending Stakes:** The world will be fine. Your village might not.

## 5.2 Standard Elements

### Peoples

| People | Trope | Gameplay Niche |
|--------|-------|----------------|
| **Humans** | Default | Versatile, everywhere |
| **Elves** | Long-lived, aloof | Ancient knowledge, forest domains |
| **Dwarves** | Craftsmen, mountain-dwellers | Mining, smithing, underground |
| **Halflings** | Small, agrarian | Farming communities, overlooked |
| **Orcs** | Misunderstood outsiders | Frontier tribes, warrior culture |

*Note: These are starting defaults. Worldgen can reassign or remix.*

### Creatures

Drawn from classical bestiaries, placed by biome logic:
* **Forests:** Wolves, bears, giant spiders, forest spirits
* **Mountains:** Trolls, griffons, wyverns, giants
* **Swamps:** Lizardfolk, will-o-wisps, hydras
* **Plains:** Wild horses, centaurs, roving beast packs
* **Underground:** Goblins, kobolds, cave creatures, the nameless things

### Factions (Default)

| Faction | Type | Typical Goals |
|---------|------|---------------|
| **Local Lord** | Political | Maintain order, collect taxes, expand influence |
| **Merchant Guild** | Economic | Secure trade routes, maximize profit |
| **Temple** | Religious | Spread faith, protect sacred sites |
| **Bandit Clan** | Criminal | Survive, steal, avoid authority |
| **Ranger Order** | Protective | Guard the wilds, hunt monsters |
| **Mage Circle** | Arcane | Gather knowledge, stay hidden |

## 5.3 Magic (Optional Module)

Magic is **rare and costly**. It is not a reliable tool.

### Magic Philosophy

* Requires an **Asset** (Ritual path) to access
* Always has a **cost** (Spirit, Supply, or narrative price)
* Often **unpredictable** (matches on the action roll = wild magic)
* Never **trivializes** challenges

### Example Ritual Assets

* **Healer's Touch:** Restore health, but suffer spirit cost
* **Spirit Sight:** Perceive the invisible, risk attracting attention
* **Binding Word:** Compel truth, but the target remembers
* **Fire Calling:** Wield flame, but it hungers for more

---

# 6. User Interface (CLI)

## 6.1 Design Philosophy

The CLI is an **oracle**, not a video game. It:
* Presents information clearly
* Resolves mechanics faithfully
* Generates content from tables
* Tracks state reliably

The **player** provides the narrative interpretation.

## 6.2 Core Screens

### Main Menu
```
═══════════════════════════════════════════
         pGLOWrpg - Main Menu
═══════════════════════════════════════════

  [1] New Wanderer
  [2] Continue Journey
  [3] Generate World
  [4] World Viewer
  [5] Quit

> _
```

### Journey View (Primary Play Screen)
```
═══════════════════════════════════════════
  Day 47 | Spring | Clear Weather
───────────────────────────────────────────
  LOCATION: Ashford Village (Settlement)
  BIOME: Temperate Woodland | THREAT: Safe
  REGION: The Greenmark (Controlled by: Baron Aldric)
───────────────────────────────────────────
  STATS: Edge +2 | Heart +3 | Iron +1 | Shadow +2 | Wits +2
  HEALTH: ●●●●○  SPIRIT: ●●●○○  SUPPLY: ●●●●●
  MOMENTUM: +4
───────────────────────────────────────────
  ACTIVE VOWS:
  [■■■■■□□□□□] Find the missing merchant (Dangerous)
  [■■□□□□□□□□] Reach Thornhaven (Troublesome)
───────────────────────────────────────────
  ACTIONS:
  [1] Speak with locals       [5] Make Camp
  [2] Resupply                [6] Check Map  
  [3] Undertake Journey       [7] View Character
  [4] Explore Area            [8] Save & Quit

> _
```

### Action Resolution
```
═══════════════════════════════════════════
  ACTION: Compel — Convince the innkeeper to share rumors
  STAT: Heart (+3)
───────────────────────────────────────────
  Rolling...
  
  ACTION DICE: [7] + [4] + 3 = 14
  CHALLENGE:   [6]   [8]
  
  ★ STRONG HIT ★
  
  You succeed fully. The innkeeper leans in...
───────────────────────────────────────────
  ORACLE PROMPT: What does she know?
  [1] Roll on Rumor Table
  [2] Custom input
  
> _
```

### Map View
```
═══════════════════════════════════════════
  THE GREENMARK — Regional Map
═══════════════════════════════════════════
  
    ▲▲▲ · · · · ░░░     ▲ Mountain
    ▲▲ · · ◊ · ░░░░     · Woodland  
    · · · · · ·@░░░     ░ Swamp
    · · ■ · · · · ░     ◊ Ruin
    ≈≈ · · · · · ·      ■ Settlement
    ≈≈≈ · · □ · ·       □ Waystation
                        ≈ River
  @ = You (Ashford Village)
  
  Known Locations:
  [1] Ashford Village - Settlement (HERE)
  [2] Old Mill Ruins - Ruin (2 days NE, Dangerous)
  [3] Thornhaven - Settlement (5 days S, Safe)
───────────────────────────────────────────
  [J] Set Journey Destination  [Q] Return

> _
```

## 6.3 Oracle System

When narrative input is needed, the game provides **oracle prompts**:

### Yes/No Oracle
```
  ORACLE: Does the guard recognize you?
  Rolling... [67] — YES
```

### Table Oracles

Pre-built tables for common needs:
* **Action + Theme** (What is happening?)
* **NPC Descriptor** (Who is this person?)
* **Location Feature** (What's notable here?)
* **Creature Encounter** (What do you face?)
* **Complication** (What goes wrong?)
* **Opportunity** (What advantage appears?)

### Custom Tables

The game loads tables from external files, allowing:
* Setting customization
* User-created content
* Biome-specific encounters

---

# 7. Development Phases

## Phase 1: Core Loop ("The First Journey")

**Goal:** A complete, playable vertical slice demonstrating the core experience.

### Scope

* **One generated world** (use existing worldgen)
* **One starting character** with full stat system
* **Journey Module:** Travel between 3-5 pre-placed locations
* **Combat Module:** Basic combat against 2-3 enemy types
* **Social Module:** Stub implementation (Compel, Gather Information)
* **Survival Module:** Supply, Make Camp, Resupply
* **One Vow:** A simple "reach destination" or "find object" vow
* **Oracle System:** Basic yes/no and 2-3 tables

### Deliverable

A player can:
1. Create a character
2. View the world map
3. Set a vow
4. Travel between locations (with encounters)
5. Fight enemies
6. Talk to basic NPCs
7. Track and fulfill the vow
8. Experience the core action roll loop

### Milestone Checklist

- [ ] Character creation (stat assignment)
- [ ] World loading (from existing worldgen output)
- [ ] Location data structure and display
- [ ] Journey move implementation
- [ ] Basic encounter generation
- [ ] Combat move implementation
- [ ] Action roll engine (2d10 vs 2d10 with outcomes)
- [ ] Vow tracking (progress, fulfill)
- [ ] Simple CLI interface
- [ ] Save/Load character state

## Phase 2: Living World ("The Chronicle Begins")

**Goal:** The world exists beyond the player. Time passes. Events occur.

### Scope

* **Time System:** TU tracking, world event ticks
* **Season System:** Weather and condition changes
* **Chronicle:** Event logging
* **Expanded Locations:** Settlement generation with NPCs
* **Connections:** Relationship tracking with named NPCs
* **Faction Stubs:** Basic standing system
* **Exploration Module:** Waypoint exploration, discoveries
* **More Vow Types:** Dangerous, Formidable ranks

### Deliverable

The world feels alive:
* Seasons change
* NPCs have names and remember you
* Events happen in the background
* Your actions have recorded consequences

## Phase 3: Depth ("Roots in the World")

**Goal:** Long-term play becomes meaningful.

### Scope

* **Full Faction System:** Goals, territory, conflicts
* **Delve System:** Interior exploration (ruins, dungeons)
* **Asset System:** Companions, paths, talents
* **Magic Module:** Optional ritual system
* **Legacy System:** Character death and continuation
* **World Events:** Faction wars, disasters, opportunities

### Deliverable

A campaign that can run for dozens of hours with emergent long-term narratives.

## Phase 4: Polish & Expansion

* Multiple world presets
* Modding support (custom tables, factions, creatures)
* Analogue export (generate printable play materials)
* Alternative settings (not just fantasy)

---

# 8. Technical Architecture (Rust CLI)

## 8.1 Leveraging Existing Code

The current pGLOWrpg codebase provides:

| Existing | Use in Reimagined |
|----------|-------------------|
| Worldgen (terrain, climate, biomes, rivers) | Foundation layer — keep as-is |
| LayerPack data structure | World state storage |
| Biome/region mapping | Location context |
| Flora distribution | Discovery/resource content |
| CLI infrastructure | UI foundation |

### What Needs to Be Built

* **Character system** (stats, resources, vows)
* **Move engine** (action roll resolution)
* **Oracle system** (table lookups, random generation)
* **Location abstraction** (significant points on the world map)
* **Encounter system** (creature/NPC generation)
* **Save/Load** (character and world state)
* **Chronicle** (event log)

## 8.2 Proposed Module Structure

```
pglowrpg/
├── apps/
│   ├── worldgen/        # Existing - keep
│   └── game/            # Reimagined gameplay
│       ├── character/   # Stats, resources, vows
│       ├── moves/       # Action roll, all game moves
│       ├── oracle/      # Tables, random generation
│       ├── world/       # Location, travel, time
│       ├── combat/      # Combat subsystem
│       ├── social/      # NPC, connections, factions
│       └── ui/          # CLI rendering
├── libs/
│   ├── lib_dice/        # New: 2d10 engine, probability
│   ├── lib_tables/      # New: Oracle table loading
│   └── ... (existing)
└── data/
    ├── tables/          # Oracle tables (RON files)
    ├── creatures/       # Creature definitions
    ├── assets/          # Asset definitions
    └── ... (existing presets, locales)
```

## 8.3 Data Flow

```
[World Generation] → [LayerPack + Locations]
                            ↓
[Game Session] → [Character State] + [World State]
                            ↓
[Player Input] → [Move Resolution] → [State Update]
                            ↓
[CLI Render] ← [Oracle Prompts] ← [Chronicle Entry]
```

---

# 9. Analogue Compatibility

## 9.1 The Constraint That Clarifies

Every mechanic must be playable with:
* 2d10 (Action dice)
* 2d10 of different color (Challenge dice)
* Paper character sheet
* Printed world map
* Oracle tables (from the game or Ironsworn)

If the digital version does something the player cannot replicate at the table, the design is wrong.

## 9.2 Export Features (Phase 4)

* **Character Sheet PDF:** Printable sheet with all tracks
* **World Map Export:** Regional maps suitable for printing
* **Location Cards:** Generated locations as reference cards
* **Oracle Booklet:** All tables in printable format
* **Chronicle Export:** Session log as readable document

## 9.3 Hybrid Play

A player could:
1. Generate a world digitally
2. Print the map and tables
3. Play at the table using Ironsworn rules
4. Return to digital to update the world state

The digital version is an **enhanced oracle**, not a replacement for imagination.

---

# 10. Comparison: Old vs. Reimagined

| Aspect | Original pGLOWrpg | Reimagined |
|--------|-------------------|------------|
| Core question | "Can I simulate a world?" | "Can I tell a story in a world?" |
| Player role | Observer | Protagonist |
| Goal system | None | Vows with progress tracks |
| Resolution | None (pure data display) | Ironsworn-style action rolls |
| Combat | None | Move-based with progress |
| NPCs | None | Connections with relationships |
| Time | None | TU → World Event Ticks |
| Failure | Not possible | Meaningful (Miss consequences) |
| Success | Not defined | Defined (Strong/Weak hits) |
| Narrative | Implied by data | Emergent from play |
| Scope | Infinite (undefined) | Personal (your wanderer) |

---

# 11. Inspirations & References

* **Ironsworn / Ironsworn: Starforged** by Shawn Tomkin — Core mechanical inspiration
* **Dwarf Fortress (Adventure Mode)** — Procedural world, personal scale
* **Caves of Qud** — Procedural narrative through systems
* **Traveller** — Classic RPG structure for exploration
* **The One Ring RPG** — Journey mechanics
* **Classic Roguelikes** — Permadeath weight, procedural challenge

---

# 12. Design Decisions (Resolved)

These questions have been resolved:

---

## 12.1 Magic

**Decision:** Include in Phase 1 as a stub or very simple mechanics.

Magic should be present from the start, even if minimal. A single "Ritual" asset type with 2-3 basic effects (e.g., Healer's Touch, Spirit Sight) establishes that magic *exists* without requiring a full system. This sets player expectations early: magic is rare, costly, and not a crutch.

**Phase 1 Implementation:**
* 1-2 Ritual assets available at character creation (optional)
* Simple cost structure (spend Spirit to activate)
* No complex interactions or spell lists

---

## 12.2 Permadeath & The Soul System

**Decision:** Full permadeath, BUT with a soul/reincarnation mechanic inspired by Xianxia fiction.

### The Core Concept

Characters (player and NPC alike) possess **Souls**. When a character dies, their soul enters a cycle of reincarnation. Death is permanent for the *person*, but not for the *soul*.

### Gameplay Implications

| Event | Consequence |
|-------|-------------|
| **Character dies** | The person is gone. Their memories, relationships, skills—all lost. |
| **Soul persists** | The soul enters the reincarnation queue. Eventually, a new person is born carrying that soul. |
| **Soul marks** | The new person has no memory of their past life, but carries **Soul Marks**—echoes that manifest as inexplicable affinities, fears, or connections. |
| **Recognition** | NPCs or locations tied to the previous life may trigger vague recognition or déjà vu moments. |

### The Player Experience

When your character dies:
1. **The Chronicle records their story.** Their deeds become history.
2. **Their soul is marked.** Significant relationships, vows fulfilled/forsaken, and manner of death leave imprints.
3. **You create a new character.** Fresh person, fresh start.
4. **Soul Marks emerge over time.** Your new character may feel drawn to places, people, or causes connected to their soul's past—without knowing why.

### Why This Works

* **Death still matters.** You lose everything personal—skills, relationships, progress. This preserves tension.
* **Emotional continuity.** The companion who died saving you will be reborn. You might meet them again, as a child, as a stranger, as an enemy—and feel the echo.
* **Plot hooks regenerate.** The soul system creates natural story seeds. Why does this stranger trust you? Why do you hate the sea?
* **NPC depth.** NPCs also reincarnate. A villain's soul may return in an innocent. A lost love may find you again.
* **Soft landing.** Total permadeath can feel punishing. This offers emotional softening without mechanical resurrection.

### Soul Mechanics (Phase 2+)

* **Soul Strength:** Souls accumulate weight through significant deeds. Stronger souls reincarnate faster and with more marks.
* **Soul Dissipation:** The ultimate failure state. A soul can be *destroyed* through certain catastrophic events (betraying a sacred vow, certain magical attacks, dying forgotten and alone). This is true permadeath—no return.
* **Fate Threads:** Souls that interacted intensely may be "bound"—they tend to reincarnate near each other, creating recurring relationships across lifetimes.

### Design Pillar

> **"Death ends a life, not a story."**

This becomes a core pillar of the game's identity, distinguishing it from standard roguelikes while maintaining meaningful stakes.

---

## 12.3 World Persistence

**Decision:** Yes. The world persists across all characters.

### The Living World

Like Dwarf Fortress, you generate one world and inhabit it with successive characters. The world accumulates history:
* Events are logged in the Chronicle
* NPCs live, die, and are remembered (or forgotten)
* Factions rise and fall
* Your previous characters become part of the world's history

### The Metagaming Problem

**Problem:** If the player remembers everything from their previous character, they could simply recreate relationships, find the same treasures, or pick up abandoned vows with perfect knowledge.

**Solution: Knowledge Boundaries**

| Knowledge Type | Persists for Player? | Persists for New Character? |
|----------------|---------------------|----------------------------|
| World geography | Yes (map explored) | Partial (common knowledge only) |
| NPC existence | Yes | No (must meet again) |
| Relationship status | Yes (as history) | No (stranger to them) |
| Hidden locations | Yes (player knows) | No (must discover) |
| Faction standings | No (character-specific) | Fresh start |
| Vow progress | No | New vows required |

### Mechanical Enforcement

* **Fog of War Reset:** A new character starts with only "common knowledge" map visibility. Major settlements are known; secret locations are hidden again.
* **Relationship Reset:** NPCs treat your new character as a stranger. You must rebuild trust. (Soul Marks may create shortcuts—an NPC might feel unexplained warmth toward you.)
* **Chronicle Access:** Your new character can *hear stories* about previous heroes. "There was a wanderer named Kira who slew the beast of Thornwood..." This provides narrative continuity without mechanical advantage.

### The Emotional Loop

1. Play Character A. Build relationships. Die.
2. Create Character B. The world remembers A. You're a stranger now.
3. Encounter an NPC who knew A. They tell stories of your past self. You feel the echo.
4. Soul Marks draw you toward unfinished business—but you must earn your place again.

This creates a bittersweet, resonant experience that rewards long-term investment in a single world.

---

## 12.4 Difficulty & Death Spiral

**Decision:** Ironsworn-style lethality with layered "fate-changing" safety nets.

### Death is a Process, Not an Instant

Combat and hardship should feel dangerous, but death should rarely come from a single bad roll. Instead, there's a cascade:

### The Descent

| Stage | Trigger | Consequence |
|-------|---------|-------------|
| **Healthy** | Default state | Normal play |
| **Wounded** | Health reaches 0 | Must **Endure Harm**. On a miss, become... |
| **Broken** | Failed Endure Harm | Severe penalties. One more serious harm and... |
| **Death's Door** | Harm while Broken | Must **Face Death**. Roll to survive. |
| **Dead** | Failed Face Death | Character dies. Soul enters reincarnation. |
| **Soul Dissipation** | Special circumstances | True permadeath. No reincarnation. |

### Fate-Changing Interventions

At various stages, the player can invoke **Fate** to avoid the worst:

| Intervention | Cost | Effect |
|--------------|------|--------|
| **Burn Momentum** | Reset momentum to +2 | Convert a miss to a weak hit (if momentum > challenge die) |
| **Sacrifice Supply** | Lose all Supply | Escape a deadly situation (narratively justified) |
| **Break a Bond** | Permanently damage a Connection | An ally intervenes, saving you but straining the relationship |
| **Invoke a Vow** | Forsake a vow (lose all progress, gain "Forsaken" mark) | Desperate survival at the cost of your word |
| **Soul Bargain** | Gain a permanent Soul Mark (negative) | Supernatural intervention saves you, but marks your soul |

### The Final Deaths

**Character Death:** Your body dies. Your soul persists. You create a new character. The world remembers the old one.

**Soul Dissipation:** Triggered by:
* Dying with a "Forsaken" mark on a sacred vow
* Being slain by certain supernatural forces
* Dying utterly alone, unmourned, with no bonds or legacy
* (Rare) Choosing to let go

Soul Dissipation is **true permadeath**. The soul is gone. No reincarnation. The Chronicle notes them as "Lost to the Void." This should be rare but possible—the ultimate consequence for a life of betrayal or isolation.

---

## 12.5 Procedural NPCs

**Decision:** Combined approach—procedural generation with player control.

### Default Mode: Procedural

NPCs are generated using:
* **Name tables** (culturally appropriate to region/faction)
* **Descriptor tables** (personality traits, appearance, quirks)
* **Role assignment** (based on location type)
* **Goal generation** (what do they want?)

### Player Override

At any point, the player can:
* **Accept the oracle's suggestion** (use generated NPC as-is)
* **Reroll specific elements** (keep the name, change the personality)
* **Input custom details** (fully define the NPC themselves)

### Hybrid Flow Example

```
═══════════════════════════════════════════
  NEW NPC ENCOUNTERED
═══════════════════════════════════════════
  
  Oracle suggests:
    Name: Torvin
    Role: Traveling merchant
    Trait: Suspicious, pragmatic
    Want: Safe passage through the forest
  
  [1] Accept as generated
  [2] Reroll name
  [3] Reroll traits
  [4] Custom input
  [5] Reroll everything

> _
```

This respects both the "oracle as collaborator" philosophy and player agency.

---

## 12.6 Multiplayer

**Decision:** No multiplayer. Ever.

### Rationale

* **Solo play is the core identity.** The game is designed around a single wanderer's journey. Adding multiplayer would fundamentally alter the pacing, narrative control, and mechanical balance.
* **Ironsworn works solo.** The inspiration is a solo/co-op TTRPG. The digital version should honor that.
* **Scope control.** Multiplayer adds massive technical complexity (networking, synchronization, balance) for unclear benefit.
* **The soul system is personal.** Reincarnation, fate threads, soul marks—these work because they're *your* story. Multiplayer dilutes this.

### What About Co-op?

If co-op were ever considered (it won't be), it would be:
* Asynchronous (players in the same world, different times)
* Narrative only (share chronicles, not mechanics)
* Post-Phase-4 (after the solo game is complete)

**For all practical purposes: This is a single-player game. Design accordingly.**

---

## 12.7 Summary Table

| Question | Decision |
|----------|----------|
| Magic in Phase 1? | Yes, minimal stub (1-2 rituals) |
| Permadeath? | Yes, but with soul/reincarnation system |
| World persistence? | Yes, with knowledge boundaries for new characters |
| Difficulty? | Layered death spiral with fate interventions |
| Procedural NPCs? | Hybrid (generated + player override) |
| Multiplayer? | No, never |

---

# 13. Success Criteria

## Phase 1 Success

The game is successful if:
* A player can complete a 30-minute session with a clear narrative arc
* The action roll feels meaningful and dramatic
* The world feels coherent (biomes make sense, locations fit)
* A vow can be sworn and fulfilled
* Combat feels dangerous but winnable
* The player wants to play again

## Overall Success

The game achieves its vision if:
* Players tell stories about their wanderers
* Each playthrough feels different
* The world feels vast but not empty
* Death is sad, not frustrating
* The analogue version is genuinely playable
* Someone who's never heard of Ironsworn can learn in 10 minutes

---

# Appendix A: Example Play Session

```
═══════════════════════════════════════════
  pGLOWrpg — New Wanderer
═══════════════════════════════════════════

Name your wanderer: > Kira

Distribute stats (+3, +2, +2, +1, +1):
  Edge (speed, precision): > +2
  Heart (courage, connection): > +2  
  Iron (strength, endurance): > +1
  Shadow (stealth, cunning): > +3
  Wits (knowledge, survival): > +1

Kira, a wanderer defined by cunning and quick reflexes.
What drives you? 
  [1] Seeking someone lost
  [2] Fleeing a past
  [3] Searching for purpose
  [4] Custom
> 1

Who are you seeking? > My sister, taken by slavers

═══════════════════════════════════════════
  SWEAR AN IRON VOW
═══════════════════════════════════════════
  "I will find my sister and free her from bondage."
  
  This is an EXTREME vow (life-defining).
  
  Rolling +Heart (+2)...
  ACTION: [5] + [8] + 2 = 15
  CHALLENGE: [4] [9]
  
  ★ WEAK HIT ★
  
  You swear the vow, but the path is dark. You have only 
  a rumor: slavers were seen heading toward the eastern 
  marshes. The journey will be long and dangerous.
  
  +2 Momentum (now 4)
  Vow begun: [□□□□□□□□□□]
  
  Your journey begins at Millbrook, a small farming village
  at the edge of the Greenmark...

> _
```

---

*Document ends. The world awaits.*
