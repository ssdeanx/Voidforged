# Voidforged — MMO Hades-like in Rust + Bevy 0.15

An **MMO action RPG** blending Hades-style skill-based combat with an open world, procedural dungeons, class-based progression, and persistent online multiplayer.

**Tech Stack:**
- **Client:** Rust + Bevy 0.15 ECS, 2D billboard sprites in 3D isometric
- **Server:** Bevy ECS headless (MinimalPlugins), WebSocket-based protocol skeleton
- **Rendering:** Placeholder procedural quads (colored rectangles/circles + emissive glows), bevy_hanabi GPU particles
- **Database:** SQLite via rusqlite (bundled), Bincode-serialized BLOBs
- **Networking:** Multiplayer feature-gated (`multiplayer` flag), tungstenite + UUID deps

## Repository Structure

```
crates/
├── core/          — Shared types, components, resources, events, items, save DB
├── client/        — Game client binary (Bevy app + all plugins)
├── server/        — Dedicated server binary (headless, MinimalPlugins + gameplay)
├── rendering/     — Isometric camera, lighting, VFX, HUD/UI, placeholder assets
├── gameplay/      — Combat, enemies, classes, abilities, loot, movement, equipment
├── procedural/    — Wave spawning, loot table definitions
├── progression/   — XP/leveling, meta-progression upgrades (Dark Essence)
├── world/         — Zone management, map generation, dungeon entrance detection
├── dungeon/       — Procedural room generation, exits
├── network/       — Protocol definitions, client/server connection manager
└── save/          — Save/load utilities (currently re-exports core DB)
```

## Build Commands

```bash
# Client
cargo run -p ir-client          # Launch game client

# Server (headless)
cargo run -p ir-server          # Start game server (port 9876)

# Checks
cargo check --workspace         # Full compile check (all crates)
cargo clippy --workspace        # Lint all crates
cargo test --workspace          # Run all tests

# Multiplayer feature
cargo check -p ir-network --features multiplayer
```

## Architecture Data Flow

```
Client (Bevy)  ←→  WebSocket (planned)  ←→  Server (BEVY/headless)
                                    │
                                    └─ SQLite (profiles, saves)
```

## ECS Conventions

- Components in `ir_core::components` — shared between client and server.
- Events for cross-system communication (DamageEvent, DeathEvent, etc.).
- Systems in gameplay/rendering crates — registered in their Plugin.
- Systems use run conditions (`.run_if(has_combat)`, `.run_if(can_move)`).
- No `Entity::from_raw(0)` — use `Entity::PLACEHOLDER` or `Option<Entity>`.

## Class System

| Class    | Resource    | Role           | Primary    | Secondary  | Cast          | Dash        |
|----------|-------------|----------------|------------|------------|---------------|-------------|
| Warrior  | Rage        | Melee Tank     | Cleave     | Shield     | Charge        | Roll        |
| Paladin  | Holy Power  | Hybrid Healer  | Strike     | Heal       | Consecration  | Steed       |
| Rogue    | Energy      | Melee DPS      | Backstab   | Poison     | Vanish        | Shadowstep  |
| Hunter   | Focus       | Ranged DPS     | Aimed Shot | Multi Shot | Trap          | Disengage   |
| Mage     | Mana        | Magic DPS      | Fireball   | Frostbolt  | Arcane Blast  | Blink       |

## Item System

Full pipeline: `ItemDef` templates → `ItemInstance` (with rolls) → `Inventory` (slots) → `Equipment` (equipped slots) → `GearScore` (rating) → stat modifiers. Rarity tiers: Common → Uncommon → Rare → Epic → Legendary. Stats include DamageBonus, AttackSpeedBonus, CritChance, Armor, MaxHealth, MoveSpeed, Lifesteal, PickupRadius.

## Meta-Progression

Permanent upgrades between runs using Dark Essence: stat boosts (Vitality, Might, Fortitude, Agility, Precision, Leech), weapon unlocks (Dagger, Bow, Staff), and utility upgrades (Wisdom, Greed, Attraction). Tiered with escalating costs.

## Commit Conventions

- One commit per meaningful change.
- Format: `feat: add X`, `fix: correct Y`, `refactor: extract Z`, `docs: update README`.
- Never commit `.env` files or credentials.
- Keep AGENTS.md synced — it's the agent onboarding document.
