//! Network crate — multiplayer protocol, client/server communication.
//!
//! # Architecture
//!
//! - `protocol` — WebSocket message types (envelope + payload enum).
//! - `client` — Client-side connection state machine.
//! - `server` — Server-side connection and room manager.
//! - `plugin` — Bevy plugin that registers network resources.
//!
//! # Feature flags
//!
//! - `multiplayer` (default: off) — enables the full network stack with
//!   WebSocket (tungstenite) and UUID dependencies.

/// WebSocket message types (envelope + payload enum).
pub mod protocol;
/// Bevy plugin that registers network resources and feature-gated I/O systems.
pub mod plugin;

#[cfg(feature = "multiplayer")]
/// Client-side connection state machine and configuration.
pub mod client;

#[cfg(feature = "multiplayer")]
/// Server-side connection and room manager.
pub mod server;

#[cfg(feature = "multiplayer")]
/// ECS world replication configuration.
pub mod replication;

pub use plugin::{IsMultiplayer, NetworkPlugin};
pub use protocol::*;

#[cfg(feature = "multiplayer")]
pub use client::{ClientConfig, ConnectionState, NetworkClient};

#[cfg(feature = "multiplayer")]
pub use server::{ClientConnection, ClientId, NetworkServer, Room, RoomId, ServerConfig, SessionId};

#[cfg(feature = "multiplayer")]
pub use replication::ReplicationConfig;
