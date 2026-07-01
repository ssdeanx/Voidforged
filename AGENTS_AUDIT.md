# Enterprise-Grade Quality Audit: Network, Server, Save Crates

Audit date: 2026-07-01
Auditor: Hermes Agent

---

## Network Crate: 2/10

### What's here
| File | Lines | Content |
|------|-------|---------|
| `src/protocol.rs` | 163 | Well-designed message types (14 variants) with serde JSON serialization |
| `src/plugin.rs` | 10 | **Empty Bevy Plugin** — just `// TODO` |
| `src/client.rs` | 2 | `pub struct NetworkClient;` — **empty stub** |
| `src/server.rs` | 2 | `pub struct NetworkServer;` — **empty stub** |
| `src/lib.rs` | 31 | Module declarations with **3 broken imports** |

### Scoring breakdown

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| **Compiles** | 0/1 | 3 compilation errors with `multiplayer` flag, 1 without it. Broken `pub use` re-exports (`IsMultiplayer`, `ClientConfig`, `ConnectionState`, `ClientConnection`, `ClientId`, `Room`, `RoomId`, `SessionId` — none exist) |
| **Protocol design** | 1/1 | Actually decent. NetworkMessage envelope with SequenceNo, 14 MessagePayload variants covering handshake, input, world sync, chat, rooms, disconnect. Strong typing, serde JSON, well-documented. |
| **Client implementation** | 0/1 | Zero. Unit struct. No connection management, no state machine, no WebSocket init, no reconnect logic, no ping/pong handling. |
| **Server implementation** | 0/1 | Zero. Unit struct. No connection acceptor, no session management, no room/instance logic, no replication. |
| **Plugin registration** | 0/1 | Plugin is a no-op. Registers zero systems, zero resources. Doesn't even init the `NetworkClient`/`NetworkServer` resources. |
| **Testing** | 0/1 | Zero tests. No unit tests on protocol messages, no integration tests. |
| **Error handling** | 0.5/1 | Protocol types use proper `Option`/`Result` in payload fields (e.g., `target: Option<AbilityTarget>`). But no actual connection error handling exists. |
| **Documentation** | 0.5/1 | Module-level docs are good (architecture overview + feature flags). Protocol types are undocumented. |
| **Production readiness** | 0/1 | Feature-gated code behind `multiplayer` flag exists but is entirely stubs. Not usable in any form. |
| **Security** | 0/1 | No rate limiting, no auth beyond a bare token field, no input validation, no message size limits, no DoS protection. |

### Critical issues
1. **Does not compile** — 3 errors with `multiplayer` feature enabled
2. **Zero network transport code** — no tungstenite usage despite it being a dependency
3. **Plugin is a no-op** — nothing happens
4. `lib.rs` re-exports types that don't exist in their source modules

### What's salvageable
The protocol.rs is genuinely well-structured. The message types, sequencing, and variant coverage are appropriate for an MMO action RPG. Protocol alone is ~1.5/10 quality; everything else drags it down.

---

## Server Crate: 2/10

### What's here
| File | Lines | Content |
|------|-------|---------|
| `src/lib.rs` | 6 | Module declarations + re-export of `ServerPlugin` |
| `src/plugin.rs` | 9 | **Empty Bevy Plugin** — just `// TODO` |
| `src/server_app.rs` | 8 | Creates `App` with `MinimalPlugins + CorePlugin + GameplayPlugin` |

### Scoring breakdown

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| **Compiles** | 0/1 | Fails transitively due to ir-network crate errors |
| **Server architecture** | 0.5/1 | Has a `build_server_app()` function that constructs a headless `App`. Correct use of `MinimalPlugins`. But no server-specific resources or systems. |
| **Network integration** | 0/1 | No network listener, no connection acceptor, no WebSocket setup. `ir-network` is a dependency but never used. |
| **Game simulation** | 0.5/1 | Relies entirely on `CorePlugin` + `GameplayPlugin`. No server-specific simulation logic (authoritative movement, combat, AI). |
| **Plugin registration** | 0/1 | `ServerPlugin` is empty. |
| **Testing** | 0/1 | Zero tests. |
| **Error handling** | 0.5/1 | No server-specific error handling exists. |
| **Documentation** | 0.5/1 | Minimal but correct. |
| **Production readiness** | 0/1 | Not runnable as a dedicated server. No TCP/WebSocket listener. No tick loop. No player management. |
| **Security** | 0/1 | Nothing to evaluate — no server logic at all. |

### Critical issues
1. **Does not compile** — transitive via ir-network
2. `build_server_app()` doesn't even add `ServerPlugin` — the server has no server-specific behavior
3. No network listener, no rooms, no tick loop
4. Plugin is a no-op stub

---

## Save Crate: 4/10

### What's here
| File | Lines | Content |
|------|-------|---------|
| `src/lib.rs` | 180 | Full save/load system: types, save/load functions, SavePlugin with autosave/autoload/mark_pending |

### Scoring breakdown

