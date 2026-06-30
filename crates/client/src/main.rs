//! Isometric Roguelite — Main client binary.
//!
//! A 3D isometric action roguelite combining Vampire Survivors auto-combat
//! with Hades-style depth and MMO-capable architecture.

use bevy::prelude::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Core — shared types, resources, events
        .add_plugins(ir_core::CorePlugin)
        // Rendering — isometric 3D camera, lighting, assets
        .add_plugins(ir_rendering::RenderingPlugin)
        // Gameplay — combat, enemies, projectiles, player control
        .add_plugins(ir_gameplay::GameplayPlugin)
        // Procedural — wave spawning, map gen, loot
        .add_plugins(ir_procedural::ProceduralPlugin)
        // Progression — XP, leveling, meta-progression
        .add_plugins(ir_progression::ProgressionPlugin)
        // Network — multiplayer (stubbed)
        .add_plugins(ir_network::NetworkPlugin)
        // Run the app
        .run();
}
