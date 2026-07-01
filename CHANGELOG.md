# Changelog

## v0.6.0 — Documentation overhaul + MMO architecture foundation

### Added
- **AGENTS.md** rewritten for Voidforged — build commands, architecture data flow, ECS conventions, class system table, security baseline, engine guidance
- **.env.example** rewritten for MMO server — PostgreSQL, Redis, JWT auth, server config
- **README.md** updated to v0.5.0 with full feature inventory, MMO architecture diagram, workspace crate table, controls, class table, milestones

### Changed
- **Database architecture**: SQLite is now a CLIENT-SIDE dev/testing store. Production MMO uses server-side PostgreSQL via sqlx.
- `core/src/db.rs` — SQLite `SaveDatabase` wrapped in Mutex for Bevy Send+Sync compatibility, `PlayerProfile` is now a Bevy Component for auto-save query, `CharacterClass` implements `Default` + `FromStr`
- Core plugin registers `init_save_db`, `auto_save` (every 30s), `save_on_quit` systems
- `init_item_database` moved to `items_db.rs` (was deleted from plugin.rs)

### Fixed
- `DungeonState` is now correctly accessed as `Res<DungeonState>` not `Option<&DungeonState>`
- `PlayerDeathEvent` now correctly includes `player: Entity` and `killer: Option<Entity>`
- Removed unused `death_events` reader from `handle_player_death`

## v0.5.0 — Combat pipeline, hitbox system, loot tables, gear score, stamina, HUD updates

### Added
- **Damage pipeline** — `apply_damage` now properly handles armor, dodge, crit, lifesteal, armor pen, damage reduction buffs
- **Hitbox system** — `DamageHitbox` entities with configurable shapes (Cone, Circle, Rect, Point), processed per-frame for overlap → `DamageEvent`
- **Melee classes now use hitboxes** — Warrior/Paladin/Rogue spawn `DamageHitbox` instead of direct distance check. Removed old `handle_melee_swing` + `MeleeSwing`
- **Stamina system** — `Stamina` component + regen + sprint drain. Sprint costs 20/s when moving + holding Shift
- **Sprinting** — `Sprinting(pub bool)` component, `sprint_stamina_drain` system
- **Loot tables** — `LootTable` + `LootEntry` with weighted random drops per enemy variant. `spawn_loot_from_table` listens to `DeathEvent` and spawns `ItemDrop` entities
- **Gear score / Item level** — `core/src/items/gear_score.rs` with stat weight formula, rarity budget, ilvl calculator, gear score, loot generation tables by player level, gear comparison (`compare_items`)
- **ItemDatabase** — populated at startup with all 19 starter items from `starter_item_defs()`
- **HUD stamina bar** — `HudStaminaBar` + `HudStaminaText` components, `update_stamina_bar` system
- **Death & respawn** — `PlayerDeathEvent` handler: open world (graveyard + XP/gold penalty), dungeon (end + score). `DungeonEndEvent`, `Graveyard`, `DeathPenalty` resources
- **Equipment events** — `EquipItemEvent` + `UnequipItemEvent` with full swap logic
- **No RNG loot** — Loot is now specific by rarity tiers with stat-weighted budgets via `gear_score.rs`

### Changed
- `core/src/items.rs` → modular `items/` directory (8 files: mod.rs, rarity.rs, slots.rs, modifiers.rs, definitions.rs, instance.rs, inventory.rs, equipment.rs)
- `core/src/resources.rs` → modular `resources/` directory (11 files)
- `apply_damage` now checks `ShieldBlock` / `DamageReduction` buffs
- Stamina: 100 max, regen 15/s, dodge costs 25, sprint costs 20/s
- Movement: sprint speed bonus when stamina > 0
- `hud/mod.rs` exports `update_stamina_bar`

### Removed
- `handle_melee_swing` (replaced by hitbox spawning)
- `MeleeSwing` struct (dead code)
- Old `ProjectileSpawnEvent`, `WaveEnemySpawnedEvent` (unused)

## v0.4.0 — Modular class architecture + item system + death mechanics

### Added
- **Modular class architecture** — `gameplay/src/classes/` directory with per-class files
  - Centralized ability dispatchers (`primary_attack`, `secondary_attack`, `cast_ability`, `dash_ability`)
  - `abilities.rs`: `ClassResource` component, `MeleeSwing` hitbox utility, resource regen system
  - `warrior.rs`: Melee cleave, ShieldBlock buff (40% dmg reduction, 3s), Charge, Combat Roll
  - `paladin.rs`: Righteous strike, Holy Light (30% max HP heal), Consecration (AoE field), Divine Steed
  - `rogue.rs`: Backstab, Poison blade DoT, Vanish (guaranteed next crit), Shadowstep
  - `hunter.rs`: Aimed shot, Multi-shot (spread), Snare trap (50% slow), Disengage
  - `mage.rs`: Fireball, Frostbolt, Arcane Blast (high single-target), Blink
- **Item system** (`core/src/items.rs`) — professional-grade foundation
  - `ItemDef` (templates), `ItemInstance` (runtime), `Equipment` (8 slots), `Inventory` (bags)
  - `ItemRarity` (Common→Legendary), `StatMod` system, `ItemCategory`, `EquipSlot`
  - `ItemDatabase` resource for all item definitions
