//! Isometric Roguelite — Main client binary.
//!
//! A 3D isometric action roguelite with open world exploration,
//! procedural dungeons, Hades-style combat, and deep meta-progression.

use bevy::prelude::*;

/// Entry point for the Voidforged game client.
///
/// Builds and runs the Bevy application with all game plugins:
///
/// - **Core** — shared types, resources, and events.
/// - **Rendering** — isometric 3D camera, lighting, asset loading, HUD.
/// - **World** — open world map, zones, dungeon entrances.
/// - **Dungeon** — procedural room generation, encounters, boss fights.
/// - **Gameplay** — combat, enemies, projectiles, player control.
/// - **Procedural** — loot tables and procedural content generation.
/// - **Progression** — XP, leveling, meta-progression.
/// - **Save** — persistent player profiles and autosave/autoload.
/// - **Network** — multiplayer connectivity (currently stubbed).
fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        // Core — shared types, resources, events
        .add_plugins(ir_core::CorePlugin)
        // Rendering — isometric 3D camera, lighting, assets, HUD
        .add_plugins(ir_rendering::RenderingPlugin)
        // World — open world map, zones, dungeon entrances
        .add_plugins(ir_world::WorldPlugin)
        // Dungeon — procedural rooms, encounters, boss fights
        .add_plugins(ir_dungeon::DungeonPlugin)
        // Gameplay — combat, enemies, projectiles, player control
        .add_plugins(ir_gameplay::GameplayPlugin)
        // Procedural — loot tables
        .add_plugins(ir_procedural::ProceduralPlugin)
        // Progression — XP, leveling, meta-progression
        .add_plugins(ir_progression::ProgressionPlugin)
        // Save/Load — persistent game state
        .add_plugins(ir_save::SavePlugin)
        // Network — multiplayer (stubbed)
        .add_plugins(ir_network::NetworkPlugin)
        // Run the app
        .run();
}
