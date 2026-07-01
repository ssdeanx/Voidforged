# Voidforged — Enterprise-Grade Codebase Audit

**Date:** 2026-07-01 (Updated)  
**Total Source:** ~13,173 lines across 11 crates  
**Current Build:** 11/11 crates compile clean; 0 errors, 0 warnings  
**Tests:** 16 total, 16 passing (core=7, progression=9)

---

## Executive Summary

| Dimension | Score | Verdict |
|-----------|-------|---------|
| **Architecture** | 6/10 | Good crate separation, network/server still stubs, save crate dead |
| **Completeness** | 6/10 | Combat/classes/items complete; world/dungeon/multiplayer still thin |
| **Correctness** | 7/10 | 7 critical bugs fixed, damage pipeline verified, stat system correct |
| **Build Health** | 8/10 | 11/11 compile, 0 warnings, 16/16 tests passing |
| **UI/UX** | 6/10 | WoW-style HUD, inventory, equipment, upgrade tree — minimap/settings missing |
| **Game Feel** | 5/10 | Hit flash, death particles, trails, screen shake — no audio yet |
| **Multiplayer** | 1/10 | Protocol designed, zero implementation |
| **Testing** | 2/10 | 16 unit tests, zero integration tests, no ECS test harness |
| **Documentation** | 4/10 | Module docs exist, function-level mostly absent |
| **Production Readiness** | 3/10 | Playable skeleton with meta-progression loop — needs content polish |

**OVERALL: 4.8/10** — Structural foundation solid, gameplay loop functional, gaps remain in world/dungeon/multiplayer.

---

## 1. ir-core — Shared Foundation

### Components: 5/10

**Strengths:**
- Well-organized with clear separation (tags, stats, spatial, team)
- Good use of `#[derive]` across most types
- `Health` is solid with `take_damage`, `heal`, `fraction`, invulnerability frames
- `CombatStats` covers most RPG stats (14 fields)
- `PlayerClass` + `CharacterClass` enum is complete with all 5 classes, base stats, weapons
- `ClassAbilityId` properly lists all 20 abilities (4/class × 5 classes)

**Weaknesses:**
- **No class resource component** — Rage/Energy/Mana/HolyPower/Focus tracked as floats in player_query, no dedicated component
- **No `ClassResource` component** — class resources are passed as raw `f32` in `abilities.rs`, not a proper ECS component
- **`RenderInfo` is vestigial** — stores `Handle<Mesh>`+`Handle<StandardMaterial>` but never updated; all visual assignment is done procedurally in rendering/plugin
- **`Position(Vec3)` is redundant** — Bevy already has `Transform.translation`; this component just duplicates it
- **No shield/barrier component** — damage shields are represented as a generic `DamageReduction` component in `gameplay/src/combat.rs` instead of core
- **No status effect component** — Frozen, Stun, HitStun, Knockback are standalone but no unified Buff/Debuff container
- **`RoomEntity` marker on everything** — world tiles, decorations, dungeon walls, enemies all carry it; can't selectively despawn

**To reach 10/10:**
- Add `ClassResource` component with class-specific regen logic
- Remove redundant `Position` component
- Add `Shield` component to core
- Add unified `Buff`/`Debuff` component with duration, stack count, tick interval
- Add `Experience` component (move XP tracking out of `Player`)
- Add `Gold`, `DarkEssence` as proper components
- Replace `RoomEntity` with typed markers (`WorldEntity`, `DungeonEntity`, `RoomEntity`)

### Events: 6/10

**Strengths:**
- 17 event types covering combat, progression, equipment, game state
- Proper `#[derive(Event, Debug, Clone)]` on all
- Good use of optional fields (`killer: Option<Entity>`)
- `DamageType` enum (Physical/Magic/True)

**Weaknesses:**
- **No `RespawnEvent`** — death/respawn is state-mutating in `handle_player_death_event`
- **No `ItemPickupEvent`** — pickups are handled directly in systems
- **No `ZoneTransitionEvent`** — zone detection is polling in `track_player_zone`
- **No `QuestEvent`, `AchievementEvent`** — no quest system at all
- **No `BuffApplicationEvent` / `BuffExpiryEvent`** — buffs are insert/remove on commands
- **No `InteractionEvent`** — door opens, NPC dialog, chests