- **Death & respawn system**
  - `PlayerDeathEvent` + `DungeonEndEvent` for proper death flow
  - `DeathPenalty` resource (10% XP loss, 15% gold loss on open-world death)
  - `Graveyard` resource for respawn positioning
- **Multi-character saves** — `PlayerProfiles` resource, persistent to disk

### Changed
- Class abilities now use **real implementations** instead of stubs:
  - ShieldBlock applies a timed buff component (Warrior RMB)
  - Holy Light heals 30% max HP via component (Paladin RMB)
  - Vanish applies buff with guaranteed-crit marker (Rogue Q)
  - Consecration deploys a tick-damage field (Paladin Q)
  - Snare trap slows enemies by 50% (Hunter Q)
- `handle_player_death` in core plugin now fires `PlayerDeathEvent`
- Save directory: `~/.isometric_roguelite/` → `~/.voidforged/`
- Save schema: `SaveData` v2 includes `PlayerProfiles` + `MetaProgression`

### Technical
- Plugin now registers: `tick_shield_block`, `cleanup_shield_block`, `apply_holy_light`
- All 11 crates compiling with 0 errors, 2 minor warnings
- Item system ready for loot tables, equipment UI, crafting

## v0.3.1 — Renamed to Voidforged, docs sync, bug fixes

### Added
- **Official name: Voidforged** — title screen, docs, and metadata updated
- **Full documentation rewrite** — README, ARCHITECTURE.md, DESIGN.md all synced
- **DESIGN.md expanded** with class system design (Warrior/Paladin/Rogue/Hunter/Mage)
- **Architecture doc** includes VFX pipeline, save flow, crate dependency graph

### Fixed
- Several HUD notification systems were defined but never registered (wave announcements, enemy health bars, wave cleared)
- `Local<f32>` cooldowns in secondary attack / cast persist across restart
- Gem magnet uses direct Transform mutation instead of Velocity
- Boss HP scaling produces absurd values at high waves
- Charger AI jitter (random wobble recalculated every frame)
- `invulnerable_until` timestamp type mismatch between dash and death check

## v0.3.0 — GPU particles, shaders, combat polish

### Added
- **GPU particle effects** (bevy_hanabi 0.15) — impact bursts, glow auras, dash trails
- **Custom WGSL shader** (`GlowMaterial`) — time-pulsing emissive glow for telegraph/VFX
- **`EffectsLibrary` resource** — pre-built effect handles built at load time
- **`SpawnImpactEvent`** — event-driven VFX pipeline (cross-crate)
- **Impact bursts** on projectile hit (player + enemy)
- **Screen shake** — camera trauma on player damage, decays over time
- **Floating damage numbers** — 3D text with lifetime, yellow for crits
- **Enemy telegraphing** — 0.3-0.6s windup pause before attack
- **Lifesteal** — heal % of damage dealt
- **Health pickups** — 15-40% drop chance, heal 25 HP
- **Gold pickups** — 10-50% drop chance, +10 gold
- **Equipment system** — 4 slots (weapon/offhand/armor/accessory), applied on run start
- **Expanded CombatStats** — all 12 fields wired into gameplay
- **Crit rolls** — `crit_chance` rolled per hit, `crit_multiplier` applied
- **Armor calculation** — `armor / (armor + 100)` formula, min 1 damage
- **Dash cooldown reduction** — stat now applied
- **HUD** — health bar + HP text, XP bar, wave counter, level, dash CD, damage numbers
- **Dead code removed** — `projectile.rs` deleted

### Changed
- **Player movement** — velocity lerps for acceleration (10x/sec), weightier feel
- **Primary attack** — hold-to-fire on cooldown (Hades-style), mouse-aimed
- **Secondary attack** — right mouse, 3-projectile spread, 0.8s CD
- **Cast ability** — Q key, piercing projectile, 3s CD
- **Dash** — i-frames, dash attack, dash trail VFX ready
- **Loot drops** — XP gems + health + gold with per-variant chances
- **`GameAssets`** — added health/gold pickup mesh + material

### Fixed
- Weapon cooldown now decrements every frame (was stuck at 1 attack per life)
- `cursor_to_world` state-gated to Playing
- Pickup collection uses real player entity (not `Entity::from_raw(0)`)

### Technical
- bevy_hanabi 0.15.1 for GPU particles
- Custom assets/shaders/glow.wgsl for emissive material
- All 8 crates compiling with zero warnings
- Event-driven cross-crate VFX (no circular deps)

## v0.2.0 — Skill-based combat + game loop

### Added
- Mouse aiming, click-to-attack, dash/dodge system
- Enemy melee/ranged attacks, projectile movement, collision
- AttackCooldown component, XP gem collection
- Wave announcements, pause toggle
- Game loop: MainMenu → Playing → GameOver → restart
- Per-run resource reset

### Fixed
- Entity handle placeholders, weapon spawn, projectile velocity
- Deprecated Bevy API migration, 21 warnings eliminated

## v0.1.0 — Initial scaffold

8-crate workspace, core ECS types, isometric camera, 5-variant enemy AI,
wave spawning, XP leveling, networking stubs, server stub.
