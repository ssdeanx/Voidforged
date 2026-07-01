//! Client-side network connection state machine.
//!
//! Manages a single WebSocket connection to the authoritative server.
//! Outgoing messages are queued and flushed each frame; incoming messages
//! are dispatched as Bevy events.
//!
//! # State machine
//!
//! ```text
//! ┌──────────┐  connect()  ┌───────────┐  on_open   ┌───────────┐
//! │Disconnected│ ──────────→ │Connecting│ ──────────→ │Connected │
//! └──────────┘             └───────────┘             └─────┬─────┘
//!       ▲                        ▲                        │
//!       │  on_close / error      │  timeout               │  disconnect()
//!       └────────────────────────┴────────────────────────┘
//! ```

use crate::protocol::{NetworkMessage, SequenceNo};
use bevy::prelude::*;

// ── Configuration ──────────────────────────────────────────────────────────

/// Configuration for the network client.
#[derive(Debug, Clone, Resource)]
pub struct ClientConfig {
    /// Server address (e.g. `"127.0.0.1:9876"`).
    pub server_addr: String,
    /// Optional player authentication token.
    pub auth_token: Option<String>,
    /// Player display name sent during authentication.
    pub player_name: String,
    /// Interval (in seconds) between heartbeat pings.
    pub heartbeat_interval_s: f32,
    /// Connection timeout in seconds.
    pub connect_timeout_s: f32,
}

impl Default for ClientConfig {
    fn default() -> Self {
        Self {
            server_addr: "127.0.0.1:9876".to_string(),
            auth_token: None,
            player_name: "Player".to_string(),
            heartbeat_interval_s: 5.0,
            connect_timeout_s: 10.0,
        }
    }
}

// ── Connection state ───────────────────────────────────────────────────────

/// Current status of the client connection.
#[derive(Debug, Clone, PartialEq, Eq, Hash)]
pub enum ConnectionState {
    /// Not connected and not attempting to connect.
    Disconnected,
    /// Handshake / TCP setup in progress.
    Connecting,
    /// WebSocket is open and authenticated.
    Connected {
        /// Last tick received from the server for state synchronization.
        server_tick: u64,
    },
    /// Graceful disconnect in progress.
    Disconnecting,
}

impl Default for ConnectionState {
    fn default() -> Self {
        Self::Disconnected
    }
}

// ── Client resource ────────────────────────────────────────────────────────

/// Network client resource managed by the Bevy scheduler.
///
/// Holds the current connection state and a send queue. Systems
/// read/write this resource each frame — the plugin drains the
/// send queue and polls the WebSocket for incoming messages.
#[derive(Debug, Resource)]
pub struct NetworkClient {
    /// Current connection state.
    pub state: ConnectionState,
    /// Outgoing message queue (drained each frame by the I/O system).
    pub send_queue: Vec<NetworkMessage>,
    /// Next sequence number for outgoing messages.
    pub next_seq: SequenceNo,
}

impl Default for NetworkClient {
    fn default() -> Self {
        Self {
            state: ConnectionState::Disconnected,
            send_queue: Vec::new(),
            next_seq: 0,
        }
    }
}

impl NetworkClient {
    /// Queue a message to be sent on the next I/O tick.
    ///
    /// Messages are silently dropped when the client is not connected.
    pub fn send(&mut self, payload: impl Into<crate::protocol::MessagePayload>) {
        let msg = NetworkMessage::new(self.next_seq, payload.into());
        self.next_seq += 1;
        self.send_queue.push(msg);
    }
}

// ── Client events (emitted by the I/O system) ──────────────────────────────

/// Fired when the WebSocket connection is established and the server
/// has sent its `ServerHello`.
#[derive(Debug, Event)]
pub struct ConnectedEvent {
    /// Human-readable server name from the `ServerHello`.
    pub server_name: String,
    /// Protocol version reported by the server.
    pub protocol_version: String,
}

/// Fired when the connection is closed for any reason.
#[derive(Debug, Event)]
pub struct DisconnectedEvent {
    /// Human-readable reason for the disconnection.
    pub reason: String,
}

/// Fired when a `WorldSnapshot` arrives — the server's view of replicated
/// entities at a specific tick.
#[derive(Debug, Event)]
pub struct SnapshotReceivedEvent {
    /// The world snapshot payload from the server.
    pub snapshot: crate::protocol::WorldSnapshot,
}

/// Fired when a `ChatMessage` arrives from another player or the server.
#[derive(Debug, Event)]
pub struct ChatReceivedEvent {
    /// Display name of the message sender.
    pub sender_name: String,
    /// Message body text.
    pub text: String,
}
