//! Saved character profiles and character creation UI state.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use crate::components::CharacterClass;

/// One saved character profile.
///
/// Contains persistable data for a single character: identity, class, level,
/// inventory snapshot, and accumulated wealth. Serialized to/from the
/// [`SaveDatabase`](crate::db::SaveDatabase) via Bincode.
#[derive(Debug, Clone, Component, Serialize, Deserialize)]
pub struct PlayerProfile {
    /// Unique numeric identifier for this profile.
    pub id: u32,
    /// Player-chosen character name.
    pub name: String,
    /// The five playable classes.
    pub class: CharacterClass,
    /// Current character level (persists across runs).
    pub level: u32,
    /// Experience points accumulated.
    pub xp: u64,
    /// Gold currency held.
    pub gold: u64,
    /// Names of dungeons this character has completed.
    pub completed_dungeons: Vec<String>,
    /// Total play time on this character in seconds.
    pub play_time: f32,
}

/// All saved character profiles, loaded from disk on startup.
///
/// `next_id` is an auto-increment counter used when creating new profiles.
#[derive(Resource, Debug, Clone, Serialize, Deserialize, Default)]
pub struct PlayerProfiles {
    /// All known profiles.
    pub profiles: Vec<PlayerProfile>,
    /// Next auto-increment ID for new profiles.
    pub next_id: u32,
}

/// UI state for the character creation and selection screen.
///
/// Populated by the character select UI and consumed when spawning
/// the player entity into the game world.
#[derive(Resource)]
pub struct CharacterCreationState {
    /// Currently selected character class (None = not yet chosen).
    pub selected_class: Option<CharacterClass>,
    /// Player-entered name for the new character.
    pub player_name: String,
    /// If editing an existing character, their profile ID.
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
