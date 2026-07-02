//! Dungeon crate — procedural room generation, encounters, boss fights.

/// Procedural dungeon room generation — 3×3 room grid with corridors, walls, and enemy placement.
pub mod rooms;
pub mod raid;
/// Plugin registration for dungeon systems.
pub mod plugin;

/// Re-export of [`DungeonPlugin`] for convenience.
pub use plugin::DungeonPlugin;
