//! Meta-progression — persistent upgrades that carry across runs.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Persistent meta-progression data saved between runs.
///
/// Tracks accumulated currency (dark essence, gold), run statistics,
/// unlocked items/classes, and purchased permanent upgrades.
/// Loaded on startup and saved periodically via [`SaveDatabase`](crate::db::SaveDatabase).
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgression {
    /// Premium currency earned from achievements and challenges.
    pub dark_essence: u64,
    /// Regular gold currency.
    pub gold: u64,
    /// Total number of runs ever attempted.
    pub total_runs: u32,
    /// Number of runs completed (victory).
    pub completed_runs: u32,
    /// Highest wave number ever reached across all runs.
    pub highest_wave: u32,
    /// Unlocked item/ability identifiers (e.g. "dagger", "fireball").
    pub unlocks: Vec<String>,
    /// Purchased permanent upgrade tiers.
    pub upgrades: Vec<UpgradeTier>,
}

impl Default for MetaProgression {
    fn default() -> Self {
        Self {
            dark_essence: 0,
            gold: 0,
            total_runs: 0,
            completed_runs: 0,
            highest_wave: 0,
            unlocks: vec!["dagger".to_string()],
            upgrades: Vec::new(),
        }
    }
}

/// A single purchased tier of a permanent upgrade.
///
/// These are applied at the start of every run and persist in save data.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeTier {
    /// Unique identifier of the upgrade (e.g. "move_speed", "max_hp").
    pub id: String,
    /// Purchased tier level (starts at 1, each tier costs more).
    pub tier: u32,
    /// Currency cost paid for this tier.
    pub cost: u64,
}
