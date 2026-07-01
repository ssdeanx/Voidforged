//! WebSocket-based protocol between game client and authoritative server.
//!
//! Every message is a JSON-serialized [`NetworkMessage`] envelope with a
//! monotonic sequence number for ordering and deduplication.
//!
//! # Message flow
//!
//! 1. Client connects → server sends [`ServerHello`].
//! 2. Client responds with [`AuthRequest`] → server replies [`AuthResponse`].
//! 3. Steady state: bi-directional exchange of [`WorldSnapshot`],
//!    [`EntityUpdate`], [`ClientInput`], [`ChatMessage`], and keep-alive
//!    (`Ping`/`Pong`).
//! 4. Disconnect: either side sends [`Disconnect`] → peer acknowledges
//!    with [`DisconnectAck`].

use serde::{Deserialize, Serialize};

/// Monotonically increasing sequence number for message ordering and dedup.
pub type SequenceNo = u64;

/// Envelope for every message sent over the WebSocket connection.
///
/// Wraps a [`MessagePayload`] with a sequence number that the receiver
/// uses to detect duplicates and reorder messages.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    /// Monotonic sequence number for ordering and deduplication.
    pub seq: SequenceNo,
    /// The typed message content.
    pub payload: MessagePayload,
}

/// All message types that can appear on the wire between client and server.
///
/// Each variant wraps a corresponding payload struct or carries simple
/// marker data inline.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    /// Initial handshake from server to client on connection open.
    ServerHello(ServerHello),
    /// Authentication credentials sent from client to server.
    AuthRequest(AuthRequest),
    /// Result of authentication sent from server to client.
    AuthResponse(AuthResponse),
    /// Keep-alive ping (client → server).
    Ping,
    /// Keep-alive pong (server → client).
    Pong,
    /// Player input snapshot (client → server, high frequency).
    ClientInput(ClientInput),
    /// Request to activate an ability (client → server).
    AbilityRequest(AbilityRequest),
    /// Full world state at a given tick (server → client, periodic).
    WorldSnapshot(WorldSnapshot),
    /// Single-entity field update (server → client, delta compression).
    EntityUpdate(EntityUpdate),
    /// Chat message from a player or the server.
    ChatMessage(ChatMessage),
    /// Request to join or create a room.
    JoinRoom(JoinRoom),
    /// Signal that a client is leaving its current room.
    LeaveRoom,
    /// Full state of a room sent to its occupants.
    RoomState(RoomState),
    /// Disconnect notification sent from either side.
    Disconnect(DisconnectReason),
    /// Acknowledgment of a disconnect notification.
    DisconnectAck,
}

/// Server greeting sent immediately after a WebSocket connection opens.
///
/// Contains protocol metadata the client uses to verify compatibility
/// and configure its heartbeat timing.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHello {
    /// Server protocol version (matches `CARGO_PKG_VERSION`).
    pub protocol_version: String,
    /// Human-readable server name.
    pub server_name: String,
    /// Recommended interval (in seconds) between heartbeat pings.
    pub heartbeat_interval_s: f32,
}

/// Authentication credentials sent from client to server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    /// Authentication token (opaque, e.g. JWT or session key).
    pub token: String,
    /// Display name chosen by the player.
    pub player_name: String,
    /// Client build version for compatibility checks.
    pub client_version: String,
}

/// Result of an authentication attempt sent from server to client.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    /// Whether authentication succeeded.
    pub success: bool,
    /// Assigned player id (valid only when `success` is `true`).
    pub player_id: u64,
    /// Human-readable reason for failure (present when `success` is `false`).
    pub reason: Option<String>,
}

/// Player input snapshot sent from client to server at each input tick.
///
/// Contains movement direction, aim direction, and action button states.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInput {
    /// Normalized 2D movement direction (x, y).
    pub movement_dir: (f32, f32),
    /// Normalized 2D aim direction (x, y) in screen-space.
    pub aim_dir: (f32, f32),
    /// Primary attack / fire button pressed.
    pub primary: bool,
    /// Secondary attack / alt-fire button pressed.
    pub secondary: bool,
    /// Spell / ability cast button pressed.
    pub cast: bool,
    /// Dodge / roll button pressed.
    pub dodge: bool,
    /// Interact / use button pressed.
    pub interact: bool,
}

