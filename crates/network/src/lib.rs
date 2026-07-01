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

pub mod protocol;
pub mod plugin;

#[cfg(feature = "multiplayer")]
pub mod client;

#[cfg(feature = "multiplayer")]
pub mod server;

pub use plugin::{IsMultiplayer, NetworkPlugin};
pub use protocol::*;

#[cfg(feature = "multiplayer")]
pub use client::{ClientConfig, ConnectionState, NetworkClient};

#[cfg(feature = "multiplayer")]
pub use server::{ClientConnection, ClientId, NetworkServer, Room, RoomId, SessionId};
