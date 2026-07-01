//! Game state machines — top-level application and run-lifecycle states.

use bevy::prelude::*;

/// Top-level application states.
///
/// Controls the high-level screen flow: loading, menus, gameplay, and pause/game-over screens.
/// Managed by `CorePlugin` via Bevy's `State` system.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    /// Initial loading screen — assets, database, and item definitions are initialized here.
    #[default]
    Loading,
    /// Title / main menu screen.
    MainMenu,
    /// Character selection or creation screen.
    CharacterSelect,
    /// Open-world overworld gameplay mode.
    World,
    /// Inside a dungeon instance.
    Dungeon,
    /// Active gameplay (shared by World and Dungeon during play).
    Playing,
    /// Game is paused (menu overlay shown).
    Paused,
    /// Game over / death recap screen.
    GameOver,
}

/// Run state — lifecycle of a single dungeon run.
///
/// Tracks the current phase of an instanced run, from entering through victory or defeat.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunState {
    /// Run has just started — transition animation / setup.
    #[default]
    Entering,
    /// Player is exploring (out of combat).
    Exploring,
    /// Active combat wave is in progress.
    Combat,
    /// Transitioning between rooms/zones.
    RoomTransition,
    /// Active boss encounter.
    Boss,
    /// Run completed successfully.
    Victory,
    /// Player died — run ended in defeat.
    Defeat,
}
