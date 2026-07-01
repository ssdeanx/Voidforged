# Voidforged

A 3D isometric **MMO action RPG** blending Hades-style skill-based combat
with an open world, procedural dungeons, 5 playable classes, and deep gear progression.

**Tech:** Rust + Bevy 0.15 ECS · PBR 3D rendering · bevy_hanabi GPU particles · PostgreSQL (server)

## Status — v0.5.0

Core gameplay systems are feature-complete:
- 5 classes with full ability sets (Warrior, Paladin, Rogue, Hunter, Mage)
- Hitbox-based melee combat with shape detection (cone, circle, rect)
- Projectile combat for ranged classes (Hunters, Mages)
- Damage pipeline with armor, dodge, crit, lifesteal, armor pen, damage reduction
- Loot tables with weighted item drops per enemy variant
- Gear score / item level system (stat weights, rarity budgets)
- SQLite save database (future: server-side PostgreSQL for MMO)
- WoW-style HUD (action bars, unit frames, enemy nameplates)
- Stamina-based sprint + dodge system
- Equipment events (equip/unequip with stat application)
- Player death & respawn with graveyard/dungeon-end mechanics

## Quick Start

```bash
# Client only (dev mode, no server needed for single-player testing)
cargo run -p ir-client

# Server (for MMO)
cargo run -p ir-server
```

### Controls

| Input | Action |
|-------|--------|
| WASD / Arrows | Move (camera-relative) |
| Mouse | Aim (cursor-to-world raycast) |
| Left click (hold) | Primary attack |
| Right click (hold) | Secondary attack |
| Q | Cast ability |
| Shift | Dodge roll (i-frames + stamina cost) |
| Shift (hold while moving) | Sprint (stamina drain) |
| F | Interact (enter dungeons, pick up items) |
| Escape | Pause |
| I | Inventory |

## What's Implemented

### Combat & Abilities
- 5-class system with per-class resources (Rage, Holy Power, Energy, Focus, Mana)
- Hitbox-based melee (Cone shape detection for Warrior/Paladin/Rogue)
- Projectile combat for ranged classes (Hunter/Mage)
- Damage pipeline: armor mitigation, dodge chance, crit rolls, lifesteal, armor pen
- Shield Block buff (40% dmg reduction), Consecration AoE field, Poison DoT stacking
- Vanish (next attack crit), Holy Light (heal), Snare Traps
- Melee + ranged enemy attacks with telegraphing (windup)
- Damage numbers, screen shake, impact particles

### Classes
| Class | Resource | Role | Primary | Secondary | Cast | Dash |
|-------|----------|------|---------|-----------|------|------|
| Warrior | Rage | Melee Tank | Cleave (cone) | Shield Block | Charge | Combat Roll |
| Paladin | Holy Power | Hybrid Healer | Righteous Strike | Holy Light | Consecration | Divine Steed |
| Rogue | Energy | Melee DPS | Backstab (precise) | Poison Blade | Vanish | Shadowstep |
| Hunter | Focus | Ranged DPS | Aimed Shot | Multi-Shot | Snare Trap | Disengage |
| Mage | Mana | Magic DPS | Fireball | Frostbolt | Arcane Blast | Blink |

### Items & Loot
- 19 starter items: 8 weapons, 7 armor pieces, 2 accessories, 1 consumable
- 8 equipment slots (weapon, offhand, helmet, chest, boots, ring, amulet, trinket)
- Gear score calculation: stat weight × rarity budget × slot base
- Loot tables per enemy level range (weighted drops, no RNG)
- Inventory with slot management + gold
- Equipment events (equip/unequip with stat swap)

### Movement
- Smooth camera-relative WASD movement
- Sprint (Shift + move, stamina cost 20/s)
- Dodge roll (Shift tap, stamina cost 25, 0.25s i-frames)
- Stamina regen (15/s base)

### HUD
- Player unit frame (portrait, health bar, resource bar)
- Target frame (enemy name + health % + level)
- Action bar (6 ability buttons with keybinds)
- XP bar, level, gold, wave counter, dash status
- Enemy health bars (3D world-space nameplates)
- Damage numbers (floating text, yellow for crits)

### Open World
- 3 zones (Grasslands, Desert, Forest) with distinct tile colors
- Procedural environment decorations (rocks, bushes, grass)
- Dungeon entrance markers
- Zone tracking in HUD

### Dungeons
- 3×3 procedural room grid with walls, corridors, doorways
- Enemy spawns in non-entrance rooms
- Green exit marker to return to world

### Characters & Save (Client)
- SQLite database with WAL mode (~/.local/share/voidforged/saves.db)
- Bincode-serialized profiles
- Auto-save every 30s, save on quit
- 5-character save slots
- Character create/delete/select UI

### Visual FX
- GPU particle bursts on hit (bevy_hanabi)
- Floating damage numbers
- Screen shake on player damage
- Custom emissive WGSL shader (GlowMaterial)
- Billboard sprites (Hades-style 2D-in-3D)

## Workspace Crates

| Crate | Role |
|-------|------|
| `core` | Components, resources, events, items, DB, hitbox types |
| `gameplay` | Classes, combat, enemies, loot, equipment, movement, player death |
| `rendering` | Camera, lighting, HUD, VFX, custom shaders, asset loading |
| `procedural` | Dungeon generation, terrain decoration, legacy loot |
| `progression` | XP/leveling, stat scaling |
| `world` | Open world zones, map gen, dungeon entrances |
| `dungeon` | Room grid, encounter spawning |
| `save` | (Planned) Server-side save orchestration |
| `network` | Protocol definitions, packet schemas |
| `server` | Game server binary (Tokio/Axum + PostgreSQL) |
| `client` | Main binary — wires all plugins |

## MMO Architecture

```
Client (Bevy) ← WebSocket → Server (Tokio/Axum)
                                 │
                           PostgreSQL
                          (characters, inventory,
                           world state, auth)
```

- **Client** is a thin renderer + input collector + prediction layer
- **Server** authorizes all state changes (movement, combat, loot)
- **Database** is server-only — never accessed directly from client
- Single-player development mode uses SQLite (client-side, for testing)

## In Progress / Planned

- [ ] Server-side PostgreSQL backend (sqlx + connection pooling)
- [ ] WebSocket networking protocol
- [ ] Client-server auth (JWT sessions)
- [ ] Multiplayer open world (shared zones)
- [ ] Co-op dungeon instances
- [ ] Talent trees (3 per class)
- [ ] Meta-progression system
- [ ] Weapon evolution paths
- [ ] Boss encounter mechanics (phases, telegraph patterns)
- [ ] Sound effects + music
- [ ] Animated 3D character sprites
- [ ] More zones (Swamp, Tundra)
- [ ] Group finder / party system
