//! Gameplay logic — combat, enemies, projectiles, player control, and pickups.

pub mod player;
pub mod enemy;
pub mod combat;
pub mod projectile;
pub mod pickup;
pub mod plugin;

pub use plugin::GameplayPlugin;
