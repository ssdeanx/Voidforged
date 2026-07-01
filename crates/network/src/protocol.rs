//! WebSocket-based protocol between game client and authoritative server.
//!
//! Every message is a JSON-serialized `NetworkMessage` envelope with a
//! monotonic sequence number for ordering and dedup.

use serde::{Deserialize, Serialize};

pub type SequenceNo = u64;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct NetworkMessage {
    pub seq: SequenceNo,
    pub payload: MessagePayload,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum MessagePayload {
    ServerHello(ServerHello),
    AuthRequest(AuthRequest),
    AuthResponse(AuthResponse),
    Ping,
    Pong,
    ClientInput(ClientInput),
    AbilityRequest(AbilityRequest),
    WorldSnapshot(WorldSnapshot),
    EntityUpdate(EntityUpdate),
    ChatMessage(ChatMessage),
    JoinRoom(JoinRoom),
    LeaveRoom,
    RoomState(RoomState),
    Disconnect(DisconnectReason),
    DisconnectAck,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ServerHello {
    pub protocol_version: String,
    pub server_name: String,
    pub heartbeat_interval_s: f32,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthRequest {
    pub token: String,
    pub player_name: String,
    pub client_version: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AuthResponse {
    pub success: bool,
    pub player_id: u64,
    pub reason: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ClientInput {
    pub movement_dir: (f32, f32),
    pub aim_dir: (f32, f32),
    pub primary: bool,
    pub secondary: bool,
    pub cast: bool,
    pub dodge: bool,
    pub interact: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AbilityRequest {
    pub ability_id: String,
    pub target: Option<AbilityTarget>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum AbilityTarget {
    Direction(f32, f32),
    Position(f32, f32, f32),
    Entity(u64),
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WorldSnapshot {
    pub tick: u64,
    pub entities: Vec<ReplicatedEntity>,
    pub removals: Vec<u64>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ReplicatedEntity {
    pub id: u64,
    pub entity_type: String,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub health: Option<f32>,
    pub max_health: Option<f32>,
    pub velocity: Option<(f32, f32, f32)>,
    pub animation_state: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityUpdate {
    pub entity_id: u64,
    pub field: String,
    pub value: serde_json::Value,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ChatMessage {
    pub sender_id: Option<u64>,
    pub sender_name: String,
    pub text: String,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct JoinRoom {
    pub room_id: Option<String>,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomState {
    pub room_id: String,
    pub players: Vec<RoomPlayer>,
    pub instance_type: InstanceType,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RoomPlayer {
    pub player_id: u64,
    pub player_name: String,
    pub ready: bool,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum InstanceType {
    OpenWorld,
    Dungeon { dungeon_id: String, difficulty: u32 },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum DisconnectReason {
    ClientQuit,
    ServerShutdown,
    Kicked(String),
    Timeout,
    Reconnect,
}

impl NetworkMessage {
    pub fn new(seq: SequenceNo, payload: MessagePayload) -> Self {
        Self { seq, payload }
    }
}

impl ServerHello {
    pub fn new(server_name: &str) -> Self {
        Self {
            protocol_version: env!("CARGO_PKG_VERSION").to_string(),
            server_name: server_name.to_string(),
            heartbeat_interval_s: 5.0,
        }
    }
}

