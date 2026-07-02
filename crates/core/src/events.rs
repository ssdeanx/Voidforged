//! Bevy events for combat, progression, waves, rooms, game state,
//! player death, and equipment management.

use bevy::prelude::*;
use crate::components::*;

// ============================================================================
// Combat Events
// ============================================================================

/// Fired when damage is dealt to an entity.
///
/// Consumed by damage application, floating numbers, screen shake, and
/// death-check systems. Also used by combat logging.
#[derive(Event, Debug, Clone)]
pub struct DamageEvent {
    /// The entity that received the damage.
    pub target: Entity,
    /// The entity that dealt the damage.
    pub source: Entity,
    /// Raw damage amount before armour/damage-type mitigation.
    pub amount: f32,
    /// Whether the hit was a critical strike.
    pub is_critical: bool,
    /// Physical, Magic, or True damage type.
    pub damage_type: DamageType,
    /// World-space position where the hit landed (for spawning effects).
    pub hit_position: Option<Vec3>,
}

/// Categorisation of damage for resistances and VFX.
#[derive(Debug, Clone, PartialEq, Eq)]
pub enum DamageType {
    /// Reduced by armour; used by most weapon attacks.
    Physical,
    /// Reduced by magic resistance; used by spells and magical attacks.
    Magic,
    /// Bypasses all defences; used by environmental and special effects.
    True,
}

/// Fired when an entity's health reaches zero.
///
/// Consumed by drops, XP rewards, kill counters, and wave-clearing logic.
#[derive(Event, Debug, Clone)]
pub struct DeathEvent {
    /// The entity that died.
    pub entity: Entity,
    /// The entity that delivered the killing blow, if any.
    pub killer: Option<Entity>,
    /// Enemy variant for loot table and XP calculations.
    pub enemy_variant: Option<EnemyVariant>,
}

// ============================================================================
// Progression Events
// ============================================================================

/// Fired when the player gains experience from any source.
#[derive(Event, Debug, Clone)]
pub struct ExperienceGainEvent {
    /// Amount of XP gained.
    pub amount: u64,
    /// Source entity (enemy, quest, etc.) that granted the XP.
    pub source: Entity,
}

/// Fired when the player's level increases.
#[derive(Event, Debug, Clone)]
pub struct LevelUpEvent {
    /// The new level reached.
    pub new_level: u32,
}

/// Fired when the player picks up a collectible item (XP gem, gold, health orb).
#[derive(Event, Debug, Clone)]
pub struct PickupEvent {
    /// The player entity.
    pub player: Entity,
    /// The collected item entity.
    pub item: Entity,
    /// What kind of pickup was collected.
    pub kind: PickupKind,
}

// ============================================================================
// Wave & Encounter Events
// ============================================================================

/// Fired when a new wave of enemies begins spawning.
#[derive(Event, Debug, Clone)]
pub struct WaveStartEvent {
    /// The wave number (1-indexed).
    pub wave_number: u32,
    /// Total enemies to be spawned this wave.
    pub enemy_count: u32,
}

/// Fired when all enemies in the current wave have been defeated.
#[derive(Event, Debug, Clone)]
pub struct WaveClearedEvent {
    /// The wave number that was cleared.
    pub wave_number: u32,
}

/// Fired when the player transitions between rooms or zones.
#[derive(Event, Debug, Clone)]
pub struct RoomTransitionEvent {
    /// The room being left, if any.
    pub from_room: Option<Entity>,
    /// The room being entered.
    pub to_room: Entity,
}

// ============================================================================
// Game State Events
// ============================================================================

/// Fired when a new run begins (dungeon entry or open-world start).
#[derive(Event, Debug, Clone)]
pub struct RunStartEvent;

/// Fired when a run ends (victory or death).
///
/// Contains summary statistics for the end-of-run recap screen.
#[derive(Event, Debug, Clone)]
pub struct RunEndEvent {
    /// Whether the run ended in victory.
    pub victory: bool,
    /// Highest wave number reached.
    pub wave_reached: u32,
    /// Total enemies killed.
    pub kills: u32,
    /// Total elapsed run time in seconds.
    pub run_time: f32,
}

/// Fired to display a floating damage number above the target.
#[derive(Event, Debug, Clone)]
pub struct DamageNumberEvent {
    /// World position where the number should appear.
    pub position: Vec3,
    /// Damage amount (displayed as integer).
    pub amount: i32,
    /// Whether this was a critical hit (alters colour/size).
    pub is_crit: bool,
    /// Damage type for colour coding.
    pub damage_type: DamageType,
}

/// Fired to spawn a visual particle impact effect at a position.
#[derive(Event, Debug, Clone)]
pub struct SpawnImpactEvent {
    /// World position of the impact.
    pub position: Vec3,
    /// Optional RGBA colour override for the particles.
    pub color: Option<Vec4>,
}

/// Fired to spawn a death particle effect at a position.
#[derive(Event, Debug, Clone)]
pub struct SpawnDeathEffectEvent {
    pub position: Vec3,
    pub enemy_variant: EnemyVariant,
}

/// Fired when the player takes damage to show a screen-edge direction indicator.
///
/// The rendering system displays a brief red flash on the screen edge
/// from the direction the damage originated.
#[derive(Event, Debug, Clone)]
pub struct HitDirectionEvent {
    /// Normalized world-space direction from damage source to player.
    pub direction: Vec3,
}

// ============================================================================
// Player Death Events
// ============================================================================

/// Fired when the player dies in any context (open-world or dungeon).
///
/// Triggers screen shake, death penalty calculations, and state transitions.
#[derive(Event, Debug, Clone)]
pub struct PlayerDeathEvent {
    /// The player entity that died.
    pub player: Entity,
    /// The entity that killed the player, if any.
    pub killer: Option<Entity>,
    /// Position where the player died.
    pub position: Vec3,
    /// Whether the death occurred inside a dungeon instance.
    pub in_dungeon: bool,
}

/// Fired when a dungeon run ends via death or boss clear.
///
/// Contains statistics for the post-dungeon summary screen.
#[derive(Event, Debug, Clone)]
pub struct DungeonEndEvent {
    /// Whether the dungeon was cleared (boss defeated) vs. player died.
    pub cleared: bool,
    /// Total kills during this dungeon run.
    pub kills: u32,
    /// Highest wave reached.
    pub wave_reached: u32,
    /// Gold collected in this dungeon.
    pub gold_collected: u64,
    /// XP earned in this dungeon.
    pub xp_earned: u64,
    /// Total run time in seconds.
    pub run_time: f32,
}

// ============================================================================
// Equipment Events
// ============================================================================

/// Fired when the player wants to equip an item from inventory into a slot.
#[derive(Event, Debug, Clone)]
pub struct EquipItemEvent {
    /// Index in the player's inventory of the item to equip.
    pub inventory_slot: usize,
    /// Target equipment slot.
    pub equip_slot: crate::items::EquipSlot,
}

/// Fired when the player wants to unequip an item from a slot back to inventory.
#[derive(Event, Debug, Clone)]
pub struct UnequipItemEvent {
    /// Equipment slot to unequip.
    pub equip_slot: crate::items::EquipSlot,
}
