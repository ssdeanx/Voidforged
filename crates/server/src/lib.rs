//! Dedicated server binary — headless game simulation for multiplayer.

/// Headless server app builder.
pub mod server_app;
/// Server-specific Bevy plugin, resources, and systems.
pub mod plugin;

pub use plugin::ServerPlugin;
