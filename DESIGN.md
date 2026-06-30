# Isometric Roguelite — Design Document

## Vision
A 3D isometric action roguelite blending **Vampire Survivors'** auto-attack
bullet-heaven gameplay with **Hades'** quality-of-execution and deep
meta-progression, built on a modular architecture ready for MMO-style
multiplayer.

## Tech Stack
- **Engine:** Bevy 0.15 (ECS, PBR 3D rendering, plugin system)
- **Language:** Rust (2024 edition)
- **Physics:** Bevy's built-in (future: Rapier)
- **Networking:** Stubbed for bevy_replicon / lightyear
- **Persistence:** serde-based save files (future: SQLite via sqlx)

## Architecture (Workspace Crates)

```
isometric-roguelite/
├── crates/
│   ├── core/           # Components, resources, events, bundles
│   ├── rendering/      # Isometric camera, lighting, asset pipeline
│   ├── gameplay/       # Player control, enemy AI, combat, projectiles
│   ├── procedural/     # Wave spawning, loot tables, map generation
│   ├── progression/    # XP/leveling, meta-progression, unlocks
│   ├── network/        # Multiplayer protocol (stubbed)
│   ├── server/         # Headless server binary
│   └── client/         # Main binary — wires all plugins together
```

## Core Systems (ECS)

### States
- `AppState` — Loading → MainMenu → Playing → Paused → GameOver
- `RunState` — Entering → Exploring → Combat → RoomTransition → Boss → Victory/Defeat

### Key Components
- `Player` — run-local stats (level, XP)
- `Enemy` — variant enum + tier scaling
- `Health` — current/max/invulnerability window
- `CombatStats` — damage, speed, armor, crit
- `Weapon` — type, damage, attack speed, evolution stage
- `Projectile` — damage, speed, lifetime, piercing
- `ExperienceGem` — XP value, magnet attraction
- `Team` — Player/Enemy/Neutral for friendly fire

### Key Resources
- `WaveState` — current wave, spawn tracking, difficulty multiplier
- `RunProgression` — per-run stats (kills, damage, gold)
- `MetaProgression` — cross-run persistence (essence, unlocks, upgrades)
- `PlayerInput` — movement direction, action buttons
- `GameAssets` — handles to loaded meshes and materials

## Gameplay Loop

1. **Enter zone** → waves of enemies spawn procedurally
2. **Auto-combat** — player weapons fire at nearest enemy automatically
3. **Defeat enemies** → collect XP gems and loot
4. **Level up** → increase stats, choose upgrades
5. **Clear wave** → next wave with harder enemies
6. **Boss wave** every 10 waves
7. **Death** → spend meta-currency on permanent upgrades
8. **Repeat** — stronger each run

## Enemy Types
| Variant | Behavior | HP | Speed |
|---------|----------|----|-------|
| Grunt | Simple chase | 30 | 3.5 |
| Ranged | Keep distance, strafe | 20 | 2.5 |
| Charger | Fast charge, slight wobble | 50 | 7.0 |
| Elite | Steady advance, variable speed | 200 | 3.0 |
| Boss | Slow relentless advance | 1000+ | 2.0 |

## Milestones
- [x] Workspace scaffold (8 crates)
- [x] Core ECS types (components, resources, events, bundles)
- [x] Isometric 3D camera + lighting
- [x] Player movement + auto-attack
- [x] Enemy AI (5 variant behaviors)
- [x] Projectile system with collision
- [x] Damage/Death event pipeline
- [x] Wave spawning with scaling difficulty
- [x] XP gem magnet + pickup
- [x] Level-up system
- [ ] Meta-progression UI
- [ ] 3D models (player, enemies, environment)
- [ ] Weapon evolution system
- [ ] Procedural room generation
- [ ] HUD/UI overlay
- [ ] Sound effects + music
- [ ] Multiplayer (networking crate)
- [ ] Dedicated server
