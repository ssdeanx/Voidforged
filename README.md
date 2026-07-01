# Isometric Roguelite

A 3D isometric action roguelite blending **Hades-style** skill-based combat with
**Vampire Survivors'** wave-clearing density, built on a modular architecture
ready for MMO-style multiplayer.

**Tech:** Rust + Bevy 0.15 ECS, PBR 3D rendering, bevy_hanabi GPU particles,
custom WGSL shaders, deterministic networking stubs.

## Status — v0.3.0

Core game loop is functional with GPU particle effects, custom shaders,
and full combat polish.

## Quick Start

```bash
cargo run -p ir-client
```

### Controls

| Input | Action |
|-------|--------|
| WASD / Arrows | Move (smooth acceleration) |
| Mouse | Aim |
| Left click (hold) | Primary attack |
| Right click (hold) | Secondary attack (spread) |
| Q | Cast (piercing) |
| Shift | Dash / dodge roll (i-frames + dash attack) |
| Escape | Pause |
| Enter / Space | Confirm (menus) |

## What's Implemented

### Combat
- Mouse-aimed attacks, hold-to-fire on cooldown
- Dash with i-frames, directional or cursor-toward
- Secondary attack (3-spread), cast ability (piercing)
- 5 enemy variants with distinct AI + telegraphing
- Melee + ranged enemy attacks
- Projectile system with collision (player + enemy)
- Crit rolls, armor calculation, lifesteal

### Visual FX
- GPU particle bursts on hit (bevy_hanabi)
- Floating damage numbers (yellow for crits)
- Screen shake on player damage
- Custom emissive shader ready for telegraph/VFX

### Loot & Progression
- XP gems with magnet + collection detection
- Health pickups, gold drops (per-variant chances)
- Level-up system with stat bonuses
- Equipment system (4 slots, applied on run start)
- Expanded CombatStats (12 fields)

### Systems
- Main menu → Playing → GameOver → restart loop
- Pause toggle
- Wave spawning with scaling difficulty
- HUD: health bar + text, XP bar, wave counter, level, dash CD
- Damage numbers floating in 3D world

## Architecture (Workspace Crates)

| Crate | Role |
|-------|------|
| `core` | Components, resources, events, bundles |
| `rendering` | Camera, lighting, assets, HUD, GPU particles, custom shaders |
| `gameplay` | Player control, enemy AI, combat, projectiles, pickups |
| `procedural` | Wave spawning, loot tables |
| `progression` | XP/leveling, meta-progression stubs |
| `network` | Multiplayer protocol stubs |
| `server` | Headless server stub |
| `client` | Main binary — wires all plugins |

## Dependencies

- **bevy 0.15** — engine
- **bevy_hanabi 0.15** — GPU particles
- **rand** — RNG for crits, loot, AI
- **serde** — save/load stubs

## Milestones

- [x] Workspace scaffold (8 crates)
- [x] Core ECS types
- [x] Isometric 3D camera + lighting
- [x] Player movement + mouse-aimed attacks + dash
- [x] Enemy AI (5 variants) + telegraphing
- [x] Projectile system + collision (player + enemy)
- [x] Damage/Death event pipeline with crits + armor
- [x] Wave spawning with scaling
- [x] XP/health/gold loot drops + collection
- [x] Level-up system
- [x] Enemy attacks (melee + ranged)
- [x] Game loop: MainMenu → Playing → GameOver → restart
- [x] HUD (health, XP, wave, level, dash CD)
- [x] GPU particles + custom shaders
- [x] Screen shake + damage numbers
- [ ] Meta-progression UI
- [ ] Weapon evolution system
- [ ] Procedural room generation
- [ ] Sound effects + music
- [ ] Multiplayer (networking)
