//! Gameplay logic — combat, enemies, projectiles, player control, and pickups.

pub mod player;
pub mod enemy;
pub mod combat;
pub mod pickup;
pub mod collection;
pub mod classes;
pub mod equipment;
pub mod loot;
pub mod death;
pub mod plugin;

pub use plugin::GameplayPlugin;
