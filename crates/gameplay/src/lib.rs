//! Gameplay logic — combat, enemies, projectiles, player control, and pickups.
//!
//! This crate implements all game-facing systems that run during gameplay:
//! player input and movement, enemy AI and attacks, projectile physics,
//! damage pipelines, death and respawn, pickup collection, loot tables,
//! class-based abilities, and equipment management.

/// Player input, movement, attack, dash, and cast systems.
pub mod player;
/// Enemy AI — movement, formation awareness, melee/ranged attacks, boss phases.
pub mod enemy;
/// Combat — projectile movement, hitbox processing, damage pipeline, knockback, status effects.
pub mod combat;
/// Pickup collection — XP gem magnet, health/gold pickups, item interaction.
pub mod pickup;
/// XP gem collection — detects gems reaching player and awards XP.
pub mod collection;
/// Class-based abilities — per-class modules with ability dispatch.
pub mod classes;
/// Equipment system — equip/unequip events, stat recalc.
pub mod equipment;
/// Loot tables — weighted drop system connecting enemy kills to item drops.
pub mod loot;
/// Enemy special abilities — per-variant attack patterns and cooldowns.
pub mod enemy_abilities;
/// Death & respawn — handles player death with context-appropriate consequences.
pub mod death;
/// Plugin registration — wires all gameplay systems into the Bevy app.
pub mod plugin;

/// The top-level plugin for all gameplay systems.
pub use plugin::GameplayPlugin;
