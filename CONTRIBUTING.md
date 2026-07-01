# Contributing to Voidforged

Thank you for your interest in contributing! This document covers the development
workflow, coding conventions, and review process for the Voidforged project.

## Getting Started

### Prerequisites

- **Rust** 1.84+ (edition 2021) — install via [rustup](https://rustup.rs/)
- **Cargo** — included with Rust
- **Linux** (CachyOS Arch Linux primary target; other distros likely work)
- No external C libraries required (rusqlite uses bundled SQLite)

### Build & Run

```bash
# Client (single-player dev mode)
cargo run -p ir-client

# Server (for MMO testing)
cargo run -p ir-server

# Full workspace check
cargo check --workspace
cargo clippy --workspace
cargo test --workspace
```

### Recommended Tools

- `cargo-deny` — license/advisory checking before PR
- `cargo-outdated` — dependency update review
- `cargo-audit` — security vulnerability scanning
- `cargo-expand` — macro expansion debugging
- `rust-analyzer` — IDE support (VS Code, Helix, Neovim)

## Project Structure

The workspace contains 11 crates under `crates/`:

| Crate | Package | Purpose |
|---|---|---|
| `core` | `ir-core` | Components, resources, events, items, DB, shared types |
| `client` | `ir-client` | Main binary — wires all plugins |
| `server` | `ir-server` | Headless game server (Tokio/Axum + PostgreSQL) |
| `rendering` | `ir-rendering` | 3D camera, HUD, VFX, WGSL shaders, asset loading |
| `gameplay` | `ir-gameplay` | Combat, enemies, classes, loot, movement |
| `world` | `ir-world` | Open world zones, map gen, dungeon entrances |
| `dungeon` | `ir-dungeon` | Procedural room grid generation |
| `procedural` | `ir-procedural` | Wave spawning, difficulty scaling, loot tables |
| `progression` | `ir-progression` | XP/leveling, stat scaling, meta-upgrades |
| `save` | `ir-save` | Persistent save/load (bincode) |
| `network` | `ir-network` | Protocol types (stubbed for future multiplayer) |

See [ARCHITECTURE.md](ARCHITECTURE.md) for the full dependency graph.

## Code Conventions

### Rust Style

- Follow `rustfmt` defaults — run `cargo fmt` before committing
- All public items **must** have doc comments (`///` or `//!`)
- No `unwrap()` or `expect()` in production code — use `Result` + `?` or proper error handling
- No `#[allow(...)]` without a comment explaining why
- Use `Entity::PLACEHOLDER` or `Option<Entity>` instead of `Entity::from_raw(0)`
- Prefer `thiserror` for error types

### ECS Patterns (Bevy)

- Components and resources go in `ir_core` (shared between client and server)
- Systems live in their functional crate (gameplay, rendering, etc.)
- Cross-system communication uses Bevy events (e.g., `DamageEvent`, `DeathEvent`)
- Systems use run conditions (`.run_if(in_state(AppState::Playing))`)
- Keep systems small and focused — one responsibility per system

### Git & Commits

- One commit per logical change
- Commit messages follow conventional commits:
  - `feat:` new feature
  - `fix:` bug fix
  - `refactor:` code change with no functional difference
  - `docs:` documentation only
  - `chore:` build/config/tooling
- Branch names: `feat/short-description`, `fix/short-description`
- Never commit `.env` files, credentials, or large binaries
- Squash fixup commits before merging

### Documentation

Every public API item must have a doc comment:

```rust
/// A brief summary of what this item is or does.
///
/// Longer explanation goes here. Use code blocks for examples.
pub fn important_function() -> Result<()> {
    // ...
}
```

Run `cargo doc --no-deps --open` to preview the generated documentation.
We target zero `missing_docs` warnings on the full workspace.

## Pull Request Process

1. Create a feature branch from `main`
2. Make your changes, keeping commits clean and well-described
3. Run `cargo check --workspace` and `cargo clippy --workspace` — zero warnings
4. Run `cargo test --workspace` — all tests pass
5. Create a PR with a clear description of what changed and why
6. Request review from a maintainer
7. Address review feedback with fixup commits

## Testing

- Unit tests go in a `#[cfg(test)] mod tests { ... }` block at the bottom of the source file
- Integration tests go in `tests/` within each crate
- Bevy ECS tests can use `App::new()` with minimal plugin sets to test systems in isolation
- The project currently has limited test coverage — contributions to test infrastructure are especially welcome

## Security

- The client is a thin renderer — server validates all state changes
- No secrets in client binary (API keys, DB credentials are server-only)
- Never trust client input — validate on the server side
- Rate-limit connections per IP at the network layer

## Questions?

Open a GitHub issue or reach out to the maintainers. For architecture decisions,
check [ARCHITECTURE.md](ARCHITECTURE.md) and [DESIGN.md](DESIGN.md) first.
