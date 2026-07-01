//! Server-side network connection and room management.
//!
//! The server listens on a TCP port, accepts WebSocket connections,
//! authenticates clients, and manages rooms (open-world instances or
//! dungeon instances). Each connected client is tracked via a
//! [`ClientConnection`]; rooms hold sets of connections and carry an
//! [`InstanceType`] that determines the game-world simulation running
//! for its occupants.
//!
//! # Lifecycle
//!
//! 1. `NetworkServer::start()` binds a TCP listener and begins accepting.
//! 2. Each new connection is authenticated; on success a `ClientConnected`
//!    event is emitted.
//! 3. Clients join or create rooms via `JoinRoom` / `CreateRoom`.
//! 4. Room simulation runs via `ServerPlugin` systems.
//! 5. On disconnect the client is removed from its room and a
//!    `ClientDisconnected` event is emitted.

use crate::protocol::{InstanceType, SequenceNo};
use bevy::prelude::*;
use std::collections::HashMap;

// ── Type aliases ───────────────────────────────────────────────────────────

/// Opaque identifier for a connected client.
pub type ClientId = u64;

/// Opaque identifier for a game room (open-world shard or dungeon instance).
pub type RoomId = u64;

/// Unique identifier for a play session (survives reconnects within a timeout).
pub type SessionId = u64;

// ── Configuration ──────────────────────────────────────────────────────────

/// Server configuration.
#[derive(Debug, Clone, Resource)]
pub struct ServerConfig {
    /// TCP address to bind (e.g. `"0.0.0.0:9876"`).
    pub bind_addr: String,
    /// Maximum number of concurrent connections.
    pub max_connections: u32,
    /// Maximum players per room.
    pub max_players_per_room: u32,
    /// Tick rate (Hz) for the server simulation step.
    pub tick_rate_hz: u32,
    /// Time (seconds) without a heartbeat before a client is timed out.
    pub heartbeat_timeout_s: f32,
}

impl Default for ServerConfig {
    fn default() -> Self {
        Self {
            bind_addr: "0.0.0.0:9876".to_string(),
            max_connections: 64,
            max_players_per_room: 8,
            tick_rate_hz: 20,
            heartbeat_timeout_s: 15.0,
        }
    }
}

// ── Client connection ──────────────────────────────────────────────────────

/// Tracks a single connected client on the server.
#[derive(Debug, Clone)]
pub struct ClientConnection {
    /// Unique client identifier.
    pub id: ClientId,
    /// Player-chosen display name.
    pub player_name: String,
    /// Optional authentication token.
    pub auth_token: Option<String>,
    /// Room this client is currently in (`None` = lobby).
    pub room_id: Option<RoomId>,
    /// Monotonically increasing sequence number for the last received message.
    pub last_seq: SequenceNo,
    /// Time of the last received message (for timeout detection).
    pub last_heartbeat: f32,
}

impl ClientConnection {
    /// Create a new `ClientConnection` with the given id, player name,
    /// and optional auth token.
    ///
    /// The connection starts with no room assignment, a zero sequence
    /// number, and a zero heartbeat timestamp.
    pub fn new(id: ClientId, player_name: String, auth_token: Option<String>) -> Self {
        Self {
            id,
            player_name,
            auth_token,
            room_id: None,
            last_seq: 0,
            last_heartbeat: 0.0,
        }
    }
}

// ── Room ───────────────────────────────────────────────────────────────────

/// A game room (open-world shard or dungeon instance) containing a set of
/// connected clients.
#[derive(Debug, Clone)]
pub struct Room {
    /// Unique room identifier.
    pub id: RoomId,
    /// Human-readable room name (optional).
    pub name: Option<String>,
    /// Type of instance driving this room's simulation.
    pub instance_type: InstanceType,
    /// Client ids currently occupying this room.
    pub clients: Vec<ClientId>,
    /// Maximum number of clients.
    pub max_clients: u32,
}

impl Room {
    /// Create a new `Room` with the given id, instance type, and
    /// maximum client capacity.
    ///
    /// The room starts empty with no name.
    pub fn new(id: RoomId, instance_type: InstanceType, max_clients: u32) -> Self {
        Self {
            id,
            name: None,
            instance_type,
            clients: Vec::new(),
            max_clients,
        }
    }

    /// Returns `true` if the room is full.
    pub fn is_full(&self) -> bool {
        self.clients.len() >= self.max_clients as usize
    }

    /// Returns `true` if the room has no clients.
    pub fn is_empty(&self) -> bool {
        self.clients.is_empty()
    }
}

// ── NetworkServer resource ─────────────────────────────────────────────────

