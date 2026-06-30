//! Network crate — multiplayer protocol, client/server communication.
//! Currently stubbed for future implementation with bevy_replicon or lightyear.

pub mod protocol;
pub mod client;
pub mod server;
pub mod plugin;

pub use plugin::NetworkPlugin;
