//! 3D isometric rendering — camera, lighting, asset loading, and visual effects.
//!
//! This crate manages all rendering-related systems: the isometric camera setup
//! and following, lighting, asset placeholder generation, HUD/UI, particle effects,
//! screen shake, damage numbers, and the GLTF asset pipeline.

pub mod audio;
/// Module for isometric camera setup, following, and screen shake.
pub mod camera;
/// Module for 3D scene lighting (directional + ambient).
pub mod lighting;
/// Module for procedural mesh generation (bushes, trees, rocks, etc.).
pub mod proc_meshes;
/// Module for placeholder sprite asset generation.
pub mod assets;
/// Module for spawning game world entities and cleanup.
pub mod spawn;
/// Module for all HUD/UI screens and overlays.
pub mod hud;
/// Module for particle effects, glow materials, and VFX.
pub mod effects;
/// Module for GLTF model loading pipeline.
pub mod asset_pipeline;
pub mod ui_textures;
/// Module for UI icon asset loading.
pub mod ui_icons;
/// Plugin registration — wires all rendering systems into the Bevy app.
pub mod plugin;

/// The top-level plugin for all rendering systems.
pub use plugin::RenderingPlugin;
pub use ui_textures::UiTextureAssets;
pub use ui_icons::UiIconAssets;
pub use assets::{GameSpriteAssets, load_game_sprites};
