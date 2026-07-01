# Isometric Roguelite — Design Document

## Vision
A 3D isometric action roguelite blending **Hades'** skill-based combat,
dodge-focused gameplay, and deep meta-progression with **Vampire Survivors'**
wave-clearing density, built on a modular architecture ready for MMO-style
multiplayer.

**Key pillars:**
1. **Skill-based combat** — mouse aiming, manual attacks, dodge rolling with i-frames
2. **Hades weapon system** — multiple aspects, attack patterns, upgrades mid-run
3. **Meta-progression** — permanent upgrades between runs (dark essence)
4. **Multiplayer** — co-op or competitive, client-server architecture

## Tech Stack
- **Engine:** Bevy 0.15 (ECS, PBR 3D rendering, plugin system)
- **Language:** Rust (2024 edition)
- **Particles:** bevy_hanabi 0.15 (GPU particles)
- **Shaders:** Custom WGSL (glow/telegraph material)
- **Physics:** Manual distance checks (future: Rapier)
- **Networking:** bevy_replicon / lightyear (stubbed)
- **Persistence:** serde-based save files (future: SQLite via sqlx)

### VFX Pipeline
```
Gameplay systems → SpawnImpactEvent → Rendering system → bevy_hanabi EffectAsset
                                        → DamageNumberEvent → Floating 3D text
                                        → ScreenShake resource → Camera offset
```

## Architecture (Workspace Crates)

```
isometric-roguelite/
├── crates/
│   ├── core/           # Components, resources, events, bundles
│   ├── rendering/      # Camera, lighting, assets, HUD, GPU particles, shaders
│   ├── gameplay/       # Player control, enemy AI, combat, projectiles, pickups
│   ├── procedural/     # Wave spawning, loot tables
│   ├── progression/    # XP/leveling, meta-progression stubs
│   ├── network/        # Multiplayer protocol stubs
│   ├── server/         # Headless server stub
│   └── client/         # Main binary — wires all plugins
```

## Core Systems (ECS)

### States
- `AppState` — Loading → MainMenu → Playing → Paused → GameOver
- `RunState` — Entering → Exploring → Combat → RoomTransition → Boss → Victory/Defeat

### Key Components

**Player:**
- `Player` — run-local stats (level, XP)
- `Health` — current/max/i-frames
- `CombatStats` — damage, speed, armor, crit, dodge, lifesteal, dash reduction, etc.
- `Weapon` — kind, damage, attack speed, evolution stage
- `DashCooldown` — dodge roll state machine
- `Equipment` — 4 gear slots with stat modifiers

**Enemies:**
- `Enemy` — variant enum + tier scaling
- `Health` — current/max
- `CombatStats` — move speed, damage
- `AttackCooldown` — per-enemy attack timer + windup (telegraphing)
- `EnemyVariant` — Grunt, Ranged, Charger, Elite, Boss

**Combat:**
- `Projectile` — damage, speed, lifetime, piercing, owner
- `DamageEvent` — target, source, amount, crit, type
- `DeathEvent` — entity, killer, enemy variant
- `DamageNumberEvent` — floating text trigger

**VFX:**
- `EffectsLibrary` (resource) — pre-built particle handles
- `SpawnImpactEvent` — cross-crate VFX trigger
- `ScreenShake` (resource) — camera trauma with decay

### Key Resources
- `PlayerInput` — movement direction, aim direction, action buttons
- `WaveState` — current wave, spawn tracking, difficulty multiplier
- `RunProgression` — per-run stats
- `MetaProgression` — cross-run persistence
- `CursorWorldPos` — mouse position projected into 3D
- `GameAssets` — handles to loaded meshes and materials

## Gameplay Loop

1. **Main menu** → press Enter to start
2. **Waves** of enemies spawn procedurally with scaling difficulty
3. **Skill-based combat** — aim with mouse, hold-click to attack, dodge telegraphed attacks
4. **Defeat enemies** → collect XP gems, health pickups, gold
5. **Level up** → automatic stat bonuses (damage +2, max HP +10 per level)
6. **Clear wave** → next wave with harder enemies
7. **Boss wave** every 10 waves
8. **Death** → GameOver screen → Enter to restart

## Controls

| Input | Action |
|-------|--------|
| WASD / Arrows | Move (smooth acceleration) |
| Mouse | Aim |
| Left click (hold) | Primary attack on cooldown |
| Right click (hold) | Secondary spread attack (0.8s CD) |
| Q (press) | Cast piercing projectile (3s CD) |
| Shift (press) | Dash / dodge roll (1s CD, 0.3s i-frames) |
| Escape (press) | Pause |
| Enter / Space | Confirm menus |

## Enemy Types

| Variant | Behavior | HP | Speed | Attack | Telegraph |
|---------|----------|----|-------|--------|-----------|
| Grunt | Simple chase | 30 | 3.5 | Melee (8 dmg) | 0.3s |
| Ranged | Keep distance, strafe | 20 | 2.5 | Projectile (8 dmg) | 0.4s |
| Charger | Fast charge, wobble | 50 | 7.0 | Charge + melee (15) | 0.5s |
| Elite | Steady advance | 200 | 3.0 | Melee (20 dmg) | 0.3s |
| Boss | Slow relentless | 1000+ | 2.0 | AoE melee (40) | 0.6s |

## Weapon System (Hades-style)

Currently one weapon (MagicMissile) with basic stats. Designed for expansion:

| Weapon | Primary | Special | Cast |
|--------|---------|---------|------|
| MagicMissile | Aimed shot | Spread (3) | Pierce |

## Milestones

- [x] Workspace scaffold (8 crates)
- [x] Core ECS types (components, resources, events, bundles)
- [x] Isometric 3D camera + lighting
- [x] Mouse-based aiming (raycast from camera)
- [x] Hold-to-attack (not click-per-shot)
- [x] Dash/dodge system with i-frames + dash attack
- [x] Secondary attack (spread) + cast (piercing)
- [x] Enemy AI (5 variants) + telegraphing
- [x] Projectile system with bidirectional collision
- [x] Damage pipeline: crits, armor, lifesteal
- [x] Wave spawning with scaling difficulty
- [x] Loot: XP gems, health pickups, gold drops
- [x] Level-up system with stat bonuses
- [x] Equipment system (4 slots, stat modifiers)
- [x] HUD: health, XP, wave, level, dash cooldown
- [x] GPU particles (bevy_hanabi) — impacts, glows, trails
- [x] Custom shader (GlowMaterial) — emissive pulsing
- [x] Screen shake + damage numbers
- [ ] Meta-progression UI (between runs)
- [ ] Weapon evolution system
- [ ] Procedural room generation
- [ ] Sound effects + music
- [ ] 3D models (player, enemies, environment)
- [ ] Multiplayer (networking + dedicated server)