| Criterion | Score | Rationale |
|-----------|-------|-----------|
| **Compiles** | 1/1 | Compiles cleanly. |
| **Architecture** | 0.5/1 | Reasonable types (`PlayerSaveData`, `SaveData`, `SaveState`, `PendingSave`). Bincode serialization is appropriate. But **is entirely dead code** — `SavePlugin` is never registered by any app builder. |
| **Error handling** | 0.5/1 | `save()` returns `bool`, `load()` returns `Option`, errors logged. But **no atomic writes** — a crash mid-write corrupts the file. No write-ahead log or temp-file + rename pattern. |
| **Data integrity** | 0/1 | Direct `std::fs::write` (non-atomic). Crash during write = corrupted file. No checksums, no version migration beyond `version` field, no backup mechanism. |
| **Integration** | 0/1 | **REDUNDANT with `core/src/db.rs`**, which is the actual active save system (SQLite with WAL, auto-save every 30s, save-on-quit). `SavePlugin` is never added to any App. This crate is compile-only dead code. |
| **Testing** | 0/1 | Zero tests. No serialization round-trip tests, no file I/O tests, no corruption recovery tests. |
| **Documentation** | 0.5/1 | Module docs good. Function-level docs absent. |
| **Save strategy** | 0.5/1 | Bincode is faster than JSON but fragile — deserialization breaks on struct changes. No forward/backward compatibility. |
| **Platform compliance** | 0.5/1 | Uses `$HOME/.voidforged/` (cross-platform `HOME` env). But this conflicts with `core/src/db.rs` which uses XDG data dir (`~/.local/share/voidforged/`). |
| **Production readiness** | 0/1 | Dead code. Even if live, would lack atomic writes, integrity checks, format versioning, and migration. |

### Critical issues
1. **Dead code** — `SavePlugin` is defined but never registered. The `core/src/db.rs` SQLite system handles all actual saves.
2. **No atomic writes** — `std::fs::write` will produce a corrupted save file if power/memory is lost mid-write
3. **No tests** — at minimum a round-trip serialize/deserialize test should exist
4. **Path inconsistency** — saves to `~/.voidforged/save.dat` while `core/src/db.rs` saves to `~/.local/share/voidforged/saves.db`
5. **Brittle format** — bincode is version-sensitive with no migration path

### Comparison with `core/src/db.rs` (SQLite)

| Concern | `save/src/lib.rs` | `core/src/db.rs` | Winner |
|---------|-------------------|-------------------|--------|
| Compiles | ✅ | ✅ | Tie |
| Is used | ❌ (dead code) | ✅ (`CorePlugin` registers it) | core/db |
| Atomicity | ❌ (direct write) | ✅ (WAL mode) | core/db |
| Query capability | ❌ (whole file) | ✅ (SQL queries, `list_profiles`, `delete_profile`) | core/db |
| Migration | ❌ (none) | ✅ (PRAGMA user_version) | core/db |
| Schema | ❌ (bincode blob) | ✅ (relational columns + blob) | core/db |
| Error handling | `bool`/`Option` | `Result<_, Box<dyn Error>>` | core/db |
| Multi-character | ✅ (via SaveData.profiles) | ✅ (per-profile rows) | Tie |

`core/src/db.rs` is strictly superior in every dimension. The save crate should be either removed or refactored into a wrapper around the SQLite DB.

---

## OVERALL: 3/10

### Composite score

| Crate | Score | Weight | Contribution |
|-------|-------|--------|-------------|
| Network | 2/10 | 40% | 0.8 |
| Server | 2/10 | 40% | 0.8 |
| Save | 4/10 | 20% | 0.8 |
| **Overall** | **2.4/10** | | **3/10** (rounded) |

For an "enterprise-grade" MMO project, these three crates are **pre-alpha stubs**. The protocol design shows some forethought, and the SQLite save system in `core` is solid, but the crates under audit are largely empty, broken, or dead.

### What's good
- **Protocol message types** are well-designed (14 variants, proper serde, typed fields)
- **Server architecture intent** is correct (headless `MinimalPlugins`, `build_server_app()`)
- **Save types** (`PlayerSaveData`, `SaveData`) are reasonable
- Authors recognized the need for feature gating (`multiplayer` flag)

### What's missing / broken
- ❌ Network crate doesn't compile (3 errors)
- ❌ No actual network transport code (WebSocket not used)
- ❌ No client/server connection state machines
- ❌ No room/instance management
- ❌ No entity replication or state synchronization
- ❌ Server has no server-specific logic
- ❌ Save crate is dead code superseded by core/db.rs
- ❌ Zero tests across all three crates (0 tests total)
- ❌ No atomic writes, no checksums, no integrity guarantees
- ❌ No CI checks, no linting configured for these crates

---

## Improvement Roadmap

### Immediate (fix build, 1 day)
1. **Fix `lib.rs` broken imports** — remove `IsMultiplayer` from plugin re-export, fix client/server re-exports to match actual types
2. **Remove or refactor save crate** — either delete it (it's dead code) or wrap it as a thin convenience layer over `core/src/db.rs`

### Short-term (make functional, 1-2 weeks)
3. **Implement plugin resources** — add `NetworkClient`/`NetworkServer` as Bevy resources in `NetworkPlugin`
4. **Build WebSocket client** — use tungstenite to connect, send/receive `NetworkMessage`, manage connection state
5. **Build WebSocket server** — use tungstenite to accept connections, spawn per-client tasks, route messages
6. **Add room management** — room create/join/leave, player roster, instance assignment
7. **Wire `build_server_app()`** — add `ServerPlugin` to the server app with actual systems

### Medium-term (make reliable, 2-4 weeks)
8. **Entity replication** — implement `WorldSnapshot` broadcasting, delta compression, tick-based state sync
9. **Authoritative simulation** — server-side combat/movement validation with client prediction
10. **Auth system** — token validation, session management, reconnection
11. **Atomic save with checksums** — temp-file + rename pattern, CRC32/SHA256 on save data
12. **Testing** — protocol round-trip tests, connection lifecycle tests, save/load integrity tests, crash recovery tests

### Long-term (enterprise grade, 1-3 months)
13. **Reconnect & resilience** — session resume after disconnect, state reconciliation
14. **Rate limiting & anti-cheat** — server-side input validation, message throttling
15. **Observability** — connection metrics, latency tracking, structured logging
16. **CI pipeline** — `cargo check --features multiplayer`, `cargo test`, `cargo clippy` on every PR
