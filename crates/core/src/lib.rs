//! Core shared types, components, resources, and events for the isometric roguelite.
//! This crate is the foundation — every other crate depends on it.

pub mod components;
pub mod resources;
pub mod events;
pub mod bundles;
pub mod items;
pub mod hitbox;
pub mod db;
pub mod tween;
pub mod plugin;

pub use components::{
    Ability, AbilityCooldowns, AbilityKind, AttackCooldown, CharacterClass, ClassAbilityId,
    ClassResource, CombatStats, DashCooldown, DashTrail, Enemy, EnemyProjectileMarker,
    EnemyVariant, ExperienceGem, ForcedMovement, Frozen, Health, HitFlash, HitStop, HitStun,
    Knockback, Lifetime, MagicProjectile, Pickup, PickupKind, Player, PlayerClass, PlayerName,
    Position, Projectile, ProjectileOwner, RenderInfo, RespawnTimer, Room, RoomEntity, Stamina,
    Stun, Team, TelegraphIndicator, TrailSegment, Velocity, Weapon, WeaponKind,
};
pub use events::*;
pub use hitbox::*;
pub use items::*;
pub use tween::*;
pub use bundles::*;
pub use plugin::CorePlugin;
pub use resources::{
    AppState, CameraTransform, CharacterCreationState, CursorWorldPos, DungeonInstance,
    DungeonState, GameAssets, GameConfig, MetaProgression, PlayTimer, PlayerInput, PlayerProfile,
    PlayerProfiles, RunProgression, RunState, ScreenShake, UpgradeTier, WaveState,
    DeathPenalty, Graveyard, ItemDatabase, PendingClassSpawn,
};
