//! Save/load system for persistent game state.
//! Supports multiple characters + global meta-progression.
//! Save files in ~/.voidforged/

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use ir_core::*;

/// Serializable player data per character.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSaveData {
    pub level: u32,
    pub xp: u64,
    pub gold: u64,
    pub completed_dungeons: Vec<String>,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self { level: 1, xp: 0, gold: 0, completed_dungeons: vec![] }
    }
}

/// Full game save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    pub version: u32,
    pub profiles: Vec<PlayerProfile>,
    pub next_profile_id: u32,
    pub meta: MetaProgression,
}

impl Default for SaveData {
    fn default() -> Self {
        Self {
            version: 2,
            profiles: vec![],
            next_profile_id: 1,
            meta: MetaProgression::default(),
        }
    }
}

/// Resource holding loaded save state.
#[derive(Resource, Default)]
pub struct SaveState {
    pub data: Option<SaveData>,
}

/// Resource to trigger an autosave next frame.
#[derive(Resource, Default)]
pub struct PendingSave(pub bool);

const SAVE_DIR: &str = ".voidforged";
const SAVE_FILE: &str = "save.dat";

fn save_path() -> Result<std::path::PathBuf, String> {
    let home = std::env::var("HOME").map_err(|_| "HOME not set".to_string())?;
    let dir = std::path::PathBuf::from(&home).join(SAVE_DIR);
    std::fs::create_dir_all(&dir).map_err(|e| format!("Failed to create save dir: {e}"))?;
    Ok(dir.join(SAVE_FILE))
}

/// Write save data to disk.
pub fn save(data: &SaveData) -> bool {
    match save_path() {
        Ok(path) => match bincode::serialize(data) {
            Ok(encoded) => {
                if let Err(e) = std::fs::write(&path, encoded) {
                    error!("Failed to write save file: {e}");
                    return false;
                }
                info!("Game saved to {:?}", path);
                true
            }
            Err(e) => {
                error!("Failed to serialize save data: {e}");
                false
            }
        },
        Err(e) => {
            error!("{e}");
            false
        }
    }
}

/// Read save data from disk.
pub fn load() -> Option<SaveData> {
    match save_path() {
        Ok(path) => {
            if !path.exists() {
                return None;
            }
            match std::fs::read(&path) {
                Ok(encoded) => match bincode::deserialize(&encoded) {
                    Ok(data) => Some(data),
                    Err(e) => {
                        error!("Failed to deserialize save: {e}");
                        None
                    }
                },
                Err(e) => {
                    error!("Failed to read save file: {e}");
                    None
                }
            }
        }
        Err(e) => {
            error!("{e}");
            None
        }
    }
}

/// Builds a full SaveData from currently loaded resources.
fn build_save_data(profiles: &PlayerProfiles, meta: &MetaProgression) -> SaveData {
    SaveData {
        version: 2,
        profiles: profiles.profiles.clone(),
        next_profile_id: profiles.next_id,
        meta: meta.clone(),
    }
}

/// Autosave: writes profiles + meta to disk.
fn autosave(
    state: Res<State<AppState>>,
    profiles: Res<PlayerProfiles>,
    meta: Res<MetaProgression>,
    mut pending: ResMut<PendingSave>,
) {
    if matches!(*state.get(), AppState::GameOver | AppState::MainMenu) {
        if pending.0 {
            let data = build_save_data(&profiles, &meta);
            save(&data);
            pending.0 = false;
        }
    }
}

/// Autoload on entering MainMenu.
fn autoload(
    mut save_state: ResMut<SaveState>,
    mut profiles: ResMut<PlayerProfiles>,
    mut meta: ResMut<MetaProgression>,
    state: Res<State<AppState>>,
) {
    if *state.get() == AppState::MainMenu && save_state.data.is_none() {
        save_state.data = Some(load().unwrap_or_default());
        if let Some(ref data) = save_state.data {
            profiles.profiles = data.profiles.clone();
            profiles.next_id = data.next_profile_id;
            *meta = data.meta.clone();
            if !data.profiles.is_empty() {
                info!("Loaded {} character(s)", data.profiles.len());
            }
        }
    }
}

/// Mark autosave as pending (called on death or dungeon clear).
fn mark_pending(mut pending: ResMut<PendingSave>) {
    pending.0 = true;
}

pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<SaveState>()
            .init_resource::<PendingSave>()
            .add_systems(OnEnter(AppState::GameOver), mark_pending)
            .add_systems(Update, (
                autosave,
                autoload,
            ));
    }
}