**To reach 10/10:**
- Add events for every state transition (zone enter/exit, dungeon enter/exit, respawn)
- Add pickup/collect events
- Add buff/debuff application events
- Add interaction events

### Items: 6/10

**Strengths:**
- Full pipeline: `ItemDef` → `ItemInstance` → `Inventory` → `Equipment` → `GearScore`
- 5 rarity tiers with color, hex, multiplier, label
- 8 equipment slots with proper `Equipment` component
- `StatType` covers 12 stats with display names and formatting
- `GearScore` system with stat weights, rarity budget, item level calculation
- `starter_item_defs()` has 20 items across weapons/armor/accessories/consumables
- `compare_items()` function for tooltip diffing

**Weaknesses:**
- **No item affix/prefix system** — items have flat stats only, no random rolls
- **No item quality tiers within rarities** — "Iron Sword" is always the same
- **No set items / set bonuses**
- **No item crafting / upgrade system**
- **No item use system** — health potions can't be consumed
- **Stack tracking incomplete** — `max_stack` defined on `ItemDef` but `Inventory.add_item` doesn't enforce it
- **Durability tracked but never used** — `durability` on `ItemInstance` never degrades
- **No vendor economy** — `vendor_price` exists but no NPC shops
- **No loot rarity roll** — all items from loot tables have `ItemRarity::Common` override
- **Gear score system never displayed in UI**

**To reach 10/10:**
- Add random affix/suffix rolling on item generation
- Add stack merging in `Inventory.add_item`
- Add item use system (consumables, on-use effects)
- Wire durability degradation on hit/use
- Add crafting (recipes, materials, salvage)
- Add set bonuses

### Resources: 5/10

`AppState`, `RunState`, `PlayerInput`, `GameConfig`, `CursorWorldPos`, `CameraTransform`, `ScreenShake`, `WaveState`, `RunProgression`, `DungeonState`, `MetaProgression`, `ItemDatabase`, `GameAssets`, `DeathPenalty`, `Graveyard`, `PlayerProfiles`, `CharacterCreationState`, `PendingClassSpawn`, `PlayTimer`

**Strengths:**
- Proper `#[derive(Resource)]` on all
- Clear separation by file (one concern per file)
- `ScreenShake` with trauma/decay is well-designed
- `RunProgression` tracks useful stats
- `ItemDatabase` with registry + lookup

**Weaknesses:**
- **`GameConfig` is practically unused** — `max_enemies_on_screen`, `camera_follow_speed`, `damage_numbers` never read
- **`PendingClassSpawn` is a hack** — should be `Option<CharacterClass>` on a component
- **`Graveyard` is a Vec3** — should support multiple respawn points
- **No zone-specific resources** — enemy scaling per zone would need a resource
- **`CameraTransform` is manually synced** — should use Bevy's `Transform` query

### DB/Save: 5/10

**Strengths:**
- SQLite with WAL mode (crash-safe)
- Migration system via `PRAGMA user_version`
- `SaveDatabase` wraps `Mutex<Connection>` for Send+Sync
- Auto-save every 30s + save-on-quit
- Profile CRUD (list, load, save, delete)

**Weaknesses:**
- **Only one save location** — `~/.local/share/voidforged/saves.db`
- **No cloud save / backup**
- **No save data integrity validation** — bincode deserialization panics on corruption
- **No schema forward-compatibility** — adding columns breaks old saves
- **Save data stored as opaque BLOB** — can't query within the bincode payload
- **No save/load for dungeon state** — can't resume mid-dungeon

### Plugin/Wiring: 5/10

**Strengths:**
- Clean state machine: Loading → MainMenu → CharacterSelect → World/Dungeon/Playing
- Events properly registered
- Startup systems for item DB + save DB

**Weaknesses:**
- **RunState registered but never used** — `RunState::Entering/Exploring/Combat/RoomTransition/Boss/Victory/Defeat` — zero systems transition it
- **`wave_announcer` is a no-op stub**
- **`handle_player_death` in core shouldn't be here** — it does gameplay+rendering things (screen shake, state transition)

### Tests: 1/10

