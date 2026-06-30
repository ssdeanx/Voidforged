use bevy::prelude::*;
use crate::components::*;

// ============================================================================
// Combat Events
// ============================================================================

/// Fired when damage is dealt to an entity.
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    pub target: Entity,
    pub source: Entity,
    pub amount: f32,
    pub is_critical: bool,
    pub damage_type: DamageType,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageType {
    Physical,
    Magic,
    True,
}

/// Fired when an entity dies.
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    pub entity: Entity,
    pub killer: Option<Entity>,
    pub enemy_variant: Option<EnemyVariant>,
}

// ============================================================================
// Progression Events
// ============================================================================

/// Fired when the player gains XP.
#[derive(Event, Debug, Clone)]
pub struct ExperienceGainEvent {
    pub amount: u64,
    pub source: Entity,
}

/// Fired when the player levels up.
#[derive(Event, Debug, Clone)]
pub struct LevelUpEvent {
    pub new_level: u32,
}

/// Fired when the player picks up a collectible.
#[derive(Event, Debug, Clone)]
pub struct PickupEvent {
    pub player: Entity,
    pub item: Entity,
    pub kind: PickupKind,
}

// ============================================================================
// Wave & Encounter Events
// ============================================================================

/// Fired when a new wave starts.
#[derive(Event, Debug, Clone)]
pub struct WaveStartEvent {
    pub wave_number: u32,
    pub enemy_count: u32,
}

/// Fired when a wave is cleared (all enemies dead).
#[derive(Event, Debug, Clone)]
pub struct WaveClearedEvent {
    pub wave_number: u32,
}

/// Fired when the player enters or exits a room.
#[derive(Event, Debug, Clone)]
pub struct RoomTransitionEvent {
    pub from_room: Option<Entity>,
    pub to_room: Entity,
}

// ============================================================================
// Game State Events
// ============================================================================

/// Fired when a run begins.
#[derive(Event, Debug, Clone)]
pub struct RunStartEvent;

/// Fired when a run ends (victory or death).
#[derive(Event, Debug, Clone)]
pub struct RunEndEvent {
    pub victory: bool,
    pub wave_reached: u32,
    pub kills: u32,
    pub run_time: f32,
}
