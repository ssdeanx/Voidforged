# Voidforged — MMO Hades-like in Rust + Bevy 0.15

An **MMO action RPG** blending Hades-style skill-based combat with an open world, procedural dungeons, class-based progression, and persistent online multiplayer.

**Tech Stack:**
- **Client/Server:** Rust + Bevy 0.15 ECS (client), Tokio + Axum + sqlx (server)
- **Rendering:** PBR 3D, bevy_hanabi GPU particles, WGSL shaders
- **Database:** PostgreSQL (server-side, sqlx with connection pooling)
- **Networking:** WebSocket-based deterministic lockstep with authority
- **CI/CD:** GitHub Actions

## Repository Structure

```
crates/
├── core/          — Shared types, components, resources, events (no game logic)
├── client/        — Game client binary (Bevy app + rendering + UI)
├── server/        — Game server binary (auth, world state, DB, networking)
├── rendering/     — 3D isometric camera, lighting, VFX, HUD/UI
├── gameplay/      — Combat, enemies, classes, abilities, loot, movement
├── procedural/    — Dungeon/terrain procedural generation
├── progression/   — Character stats, leveling, talent trees
├── world/         — Zone management, spawners, NPC placement
├── dungeon/       — Instance-based dungeon logic
├── network/       — Protocol definitions, packet serialization
└── save/          — Server-side save/load and migration tools
```

## Build Commands

```bash
# Client
cargo run -p ir-client          # Launch game client
cargo run -p ir-client -- --server 127.0.0.1:9876  # Connect to custom server

# Server
cargo run -p ir-server          # Start game server (port 9876)

# Checks
cargo check --workspace         # Full compile check (all crates)
cargo clippy --workspace        # Lint all crates
cargo test --workspace          # Run all tests
```

## Architecture Data Flow

```
Client (Bevy)  ←→  WebSocket  ←→  Server (Tokio/Axum)
                                    │
                                    ├─ PostgreSQL (characters, inventory, world state)
                                    ├─ Redis (sessions, leaderboards, world cache)
                                    └─ S3 (asset store — not yet implemented)
```

## Security Baseline

- **Never trust client.** Server validates every action (position, damage, loot).
- Authentication via token (JWT or session key) on connect.
- Server authorizes: movement, combat rolls, loot acquisition, stat changes.
- Client is a thin renderer + input collector + prediction layer.
- No secrets in client binary. Asset keys, DB credentials live on server only.
- Rate-limit connections per IP.

## Entity Component System (ECS) Conventions

- Components in `ir_core::components` — shared between client and server.
- Events for cross-system communication (DamageEvent, DeathEvent, etc.).
- Systems in gameplay/rendering crates — registered in their Plugin.
- Systems use run conditions (`.run_if(has_combat)`, `.run_if(can_move)`).
- No `Entity::from_raw(0)` — use `Entity::PLACEHOLDER` or `Option<Entity>`.

## Class System

| Class    | Resource    | Role           | Primary | Secondary | Cast         | Dash        |
|----------|-------------|----------------|---------|-----------|-------------|-------------|
| Warrior  | Rage        | Melee Tank     | Cleave  | Shield    | Charge      | Roll        |
| Paladin  | Holy Power  | Hybrid Healer  | Strike  | Heal      | Consecration| Steed       |
| Rogue    | Energy      | Melee DPS      | Backstab| Poison    | Vanish      | Shadowstep  |
| Hunter   | Focus       | Ranged DPS     | Aimed   | Multi     | Trap        | Disengage   |
| Mage     | Mana        | Magic DPS      | Fireball| Frostbolt | Arcane Blast| Blink      |

## Commit Conventions

- One commit per meaningful change.
- Format: `feat: add X`, `fix: correct Y`, `refactor: extract Z`, `docs: update README`.
- Never commit `.env` files or credentials.
- Keep AGENTS.md synced — it's the agent onboarding document.

## Engine Guidance

This project uses **Hermes Agent** with Bevy's ECS paradigm.
- Complex multi-file refactors → delegate_task or work directly with Hermes.
- Targeted single-file fixes → patch directly.
- UI exploration before coding → sketch or design-md.
- Not sure about architecture → check AGENTS.md and the crates involved.