/// Request from client to activate a specific ability.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityRequest {
    /// Identifier of the ability to activate.
    pub ability_id: String,
    /// Optional targeting information for the ability.
    pub target: Option<AbilityTarget>,
}

/// Targeting information for an ability activation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityTarget {
    /// Direction vector (x, y) for directional abilities (e.g. projectile).
    Direction(f32, f32),
    /// World position (x, y, z) for location-targeted abilities (e.g. AoE).
    Position(f32, f32, f32),
    /// Target entity id for single-target abilities.
    Entity(u64),
}

/// Full world snapshot at a specific simulation tick.
///
/// Sent periodically from server to client for state reconciliation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    /// Server simulation tick at which this snapshot was taken.
    pub tick: u64,
    /// Entities currently present in the world, with their latest state.
    pub entities: Vec<ReplicatedEntity>,
    /// Entity ids that have been removed since the last snapshot.
    pub removals: Vec<u64>,
}

/// A replicated entity within a world snapshot.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicatedEntity {
    /// Unique entity identifier (matches server-side entity).
    pub id: u64,
    /// Type tag for the entity (e.g. `"player"`, `"enemy"`, `"projectile"`).
    pub entity_type: String,
    /// World-space x-coordinate.
    pub x: f32,
    /// World-space y-coordinate.
    pub y: f32,
    /// World-space z-coordinate.
    pub z: f32,
    /// Current health, if applicable to this entity type.
    pub health: Option<f32>,
    /// Maximum health, if applicable.
    pub max_health: Option<f32>,
    /// Current velocity vector (x, y, z), if applicable.
    pub velocity: Option<(f32, f32, f32)>,
    /// Current animation state identifier, if applicable.
    pub animation_state: Option<String>,
}

/// A single-field update for an existing replicated entity.
///
/// Used for delta updates between full snapshots to reduce bandwidth.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdate {
    /// Id of the entity being updated.
    pub entity_id: u64,
    /// Name of the field being updated (e.g. `"health"`, `"position"`).
    pub field: String,
    /// New value for the field as a JSON value.
    pub value: serde_json::Value,
}

/// Chat message sent between players or from the server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    /// Id of the sending player (`None` for server-originated messages).
    pub sender_id: Option<u64>,
    /// Display name of the sender.
    pub sender_name: String,
    /// Message body text.
    pub text: String,
}

/// Request to join an existing room or create a new one.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRoom {
    /// Id of the room to join. `None` requests the server to find or
    /// create a suitable room.
    pub room_id: Option<String>,
}

/// Full state of a room sent to its occupants.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomState {
    /// Unique room identifier.
    pub room_id: String,
    /// Players currently in the room.
    pub players: Vec<RoomPlayer>,
    /// The type of game instance this room hosts.
    pub instance_type: InstanceType,
}

/// A player within a room's occupant list.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomPlayer {
    /// Unique player identifier.
    pub player_id: u64,
    /// Player display name.
    pub player_name: String,
    /// Whether the player has signalled ready (for dungeon start).
    pub ready: bool,
}

/// Type of game instance driving a room's simulation.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceType {
    /// Open-world exploration zone with dynamic population.
    OpenWorld,
    /// Procedurally generated dungeon with a set difficulty.
    Dungeon {
        /// Identifier for the dungeon template or floor set.
        dungeon_id: String,
        /// Difficulty level of the dungeon.
        difficulty: u32,
    },
}

/// Reason for a disconnect notification.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    /// Client voluntarily disconnected (quit).
    ClientQuit,
    /// Server is shutting down.
    ServerShutdown,
    /// Client was kicked by a moderator or the system.
    Kicked(String),
    /// Connection timed out due to inactivity.
    Timeout,
    /// Client disconnected for reconnection (session preserved briefly).
    Reconnect,
}

impl NetworkMessage {
    /// Create a new [`NetworkMessage`] with the given sequence number
    /// and payload.
    pub fn new(seq: SequenceNo, payload: MessagePayload) -> Self {
        Self { seq, payload }
    }
}

impl ServerHello {
    /// Build a new [`ServerHello`] with the given server name and the
    /// crate version as the protocol version.
    ///
    /// The heartbeat interval defaults to 5.0 seconds.
    pub fn new(server_name: &str) -> Self {
        Self {
            protocol_version: env!("CARGO_PKG_VERSION").to_string(),
            server_name: server_name.to_string(),
            heartbeat_interval_s: 5.0,
        }
    }
}
