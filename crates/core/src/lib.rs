//! Core shared types, components, resources, and events for Voidforged.
//!
//! This crate is the foundation of the game — every other crate depends on it.
//! It defines the ECS building blocks used across the entire codebase.
//!
//! # Modules
//!
//! - [`components`] — Bevy ECS components for players, enemies, projectiles, stats,
//!   spatial data, status effects, and spawning markers.
//! - [`resources`](self::resources) — Bevy resources for game state, configuration, assets, player input,
//!   combat tracking, death/respawn, dungeons, meta-progression, profiles, and item database.
//! - [`events`] — Bevy events for combat, progression, waves, rooms, game state changes,
//!   player death, and equipment management.
//! - [`bundles`] — Convenience `Bundle`s for spawning complex entities (player, enemy,
//!   projectile, experience gem) with all required components.
//! - [`items`] — Complete item system: definitions, instances, rarity tiers, equipment slots,
//!   stat modifiers, inventory, equipment, and gear score calculations.
//! - [`hitbox`] — Hitbox system with configurable shapes for damage zones.
//! - [`db`] — SQLite-backed save database for character profiles.
//! - [`plugin`] — The [`CorePlugin`] that registers all resources, events, and core systems.

pub mod components;
pub mod resources;
pub mod events;
pub mod bundles;
pub mod items;
pub mod hitbox;
pub mod db;
pub mod tween;
pub mod plugin;
pub mod specs;

pub use components::{
    Ability, AbilityCooldowns, AbilityKind, AbilitySlot, AttackCooldown, CharacterClass, ClassAbilityId,
    ClassResource, CombatStats, DashCooldown, DashTrail, DashTrailTimer, Enemy, EnemyProjectileMarker,
    EnemyVariant, ExperienceGem, ForcedMovement, Frozen, Health, HitFlash, HitStop, HitStun,
    Knockback, Lifetime, MagicProjectile, Pickup, PickupKind, Player, PlayerClass, PlayerName,
    Position, Projectile, ProjectileOwner, RenderInfo, RespawnTimer, Room, RoomEntity, Stamina,
    Stun, Team, TelegraphIndicator, TrailSegment, Velocity, Weapon, WeaponKind, BuffComponent, DeathAnimation,
};
pub use events::*;
pub use hitbox::*;
pub use items::*;
pub use tween::*;
pub use specs::*;
pub use bundles::*;
pub use plugin::CorePlugin;
pub use resources::{
    AppState, CameraTransform, CharacterCreationState, CursorWorldPos, DungeonInstance,
    DungeonState, GameAssets, GameConfig, MetaProgression, PlayTimer, PlayerInput, PlayerProfile,
    PlayerProfiles, RunProgression, RunState, ScreenShake, UpgradeTier, WaveState,
    DeathPenalty, Graveyard, ItemDatabase, PendingClassSpawn,
};
