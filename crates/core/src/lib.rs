//! Core shared types, components, resources, and events for the isometric roguelite.
//! This crate is the foundation — every other crate depends on it.

pub mod components;
pub mod resources;
pub mod events;
pub mod bundles;
pub mod items;
pub mod hitbox;
pub mod db;
pub mod plugin;

pub use components::{
    Ability, AbilityKind, AttackCooldown, CharacterClass, ClassAbilityId, CombatStats,
    DashCooldown, Enemy, EnemyVariant, ExperienceGem, Health, Lifetime, Pickup, PickupKind, Player,
    PlayerClass, PlayerName, Position, Projectile, ProjectileOwner, RenderInfo, Room, RoomEntity,
    Stamina, Team, Velocity, Weapon, WeaponKind,
};
pub use events::*;
pub use hitbox::*;
pub use items::*;
pub use bundles::*;
pub use plugin::CorePlugin;
pub use resources::{
    AppState, CameraTransform, CharacterCreationState, CursorWorldPos, DungeonInstance,
    DungeonState, GameAssets, GameConfig, MetaProgression, PlayTimer, PlayerInput, PlayerProfile,
    PlayerProfiles, RunProgression, RunState, ScreenShake, UpgradeTier, WaveState,
    DeathPenalty, Graveyard, ItemDatabase, PendingClassSpawn,
};
