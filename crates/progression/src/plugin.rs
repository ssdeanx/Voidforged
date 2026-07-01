//! Plugin for meta-progression — experience gain, level-up, and upgrades.

use bevy::prelude::*;
use ir_core::*;
use crate::{leveling, upgrades};

/// Bevy plugin for meta-progression systems.
///
/// Registers XP handling and level-up systems that run during the
/// `Dungeon` and `Playing` game states. Also adds the upgrades sub-plugin
/// and the Dark Essence reward system for dungeon completion.
pub struct ProgressionPlugin;

fn can_gain_xp(state: Res<State<AppState>>) -> bool {
    matches!(*state.get(), AppState::Dungeon | AppState::Playing)
}

impl Plugin for ProgressionPlugin {
    fn build(&self, app: &mut App) {
        app
            .add_plugins(upgrades::UpgradesPlugin)
            .add_systems(Update, (
                leveling::handle_xp_gain,
                leveling::apply_level_up,
            ).chain().run_if(can_gain_xp))
            // Dark Essence reward on dungeon end
            .add_systems(Update, grant_dark_essence_on_dungeon_end);
    }
}

/// Listens for DungeonEndEvent and awards Dark Essence.
/// Formula: cleared as tier (0/1) * 10 + wave_reached * 5
fn grant_dark_essence_on_dungeon_end(
    mut events: EventReader<DungeonEndEvent>,
    mut meta: ResMut<MetaProgression>,
    mut progression: ResMut<RunProgression>,
) {
    for event in events.read() {
        let tier_reward = if event.cleared { 1 } else { 0 } * 10;
        let depth_reward = event.wave_reached * 5;
        let total = (tier_reward + depth_reward) as u64;
        meta.add_dark_essence(total);
        info!(
            "Dungeon ended: cleared={}, wave={}, Dark Essence earned: {}",
            event.cleared, event.wave_reached, total
        );
        // Track run counts
        meta.total_runs += 1;
        if event.cleared {
            meta.completed_runs += 1;
        }
        if event.wave_reached > meta.highest_wave {
            meta.highest_wave = event.wave_reached;
        }
        // Reset run progression for next run
        *progression = RunProgression::default();
    }
}