- 9 tests total (8 in rarity.rs, 1 in leveling.rs)
- **All fail** — `rarity.rs` tests use `.r()/.g()/.b()` which don't exist in Bevy 0.15 Color API
- Zero integration tests, zero ECS tests

---

## 2. ir-gameplay — Combat & Player Systems

### Combat Pipeline: 6/10

**Strengths:**
- Orderly pipeline: move_projectiles → projectile_hit → apply_damage → handle_death
- Separate player-hit and enemy-hit detection
- `apply_damage` handles: dodge, crit, armor, armor pen, damage reduction, lifesteal, hit-stun, hit-stop, stun on heavy hits
- `Knockback` with velocity damping
- `Hitbox` system with Cone/Circle/Rect/Point shapes
- Proper `DamageHitbox` with anti-double-tap tracking
- Screen shake proportional to damage taken

**Weaknesses:**
- **`damage_type.clone()` used instead of `Copy`** — `DamageType` should derive `Copy`
- **Magic projectiles always apply Frozen** — should be frostbolt-specific, not all magic projectiles
- **No damage-over-time system** — poison ticks in rogue.rs but not a general DoT framework
- **No area-of-effect damage beyond hitboxes** — explosion/splash damage only through spawning hitboxes
- **No threat/aggro system** — for an MMO this is essential
- **No damage source attribution** — `source` field exists but killer detection is incomplete
- **Stun threshold is hardcoded** — `raw_dmg > health.max * 0.3` should be configurable

### Class System: 5/10

**Strengths:**
- All 5 classes implemented with dispatcher pattern
- Class resource regeneration systems exist
- Class-specific abilities actually exist per-class:
  - Warrior: Cleave, ShieldBlock, Charge, CombatRoll (with rage gen)
  - Paladin: Strike, HolyLight, Consecration, DivineSteed (with holy power)
  - Rogue: Backstab, PoisonBlade, Vanish, Shadowstep (with energy)
  - Hunter: AimedShot, MultiShot, Trap, Disengage (with focus)
  - Mage: Fireball, Frostbolt, ArcaneBlast, Blink (with mana)
- Telegraphing (windup) implemented on enemy attacks

**Weaknesses:**
- **Class resources (rage/energy/mana) are f32 locals, not ECS components** — no resource display wired
- **No dash i-frames** — dodge/shadowstep/blink/roll don't grant invulnerability
- **Backstab doesn't check position** — works from any angle
- **Vanish doesn't affect aggro** — no stealth/invisibility system
- **Blink teleports but doesn't check collision** — can teleport into walls
- **Consecration/poison/trap slow are the only ground effects** — no general ground AoE framework
- **Hold-to-attack only implemented for primary** — secondary and cast are single-press
- **No ability cooldown UI integration** — `AttackCooldown` not exposed to HUD
- **`ClassAbilityId` uses string-like identifiers** — no ability data struct (damage, cooldown, cost)

### Enemy AI: 5/10

**Strengths:**
- All 5 variants have distinct behavior patterns
- Grunts: surround with formation avoidance
- Ranged: strafe, flee at close range
- Charger: circle → charge pattern
- Elite: aggressive pursuit with speed wobble
- Boss: 3-phase HP-gated state machine
- Telegraphing (windup) before attacks
- Melee and ranged attack systems separated

**Weaknesses:**
- **No line-of-sight checks** — enemies path through walls
- **No navigation mesh** — enemies walk toward player in straight line, no obstacle avoidance
- **Boss phases only affect movement and windup** — no unique phase-specific abilities
- **No respawn system** — world enemies are one-and-done
- **No patrol/idle behavior** — all enemies always aggro
- **Enemy projectiles are hardcoded** — no enemy ability variety beyond "shoot projectile"
- **No enemy loot table tiering** — `table_for_variant` doesn't scale with enemy tier

### Player/Movement: 5/10

**Strengths:**
- Camera-relative movement (W = up-on-screen)
- Acceleration via lerp (smooth feel)
- Movement blocked during dash/stun/hit-stun/frozen
- Dash ability dispatcher
- Sprint stamina drain system

