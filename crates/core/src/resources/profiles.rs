//! Saved character profiles and character creation state.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::CharacterClass;

/// One saved character profile.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct PlayerProfile {
    pub id: u32,
    pub name: String,
    pub class: CharacterClass,
    pub level: u32,
    pub xp: u64,
    pub gold: u64,
    pub completed_dungeons: Vec<String>,
    pub play_time: f32,
}

/// All saved character profiles (loaded from disk).
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerProfiles {
    pub profiles: Vec<PlayerProfile>,
    pub next_id: u32,
}

/// UI state for the character creation / selection screen.
#[derive(Resource)]
pub struct CharacterCreationState {
    pub selected_class: Option<CharacterClass>,
    pub player_name: String,
    pub editing_existing: Option<u32>,
}

impl Default for CharacterCreationState {
    fn default() -> Self {
        Self {
            selected_class: Some(CharacterClass::Warrior),
            player_name: String::new(),
            editing_existing: None,
        }
    }
}
