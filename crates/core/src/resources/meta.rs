//! Meta-progression — persistent upgrades across runs.

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

/// Persistent meta-progression data saved between runs.
#[derive(Resource, Debug, Clone, Serialize, Deserialize)]
pub struct MetaProgression {
    pub dark_essence: u64,
    pub gold: u64,
    pub total_runs: u32,
    pub completed_runs: u32,
    pub highest_wave: u32,
    pub unlocks: Vec<String>,
    pub upgrades: Vec<UpgradeTier>,
}

impl MetaProgression {
    /// Add Dark Essence — the primary meta-progression currency earned from dungeon runs.
    pub fn add_dark_essence(&mut self, amount: u64) {
        self.dark_essence = self.dark_essence.saturating_add(amount);
    }
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

/// A purchased tier of a permanent upgrade.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UpgradeTier {
    pub id: String,
    pub tier: u32,
    pub cost: u64,
}