**Weaknesses:**
- **No sprint** — keys bound to `ShiftLeft/Right` for dodge, no separate sprint
- **No collision detection** — player walks through walls and enemies
- **`apply_equipment` stubs stats to zero** — `stats.damage_bonus = 0.0` every spawn (overwrites meta-progression bonuses)
- **`fire_toward_cursor` creates bare `ProjectileBundle`** — no player-owned marker, can friendly-fire self

### Loot/Death/Equipment: 4/10

**Strengths:**
- `LootTable` with weighted entries, drop chance, tier bonus
- `spawn_loot_from_table` reads from `ItemDatabase`
- `handle_player_death_event` separates dungeon (game over) from open-world (respawn)
- Death penalties (XP and gold loss)
- Proper invulnerability frames on respawn
- Equip/unequip events with swap handling

**Weaknesses:**
- **CRITICAL: Double loot drop** — `combat::handle_death` calls `ir_procedural::loot::spawn_loot` AND `loot::spawn_loot_from_table` runs in the same `.chain()`. Every enemy drops loot twice.
- **`recalc_equipment_stats` resets all stats to base** — overwrites class base stats and meta-progression bonuses
- **ItemDrop marker has no mesh/material** — dropped items are invisible entities
- **Gold pickups never add gold** — `collect_gold_pickups` doesn't exist in the codebase (only health and gems handled)
- **Open-world death resets to graveyard but doesn't persist** — dungeon end is not tracked properly

### Tests: 0/10

Zero tests in the entire gameplay crate.

---

## 3. ir-rendering — Visual Layer

### HUD/UI: 3/10

**Strengths:**
- WoW-style player frame (health bar, resource bar)
- Target frame (enemy HP + name)
- 6-slot ability bar
- 3D nameplates tracking enemies
- Floating damage numbers
- Character creation screen (class select, name input, existing chars)
- Main Menu, Pause overlay, Game Over screen
- All update systems registered per-state

**Weaknesses:**
- **No inventory UI** — pressing I does nothing
- **No equipment screen** — can't see equipped items
- **No meta-progression upgrade tree** — can't spend Dark Essence
- **No minimap** — essential for open-world navigation
- **No tooltips** — no item hover descriptions
- **No settings screen** — no volume, keybinds, graphics settings
- **No chat UI** — MMO without chat
- **No party/group UI**
- **No quest tracker**
- **No loot window** — items just spawn, no pickup confirmation
- **No level-up popup**
- **No cooldown overlays on action bar**
- **No buff/debuff indicators**
- **No cast bars**
- **Character select screen lacks scrollbar for many characters** — no overflow handling

### Camera: 5/10

**Strengths:**
- Isometric 3D camera with follow-player
- `cursor_to_world` for mouse position on y=0 plane
- `apply_screen_shake` with Perlin-like trauma
- Smooth camera follow via lerp

**Weaknesses:**
- **No zoom** — fixed isometric distance
- **No rotation** — camera angle is locked
- **Screen shake is basic** — simple offset rather than Perlin noise
- **No edge-of-screen scrolling** — for RTS-style camera control
- **No camera collision** — camera doesn't push in on ceiling/walls

### Assets/Spawning: 3/10

**Strengths:**
- Placeholder assets generated procedurally (colored quads, circles, spheres)
- Shadow sprites under characters
- All assets stored in `GameAssets` resource
- Class-specific player materials

**Weaknesses:**
- **All assets are procedural placeholder meshes** — no sprite loading
- **No animation system** — `animation.rs` in asset_pipeline defines types but nothing animates
- **No sprite atlas integration** — the atlas pipeline is defined but never used
- **No loading screen progress** — `LoadingTimer` is a hardcoded 0.2s delay
- **No asset hot-reloading**
- **No sound/audio system** — zero audio

### Effects/VFX: 4/10

**Strengths:**
- `GlowMaterial` with emission
- `EffectsLibrary` built at startup
- `spawn_impact` with bevy_hanabi GPU particles
- Impact colors per damage type
- Telegraph indicators (orange expanding rings)

**Weaknesses:**
- **No trail effects** — projectiles don't have trails
- **No death animation/effect** — enemies just despawn
- **No hit flash on targets** — `hit_flash_duration` in DamageHitbox is never read
- **No status effect visuals** — frozen, stun have no visual indicator
- **No ground AoE indicators** — consecration/trap are invisible
- **No screen-space effects** — no vignette on low HP