/// Server-side network resource managed by the Bevy scheduler.
///
/// Owns the connection map, room map, and a pending-outgoing buffer
/// that the I/O system drains each frame.
#[derive(Debug, Resource)]
pub struct NetworkServer {
    /// Server configuration (set before `start()`).
    pub config: ServerConfig,
    /// All connected clients keyed by [`ClientId`].
    pub clients: HashMap<ClientId, ClientConnection>,
    /// All active rooms keyed by [`RoomId`].
    pub rooms: HashMap<RoomId, Room>,
    /// Next available client id.
    next_client_id: ClientId,
    /// Next available room id.
    next_room_id: RoomId,
    /// Whether the server is currently accepting connections.
    pub running: bool,
    /// Outgoing messages keyed by target [`ClientId`].
    /// The I/O system drains this map each frame.
    pub outgoing: HashMap<ClientId, Vec<crate::protocol::NetworkMessage>>,
}

impl Default for NetworkServer {
    fn default() -> Self {
        Self {
            config: ServerConfig::default(),
            clients: HashMap::new(),
            rooms: HashMap::new(),
            next_client_id: 1,
            next_room_id: 1,
            running: false,
            outgoing: HashMap::new(),
        }
    }
}

impl NetworkServer {
    /// Begin accepting connections (stub — actual TCP bind in I/O system).
    pub fn start(&mut self) {
        self.running = true;
        info!("NetworkServer starting on {}", self.config.bind_addr);
    }

    /// Gracefully stop the server and disconnect all clients.
    pub fn stop(&mut self) {
        self.running = false;
        self.clients.clear();
        self.outgoing.clear();
        info!("NetworkServer stopped");
    }

    /// Queue a message for a specific client.
    pub fn send_to(&mut self, client_id: ClientId, msg: crate::protocol::NetworkMessage) {
        self.outgoing.entry(client_id).or_default().push(msg);
    }

    /// Broadcast a message to all clients in a room.
    pub fn broadcast_to_room(&mut self, room_id: RoomId, msg: crate::protocol::NetworkMessage) {
        let Some(room) = self.rooms.get(&room_id) else {
            return;
        };
        let client_ids = room.clients.clone();
        for cid in client_ids {
            self.send_to(cid, msg.clone());
        }
    }

    /// Create a new room and return its id.
    pub fn create_room(&mut self, instance_type: InstanceType, max_clients: u32) -> RoomId {
        let id = self.next_room_id;
        self.next_room_id += 1;
        self.rooms.insert(id, Room::new(id, instance_type, max_clients));
        id
    }

    /// Assign a client to a room (removes from previous room first).
    pub fn assign_to_room(&mut self, client_id: ClientId, room_id: RoomId) {
        // Remove from previous room
        if let Some(prev_room_id) = self.clients.get(&client_id).and_then(|c| c.room_id) {
            if let Some(room) = self.rooms.get_mut(&prev_room_id) {
                room.clients.retain(|c| *c != client_id);
            }
        }
        // Add to new room
        if let Some(room) = self.rooms.get_mut(&room_id) {
            if !room.is_full() && !room.clients.contains(&client_id) {
                room.clients.push(client_id);
            }
        }
        if let Some(client) = self.clients.get_mut(&client_id) {
            client.room_id = Some(room_id);
        }
    }

    /// Remove a client from the server (disconnect cleanup).
    pub fn remove_client(&mut self, client_id: ClientId) {
        if let Some(room_id) = self.clients.get(&client_id).and_then(|c| c.room_id) {
            if let Some(room) = self.rooms.get_mut(&room_id) {
                room.clients.retain(|c| *c != client_id);
            }
        }
        self.clients.remove(&client_id);
        self.outgoing.remove(&client_id);
    }

    /// Allocate a new client id and register the connection.
    pub fn register_client(
        &mut self,
        player_name: String,
        auth_token: Option<String>,
    ) -> ClientId {
        let id = self.next_client_id;
        self.next_client_id += 1;
        self.clients.insert(
            id,
            ClientConnection::new(id, player_name, auth_token),
        );
        id
    }
}

// ── Server events ──────────────────────────────────────────────────────────

/// Fired when a new client has authenticated and been registered.
#[derive(Debug, Event)]
pub struct ClientConnectedEvent {
    /// The newly assigned client identifier.
    pub client_id: ClientId,
    /// Display name sent by the client during authentication.
    pub player_name: String,
}

/// Fired when a client disconnects or times out.
#[derive(Debug, Event)]
pub struct ClientDisconnectedEvent {
    /// Identifier of the client that disconnected.
    pub client_id: ClientId,
    /// Human-readable reason for the disconnection.
    pub reason: String,
}

/// Fired when a client joins a room.
#[derive(Debug, Event)]
pub struct ClientJoinedRoomEvent {
    /// Identifier of the client that joined.
    pub client_id: ClientId,
    /// Identifier of the room the client joined.
    pub room_id: RoomId,
}

/// Fired when a client leaves a room.
#[derive(Debug, Event)]
pub struct ClientLeftRoomEvent {
    /// Identifier of the client that left.
    pub client_id: ClientId,
    /// Identifier of the room the client left.
    pub room_id: RoomId,
}
