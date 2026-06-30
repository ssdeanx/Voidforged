use serde::{Serialize, Deserialize};

/// Messages sent between client and server.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ClientMessage {
    JoinRequest { player_name: String },
    Input { direction: (f32, f32), attack: bool },
    Disconnect,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub enum ServerMessage {
    Welcome { player_id: u32 },
    WorldState { entities: Vec<EntityState> },
    Disconnect { reason: String },
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct EntityState {
    pub id: u32,
    pub x: f32,
    pub y: f32,
    pub z: f32,
    pub entity_type: String,
}