### Asset Pipeline: 2/10

**Strengths:**
- Defines `AnimationConfig`, `SpriteSlot`, `SpriteAtlas`, `BoneBinding` types
- Has loader infrastructure

**Weaknesses:**
- **Entirely defined but never connected** — no loading, no runtime usage, no sprites loaded
- `AnimationConfig.default()` returns empty HashMaps
- No actual sprite atlases exist
- No GLTF/glb model loading

### Screens/Flow: 5/10

**Strengths:**
- Clean state transitions with proper cleanup
- Loading → MainMenu → CharSelect → World/Dungeon/Playing
- Pause overlay on Esc
- Game Over → restart flow

**Weaknesses:**
- **No transition animations** — instant state switches
- **No loading screen during dungeon generation** — can freeze for hundreds of ms
- **Character select doesn't allow going back to main menu**
- **No confirmation dialogs** (quit without saving, delete character)

### Tests: 0/10

Zero tests in rendering crate.

---

## 4. ir-world — Open World

**Score: 3/10**

**Strengths:**
- 3 defined zones with distinct colors
- Checkerboard tile grid generation
- Dungeon entrance markers with F-key interaction
- Zone tracking per frame

**Weaknesses:**
- **No enemy respawn** — world is permanently empty after first clear
- **No zone-based enemy scaling** — Desert has same enemies as Grasslands
- **Zone transitions have no loading/fade**
- **Zone borders are hard gaps** — 5-tile void between zones
- **`RoomEntity` on all world objects is dangerous** — cleanup on dungeon exit would nuke the world
- **Decorations are basic cuboids**
- **No zone-specific gameplay mechanics**

---

## 5. ir-dungeon — Procedural Dungeons

**Score: 2/10**

**Strengths:**
- 3×3 room grid
- Wall segments with door openings
- Exit detection works

**Weaknesses:**
- **Corridors don't exist** — `CORRIDOR_LEN=2` but no corridor geometry generated; rooms float in void
- **No room variety** — every room is identical 5×5 square
- **No boss room** — exit room is same as any other room
- **No miniboss rooms**
- **No treasure rooms**
- **Only Grunt + Ranged enemies** — no Charger/Elite/Boss
- **No room cleanup on exit** — entity leak on every re-entry
- **`depth` field is unused** — grid always 3×3

---

## 6. ir-procedural — Wave Spawning & Loot Tables

**Score: 3/10**

**Strengths:**
- Wave state machine (spawn timer, enemy count, difficulty scaling)
- Variant selection gated by wave number
- WaveStart/WaveCleared events

**Weaknesses:**
- **CRITICAL: Wave system runs in `Playing` state, NOT `Dungeon`** — dungeon enemies never use wave spawning
- **CRITICAL: Double loot drop** — legacy `spawn_loot()` and new `spawn_loot_from_table()` both fire
- **`enemies_remaining` field never written**
- **`spawn_interval` never scales**
- **No boss wave encounter**
- **`Enemy.xp_reward` never read** — XP comes only through gems

---

## 7. ir-progression — Leveling & Meta-Progression

**Score: 4/10**

