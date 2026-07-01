//! Save/load system for persistent game state.
//!
//! Provides bincode-based serialization for player profiles and meta-progression.
//! Save files are stored at `~/.voidforged/save.dat`. The system supports
//! multiple character profiles, autosave on game-over/dungeon-clear, and
//! automatic loading when returning to the main menu.
//!
//! # Resources
//!
//! - [`SaveState`] — holds the loaded [`SaveData`] in memory.
//! - [`PendingSave`] — flag to trigger an autosave on the next frame.
//!
//! # Systems
//!
//! - `autosave` — writes profiles + meta to disk at appropriate transitions.
//! - `autoload` — reads the save file from disk when entering `MainMenu`.
//! - `mark_pending` — sets the autosave pending flag on death or dungeon clear.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};
use ir_core::*;

/// Serializable player data for a single character.
///
/// Each entry in the saved profiles vector corresponds to one character
/// the player has created.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct PlayerSaveData {
    /// Current character level.
    pub level: u32,
    /// Experience points accumulated toward the next level.
    pub xp: u64,
    /// Gold currency carried by this character.
    pub gold: u64,
    /// Names of dungeons this character has completed.
    pub completed_dungeons: Vec<String>,
}

impl Default for PlayerSaveData {
    fn default() -> Self {
        Self { level: 1, xp: 0, gold: 0, completed_dungeons: vec![] }
    }
}

/// Full game save data encompassing all profiles and global meta-progression.
///
/// This is the top-level structure serialized to disk by [`save`] and
/// deserialized by [`load`].
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SaveData {
    /// Schema version for forward/backward compatibility.
    pub version: u32,
    /// All saved player character profiles.
    pub profiles: Vec<PlayerProfile>,
    /// Auto-incrementing counter for assigning new profile ids.
    pub next_profile_id: u32,
    /// Global meta-progression state (unlocks, currencies, etc.).
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

/// Bevy resource that holds the currently loaded save data in memory.
///
/// Populated by the `autoload` system on entering `MainMenu` and consumed
/// by gameplay systems that need profile or meta-progression information.
#[derive(Resource, Default)]
pub struct SaveState {
    /// The deserialized save data, or `None` if no save file exists yet.
    pub data: Option<SaveData>,
}

/// Bevy resource flag that requests an autosave on the next frame.
///
/// Set to `true` by the `mark_pending` system (triggered on player death
/// or dungeon completion). The `autosave` system consumes the flag and
/// resets it to `false` after writing to disk.
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

/// Serialize and write the full [`SaveData`] to disk.
///
/// The save file is placed at `~/.voidforged/save.dat`. Returns `true` on
/// success, `false` if serialization or disk I/O failed (errors are logged).
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

/// Read and deserialize the [`SaveData`] from disk.
///
/// Returns `Some(SaveData)` if the file exists and deserialization succeeds,
/// or `None` if the file does not exist, deserialization fails, or the
/// home directory cannot be determined.
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

/// Build a full [`SaveData`] from the currently loaded Bevy resources.
fn build_save_data(profiles: &PlayerProfiles, meta: &MetaProgression) -> SaveData {
    SaveData {
        version: 2,
        profiles: profiles.profiles.clone(),
        next_profile_id: profiles.next_id,
        meta: meta.clone(),
    }
}

/// Autosave system: writes player profiles and meta-progression to disk.
///
/// Runs every frame in the `Update` schedule but only performs the write
/// when the game is in `GameOver` or `MainMenu` state **and** the
/// [`PendingSave`] flag is set.
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

/// Autoload system: reads the save file from disk when entering `MainMenu`.
///
/// Runs every frame but only triggers once (guarded by
/// [`SaveState::data`] being `None`). Populates the [`SaveState`],
/// [`PlayerProfiles`], and [`MetaProgression`] resources from the saved data.
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

/// System that marks a save as pending (triggered on death or dungeon clear).
///
/// Sets [`PendingSave`] to `true` so the next `autosave` run will persist
/// the current game state to disk.
fn mark_pending(mut pending: ResMut<PendingSave>) {
    pending.0 = true;
}

/// Bevy plugin for the save/load system.
///
/// Registers [`SaveState`] and [`PendingSave`] resources, and adds the
/// `autosave`, `autoload`, and `mark_pending` systems:
///
/// - `mark_pending` runs on `OnEnter(AppState::GameOver)`.
/// - `autosave` and `autoload` run every frame in `Update`.
pub struct SavePlugin;

impl Plugin for SavePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SaveState>()
            .init_resource::<PendingSave>()
            .add_systems(OnEnter(AppState::GameOver), mark_pending)
            .add_systems(Update, (autosave, autoload));
    }
}
