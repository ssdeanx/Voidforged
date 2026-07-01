//! ECS replication strategy — how game state flows between server and client.
//!
//! # Architecture overview
//!
//! Voidforged uses an **authoritative server** model — all gameplay-critical
//! simulation runs on the dedicated server. Clients send inputs and receive
//! periodic snapshots of the world state.
//!
//! ```text
//! ┌──────────────────┐   WebSocket   ┌──────────────────────┐
//! │  Client (bevy)   │ ◄───────────► │  Server (bevy)       │
//! │  - Input → server│   JSON msgs   │  - Owns simulation   │
//! │  - Render snap   │               │  - Broadcasts snaps  │
//! └──────────────────┘               └──────────────────────┘
//! ```
//!
//! # Replicated data
//!
//! The server replicates a subset of ECS component data to clients:
//!
//! | Component / data       | Replicated? | Strategy                          |
//! |------------------------|-------------|-----------------------------------|
//! | `Transform`            | ✅ Yes      | Per-entity position in snapshot   |
//! | `Health`               | ✅ Yes      | Current + max in snapshot         |
//! | `Velocity`             | ✅ Yes      | For interpolation                 |
//! | `AnimationState` (str) | ✅ Yes      | Enum name in snapshot             |
//! | `Player` marker        | ✅ Yes      | Included implicitly               |
//! | `Enemy` marker         | ✅ Yes      | Via `entity_type` discriminator   |
//! | UI state (healthbar…)  | ❌ No       | Computed client-side from health  |
//! | Input buffer           | ❌ No       | Client → server only              |
//! | Procedural seed        | ❌ No       | Sent once on room join            |
//!
//! # Tick loop
//!
//! 1. **Receive phase** (`PreUpdate`): server reads and processes incoming
//!    `ClientInput` messages from each connected client.
//! 2. **Simulate phase** (`Update`): the server's simulation systems run
//!    (AI, physics, combat, loot). The timer is driven by `ServerTick`.
//! 3. **Snapshot phase** (`PostUpdate`): the server collects all entities
//!    that have changed since the last tick, builds a `WorldSnapshot`, and
//!    enqueues it for broadcast to each room's clients.
//!
//! # Snapshot format
//!
//! A `WorldSnapshot` (defined in [`crate::protocol`]) contains:
//!
//! - `tick`: monotonic server tick id (used for client-side interpolation).
//! - `entities`: list of `ReplicatedEntity` structs with position, health,
//!   velocity, and animation state for each visible entity.
//! - `removals`: list of entity ids that were despawned this tick.
//!
//! # Client-side reconciliation
//!
//! The client does **not** run its own simulation of replicated entities.
//! Instead it:
//!
//! 1. Stores the last two snapshots.
//! 2. Interpolates positions between them based on local render time.
//! 3. Applies entity removals immediately.
//! 4. Sends `ClientInput` every frame (not every tick).
//!
//! Player-controlled entities use **client-side prediction** for immediate
//! responsiveness: the local player's movement is predicted locally and
//! corrected when the next server snapshot arrives.
//!
//! # Room isolation
//!
//! Each `Room` has an [`InstanceType`] that determines which entities
//! are simulated and which clients receive snapshots. The server runs
//! separate simulation tick stacks per active room — an open-world shard
//! and a dungeon instance are independent ECS worlds.
//!
//! (In the initial implementation all clients in a room share one Bevy
//! `World`. Future optimisation: run each room on its own `World` with
//! a separate schedule.)
//!
//! # Future: delta compression
//!
//! Once the basic snapshot loop is stable, snapshots should switch from
//! full-state to delta-compressed: only entities whose components have
//! changed since `tick - 1` are sent, plus a full-state snapshot every
//! N ticks for recovery after packet loss.

use serde::{Deserialize, Serialize};

/// Strategy configuration — tunable parameters for the replication loop.
///
/// These live in `resources::ServerConfig` and `resources::ClientConfig`
/// at runtime; this struct documents the tunables and their defaults.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicationConfig {
    /// How many snapshots per second the server broadcasts (default: 20).
    pub snapshot_rate_hz: u32,
    /// How many ticks between full-state snapshots (default: 10).
    pub full_state_interval: u32,
    /// Whether to enable client-side prediction for local player.
    pub client_prediction: bool,
}

impl Default for ReplicationConfig {
    fn default() -> Self {
        Self {
            snapshot_rate_hz: 20,
            full_state_interval: 10,
            client_prediction: true,
        }
    }
}
