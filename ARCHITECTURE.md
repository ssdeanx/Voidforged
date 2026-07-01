# Voidforged — Architecture v2.0

## World Structure
```
Open World (ir_world crate)
├── Zones (Grasslands, Desert, Forest)
│   ├── Tile grid (2-unit spacing, 20×20 tiles per zone)
│   ├── Environment decorations (rocks, bushes, grass)
│   └── Dungeon entrances (walk near + press F)
└── World state (transient per session)
    ├── Player position
    ├── Current zone tracking
    └── Dungeon instance state

Dungeons (ir_dungeon crate)
├── 3×3 room grid (procedural, 5×5 tiles per room)
├── Room types: entrance, combat, exit (boss room)
├── Enemy encounter scaling (tier × multiplier)
├── Corridors with doorways between rooms
└── Exit marker → return to world
```

## Class System (PLANNED)
```
Warrior   — Melee tank, charge, shield block, whirlwind
Paladin   — Hybrid tank/heal, holy light, smite, consecration
Rogue     — Stealth, backstab, poisons, eviscerate
Hunter    — Ranged DPS, pet, traps, aimed shot
Mage      — Ranged magic, fire/frost/arcane, polymorph

Each class has:
- Unique starting weapon and abilities (primary/secondary/cast/dash)
- 3-branch talent tree (15+ nodes each)
- Class-specific gear sets
- Unique playstyle (resource mechanic: rage/mana/energy/focus)
```

## Combat Pipeline (ECS)
```
PlayerInput resource
  ↓
read_player_input (keyboard + mouse → PlayerInput)
  ↓
player_movement (camera-relative velocity lerp)
player_dash (i-frame burst + dash attack)
player_attack (hold-to-fire → ProjectileBundle)
  ↓
move_projectiles (lifetime + velocity)
projectile_hit (player→enemy collision)
projectile_hit_player (enemy→player collision)
  ↓
DamageEvent → apply_damage (crit, armor, lifesteal)
  ↓
DeathEvent → handle_death (spawn_loot + despawn)
  ↓
spawn_impact_effect (GPU particles)
DamageNumberEvent (floating 3D text)
```

## VFX Pipeline
```
Gameplay systems → SpawnImpactEvent → Rendering → bevy_hanabi EffectAsset
                                        → DamageNumberEvent → Floating 3D text
                                        → ScreenShake resource → Camera offset
```

## Crate Dependency Graph
```
client
├── rendering ── core
│   ├── hud (layout, updates, notifications, menus)
│   ├── camera (isometric ortho, follow, shake)
│   ├── assets (placeholder sprite generation)
│   └── effects (GPU particles, GlowMaterial shader)
├── gameplay ── core, world
│   ├── player (input, movement, attack, dash, cast)
│   ├── combat (projectiles, damage, death)
│   ├── enemy (AI, melee, ranged, telegraphing)
│   └── pickup (gem magnet, health, gold, collection)
├── world ── core
│   ├── map (tile generation, zone placement, enemies)
│   └── zone (ZoneId, ZoneDef, dungeon entrances)
├── dungeon ── core
│   ├── rooms (grid generation, walls, exits)
│   └── plugin (enter/exit transitions)
├── procedural ── core
│   ├── waves (spawning, scaling, variant selection)
│   └── loot (xp, health, gold per enemy variant)
├── progression ── core
│   ├── leveling (xp gain, level-up bonuses)
│   └── upgrades (stub — meta-progression tree)
├── save ── core
│   ├── serialization (bincode SaveData)
│   └── autosave/autoload (state-gated)
├── network ── core (stub — protocol types only)
└── server ── core (stub — headless skeleton)
```

## Data Flow — Save System
```
PendingSave (resource flag)
    ↓
autosave (on GameOver/MainMenu)
    ↓
build_save_data (RunProgression + Player → SaveData)
    ↓
bincode::serialize → write to ~/.isometric_roguelite/save.dat
    ↑
autoload (on MainMenu enter)
    ↓
bincode::deserialize → SaveState resource
```

## Networking (PLANNED)
```
Client-Server architecture:
- Server: authoritative game simulation (headless)
- Client: rendering + input → server
- Protocol: bevy_replicon or lightyear
- Replication: ECS component sync (position, health, combat state)
```

## Performance Targets
- 60 FPS on mid-range desktop (CachyOS Arch Linux tested)
- 100+ enemies on screen with GPU particles
- < 100ms network tick for multiplayer
- Minimal heap allocation during gameplay (ECS-oriented)
