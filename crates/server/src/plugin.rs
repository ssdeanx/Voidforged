//! Server-specific Bevy plugin, resources, and system stubs.
//!
//! The `ServerPlugin` should be added to the server app alongside
//! `ir_network::NetworkPlugin`. It owns the server game-loop: lobby
//! management, room lifecycle, tick-driven simulation, and snapshot
//! broadcasting.

use bevy::prelude::*;

// ── Resources ──────────────────────────────────────────────────────────────

/// Lobby state — players waiting to join or create a room.
#[derive(Debug, Resource)]
pub struct Lobby {
    /// Client ids currently in the lobby (not yet assigned to a room).
    pub pending_clients: Vec<ir_network::ClientId>,
}

impl Default for Lobby {
    fn default() -> Self {
        Self {
            pending_clients: Vec::new(),
        }
    }
}

/// Tracks the current simulation tick on the server.
#[derive(Debug, Resource)]
pub struct ServerTick(pub u64);

impl Default for ServerTick {
    fn default() -> Self {
        Self(0)
    }
}

/// Marker resource indicating this binary is running as a dedicated server.
///
/// Systems can check for this to skip client-only logic (rendering, input).
#[derive(Debug, Resource)]
pub struct IsDedicatedServer;

// ── ServerPlugin ───────────────────────────────────────────────────────────

/// Bevy plugin for the dedicated game server.
///
/// Registers the server game-loop: tick advancement, lobby management,
/// room lifecycle, and snapshot broadcasting. Adds the [`Lobby`],
/// [`ServerTick`], and [`IsDedicatedServer`] resources.
///
/// This plugin should be added alongside `ir_network::NetworkPlugin`
/// in the headless server binary. It is **not** intended for the client app.
pub struct ServerPlugin;

impl Plugin for ServerPlugin {
    fn build(&self, app: &mut App) {
        app
            // Indicate this is a dedicated server (no rendering, no input).
            .insert_resource(IsDedicatedServer)
            // Server loop resources
            .init_resource::<Lobby>()
            .init_resource::<ServerTick>()

            // ── Systems ──────────────────────────────────────────
            // Tick advance — runs every frame on the dedicated server.
            .add_systems(PreUpdate, advance_tick)

            // Room lifecycle (stubs)
            // .add_systems(Update, (
            //     lobby_tick::system,
            //     create_room_on_join::system,
            //     destroy_empty_rooms::system,
            // ))

            // Snapshot broadcast (stub — runs after world simulation each tick)
            // .add_systems(PostUpdate, broadcast_world_snapshot::system)

            // Timeout detection (stub — runs at a fixed interval)
            // .add_systems(Update, detect_client_timeouts::system)
        ;
    }
}

// ── Systems ────────────────────────────────────────────────────────────────

/// Advance the server tick counter.
///
/// The dedicated server runs at a fixed tick rate (defined in
/// `ServerConfig`). Each tick the simulation steps and a world
/// snapshot is prepared for broadcast.
fn advance_tick(mut tick: ResMut<ServerTick>) {
    tick.0 += 1;
}
