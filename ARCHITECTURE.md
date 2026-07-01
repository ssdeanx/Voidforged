# Isometric Roguelite — Architecture v2.0

## World Structure
```
Open World (ir_world crate)
├── Zones (Grassland, Desert, Forest, Tundra, Swamp)
│   ├── Terrain grid (chunk-based, 16x16 tiles)
│   ├── Points of Interest (dungeon entrances, towns, merchants)
│   └── NPCs (quest givers, shopkeepers)
└── World state (persistent, saved to disk)
    ├── Player position
    ├── Completed dungeons
    ├── Time of day / weather
    └── NPC states

Dungeons (ir_dungeon crate)
├── Procedural room generation (grid-based)
├── Enemy encounter tables per dungeon tier
├── Room types: spawn, combat, treasure, boss, exit
├── Locked doors / keys system
└── Boss room at depth 3/5/7
```

## Player Progression (ir_progression crate overhaul)
```
Stats: STR (melee dmg) | DEX (crit/evade) | INT (magic dmg) | VIT (HP)
Levels: XP curve, +1 stat point per level
Talents: passive tree (3 branches: Combat, Survival, Magic)
Gear: Weapon, Helmet, Chest, Boots, Ring, Amulet
  Each item has: base stats + 0-3 random affixes
Inventory: 24-slot grid, stackable consumables (potions, scrolls)
```

## Save System (ir_save crate)
```
- Player stats, inventory, equipment
- World state (completed content, NPC affinity)
- Settings (keybinds, audio)
- Save on: zone transition, dungeon exit, manual
- Format: bincode (fast binary serialization)
```

## Combat (keep + refactor)
```
Same ECS pipeline but stats now feed from real character stats.
Enemy scaling: Dungeon tier × enemy level × variant multiplier.
Loot drops: Based on enemy type + dungeon tier + luck stat.
```

## Crate Dependency Graph
```
client
├── rendering ── core
│   ├── hud
│   ├── camera
│   └── assets
├── gameplay ── core, world
│   ├── player
│   ├── combat
│   └── enemies
├── world ── core, rendering
│   ├── map
│   ├── zones
│   └── generation
├── dungeon ── core, world
│   ├── rooms
│   ├── encounters
│   └── rewards
├── progression ── core, save
│   ├── stats
│   ├── talents
│   └── gear
└── save ── core
```
