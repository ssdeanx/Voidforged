//! Open world map system — zones, terrain generation, zone transitions.

/// Zone definitions (Grasslands, Desert, Forest) and world-position tracking.
pub mod zone;
/// Tile grid generation, environment decorations, dungeon entrance markers.
pub mod map;
/// Plugin registration for world map systems.
pub mod plugin;

/// Re-export of [`WorldPlugin`] for convenience.
pub use plugin::WorldPlugin;
/// Re-export of all public items from the [`zone`] module.
pub use zone::*;
/// Re-export of all public items from the [`map`] module.
pub use map::*;
