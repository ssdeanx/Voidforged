//! World plugin — wires map generation, zone tracking, dungeon entrance detection.

use bevy::prelude::*;
use ir_core::*;
use crate::zone::*;
use crate::map::*;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app
            .init_resource::<CurrentZone>()
            .add_systems(OnEnter(AppState::World), (
                generate_world,
            ))
            .add_systems(Update, (
                track_player_zone,
                detect_dungeon_entry,
            ).run_if(in_state(AppState::World)));
    }
}

/// Detects player standing on a dungeon entrance → transition to Dungeon state.
fn detect_dungeon_entry(
    player_query: Query<&Transform, With<Player>>,
    entrances: Query<(&DungeonEntrance, &Transform), With<EntranceMarker>>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<AppState>>,
    mut dungeon_state: ResMut<DungeonState>,
) {
    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    for (entrance, entrance_tf) in entrances.iter() {
        let dist = player_pos.distance(entrance_tf.translation);
        if dist < 1.5 && keyboard.just_pressed(KeyCode::KeyF) {
            dungeon_state.current = Some(ir_core::DungeonInstance {
                name: entrance.name.clone(),
                tier: entrance.dungeon_tier,
                depth: entrance.depth,
            });
            next_state.set(AppState::Dungeon);
            info!("Entering dungeon: {}", entrance.name);
            return;
        }
    }
}

use ir_core::DungeonState;
