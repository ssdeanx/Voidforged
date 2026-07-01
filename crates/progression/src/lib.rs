//! Meta-progression — experience, leveling, permanent upgrades, unlocks.

/// Experience and leveling systems — XP gain events, level-up bonuses, stat scaling.
pub mod leveling;
/// Permanent upgrade definitions, purchase logic, and stat bonus application.
pub mod upgrades;
/// Plugin registration for meta-progression systems.
pub mod plugin;

/// Re-export of [`ProgressionPlugin`] for convenience.
pub use plugin::ProgressionPlugin;
