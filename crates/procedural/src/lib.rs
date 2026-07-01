//! Procedural generation — maps, enemy waves, loot tables, and difficulty scaling.

/// Wave-based enemy spawning systems — spawn timing, difficulty scaling, variant selection.
pub mod waves;
/// Loot drop system — XP gems, health pickups, and gold based on enemy variant and wave tier.
pub mod loot;
/// Plugin registration for procedural generation systems.
pub mod plugin;

/// Re-export of [`ProceduralPlugin`] for convenience.
pub use plugin::ProceduralPlugin;
