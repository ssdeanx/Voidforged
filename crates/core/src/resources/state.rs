//! Game state machines.

use bevy::prelude::*;

/// Top-level application states.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum AppState {
    #[default]
    Loading,
    MainMenu,
    CharacterSelect,
    World,
    Dungeon,
    Playing,
    Paused,
    GameOver,
}

/// Run state — lifecycle of a single run.
#[derive(States, Default, Debug, Clone, PartialEq, Eq, Hash)]
pub enum RunState {
    #[default]
    Entering,
    Exploring,
    Combat,
    RoomTransition,
    Boss,
    Victory,
    Defeat,
}