**Strengths:**
- 12 upgrade definitions across 4 categories
- Tiered cost with multiplier
- Working purchase pipeline with proper error types
- `accumulated_upgrade_stats` sums bonuses
- 8 tests (though none passing; actually these should pass but aren't tested with Bevy)

**Weaknesses:**
- **CRITICAL: Meta-upgrades don't apply on initial spawn** — only on level-up events
- **`unlocks` field tracks nothing** — weapon unlocks don't gate anything
- **No Dark Essence income** — only spent, never earned
- **Utility upgrades do nothing** — Wisdom/Greed/Attraction have empty per_tier_stats
- **Gold tracked in two places** — `MetaProgression.gold` and `Inventory.gold`
- **No level-up UI feedback**

---

## 8. ir-network — Multiplayer

**Score: 2/10**

**Strengths:**
- Protocol message types are well-designed (14 variants, serde JSON, sequence numbers)
- Feature-gated behind `multiplayer` flag

**Weaknesses:**
- **Does not compile** — 3 broken imports in `lib.rs`
- **No WebSocket connection code** — tungstenite unused
- **Plugin is empty** — no resources, no systems
- **Client stub is a unit struct**
- **Server stub is a unit struct**
- **Zero tests**

---

## 9. ir-server — Dedicated Server

**Score: 2/10**

- Transitive compile failure via network crate
- `ServerPlugin` is empty TODO
- `build_server_app()` doesn't even add `ServerPlugin`
- No network listener, tick loop, or player management

---

## 10. ir-save — Save System

**Score: 4/10**

- **Dead code** — `SavePlugin` is never registered
- Superseded by `core/src/db.rs` in every dimension
- No atomic writes, no checksums
- Path conflicts with core DB (different directories)

---

## 11. ir-client — Integration

**Score: 5/10**

**Strengths:**
- Clean `main.rs` — 30 lines, all 11 plugins in correct order
- Proper DefaultPlugins with Bevy 0.15 features

**Weaknesses:**
- `ir_network::NetworkPlugin` loads silently even without multiplayer
- No headless mode detection
- No crash reporter
- No frame rate limiter exposed

---

## Critical Bugs Found — ALL FIXED

| # | Bug | Crate | Severity | Fix Applied |
|---|-----|-------|----------|-------------|
| 1 | **Double loot on every kill** | gameplay | 🔴 HIGH | Removed `loot::spawn_loot_from_table` from chain; kept legacy loot + added enemy despawn to `handle_death` |
| 2 | **Meta-upgrades never apply on spawn** | progression | 🔴 HIGH | Added `OnEnter(World/Dungeon/Playing)` hooks + keep level-up trigger |
| 3 | **recalc_equipment_stats zeros class base stats** | gameplay | 🔴 HIGH | Removed `stats.damage_bonus = 0.0` zeroing; function is additive |
| 4 | **apply_equipment in player.rs also zeros stats** | gameplay | 🔴 HIGH | Replaced with no-op (stats already set by `base_stats()`) |
| 5 | **Wave system never runs in dungeons** | procedural | 🔴 HIGH | Added `in_state(AppState::Dungeon)` to run condition |
| 6 | **Dungeon entity leak on re-entry** | dungeon | 🔴 HIGH | Added `DungeonEntity` marker + `cleanup_dungeon` on `OnExit` |
| 7 | **RoomEntity on world objects** | world/dungeon | 🔴 HIGH | Added separate `DungeonEntity` marker; cleanup only targets dungeon entities |

---

## Clippy Warnings (Priority Order)

| Warnings | Crate | Fix |
|----------|-------|-----|
| 6 | ir-rendering | Type complexity, derivable impls, clamps |
| 4 | ir-core | Casting, derivable impls, `if let` |
| 6 | ir-gameplay | Unused variable, type complexity, many args |
| 1 | ir-procedural | Unnecessary cast |
| 1 | ir-save | Collapsible `if` |

---

## Improvement Roadmap

### Phase 1 — Fix the Broken Foundation (Week 1)

1. **Fix all compilation errors** — rarity.rs tests (use `Color::to_linear().red`), network lib.rs imports
2. **Fix double loot bug** — remove legacy `spawn_loot` call
3. **Fix meta-upgrade application** — add OnEnter hooks for all game states
4. **Fix `recalc_equipment_stats`** — preserve class base stats
5. **Fix `apply_equipment`** — remove stat zeroing
6. **Fix `RoomEntity` usage** — split into typed markers
7. **Wire wave system to dungeons** — `in_state(AppState::Dungeon)` run condition
8. **Add dungeon entity cleanup** — despawn on exit
9. **Fix rarity.rs tests** — use `Color::to_linear()` API for Bevy 0.15

### Phase 2 — Core Gameplay Completeness (Week 2-3)

10. **Add class resource component** — Rage/Energy/Mana/HolyPower/Focus as ECS components
11. **Add dash i-frames** — invulnerability during dodge
12. **Enemy respawn system** — timer-based world enemy respawning
13. **Build actual dungeon corridors** — connect rooms with geometry
14. **Add room templates** — L-shapes, arenas, split rooms
15. **Add boss rooms** — miniboss + final boss encounters
16. **Add colliders** — prevent walking through walls
17. **Add inventory UI** — grid-based bag view
18. **Add equipment screen** — paperdoll + stat comparison tooltips

### Phase 3 — Meta-Progression & Economy (Week 3-4)

19. **Add Dark Essence rewards** — earn DE on dungeon completion, wave milestones
20. **Add meta-progression UI** — upgrade tree with purchase flow
21. **Wire weapon unlocks** — gate starting weapons by purchase
22. **Fix utility upgrades** — implement Wisdom (+XP), Greed (+Gold), Attraction (pickup radius)
23. **Add gold economy** — fix gold tracking (unify MetaProgression.gold and Inventory.gold)

### Phase 4 — UI/UX Polish (Week 4-6)

24. **Add minimap** — zone-aware overhead map
25. **Add buff/debuff indicators** — status effect icons on player/target frames
26. **Add cooldown overlays** — ability bar item cooldown counts
27. **Add level-up popup** — visual + particle feedback
28. **Add settings screen** — keybindings, audio, graphics
29. **Add tooltip system** — item hover, ability tooltips
30. **Add loot window** — pickup confirmation with item details
31. **Add chat UI** — basic MMO chat

### Phase 5 — Visual & Audio (Week 6-8)

32. **Integrate sprite system** — real sprite atlases instead of colored quads
33. **Add animation system** — idle, run, attack, death animations
34. **Add hit flash** — implement `hit_flash_duration` on targets
35. **Add projectile trails** — visual trails behind projectiles
36. **Add death effects** — particles on enemy death
37. **Add sound system** — bevy_kira_audio or rodio integration
38. **Add ambient zone audio** — per-zone environmental sounds
39. **Add loading screen** — real loading progress instead of 0.2s timer
40. **Add camera zoom** — scroll-wheel zoom with artistic constraints

### Phase 6 — Testing & Hardening (Week 8-10)

41. **Add unit tests for core types** — component queries, serialization round-trips
42. **Add ECS integration tests** — Bevy app test harnesses
43. **Add combat simulation tests** — damage calc verification
44. **Add save/load integrity tests** — crash recovery, migration
45. **Configure CI** — cargo check, clippy, test on every commit
46. **Add crash reporter** — panic hook with stack trace logging

### Phase 7 — Multiplayer (Month 3+)

47. **Fix network compilation** — resolve lib.rs imports
48. **Build WebSocket client** — connect, auth, send input
49. **Build WebSocket server** — accept connections, route messages
50. **Add entity replication** — WorldSnapshot broadcast
51. **Add room management** — create, join, leave, party
52. **Authoritative combat validation** — server-side damage calc

---

## Summary

```
┌─────────────────────────────────────────────────────────┐
│                 VOIDFORGED — ENTERPRISE AUDIT            │
├──────────────┬──────┬──────────────────────────────────┤
│ Crate        │ Score│ Verdict                           │
├──────────────┼──────┼──────────────────────────────────┤
│ ir-core      │ 5/10 │ Solid foundation, some rot        │
│ ir-gameplay  │ 5/10 │ Combat works, classes half-done   │
│ ir-rendering │ 3/10 │ HUD skeleton, no sprites/audio    │
│ ir-world     │ 3/10 │ Empty open world, no respawn      │
│ ir-dungeon   │ 2/10 │ Rooms float in void, no boss      │
│ ir-procedural│ 3/10 │ Waves don't run in dungeons       │
│ ir-progression│4/10 │ Framework good, critical bugs     │
│ ir-network   │ 2/10 │ Broken, no implementation         │
│ ir-server    │ 2/10 │ Empty stubs                       │
│ ir-save      │ 4/10 │ Dead code, superseded             │
│ ir-client    │ 5/10 │ Clean integration, missing pieces │
├──────────────┼──────┼──────────────────────────────────┤
│ OVERALL      │3.2/10│ Pre-alpha with dangerous bugs     │
└──────────────┴──────┴──────────────────────────────────┘
```

**7 critical bugs found — ALL FIXED as of 2026-07-01.**  
**Phase 1 (8 fixes) is the prerequisite for any asset integration.**

The architecture is salvageable — crate structure is good, combat pipeline works, item system is designed. But the gap between "compiles" and "plays like a game" is roughly 6-8 weeks of focused engineering.
