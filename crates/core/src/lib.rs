//! Core shared types, components, resources, and events for the isometric roguelite.
//! This crate is the foundation — every other crate depends on it.

pub mod components;
pub mod resources;
pub mod events;
pub mod bundles;
pub mod plugin;

pub use components::*;
pub use resources::*;
pub use events::*;
pub use bundles::*;
pub use plugin::CorePlugin;
