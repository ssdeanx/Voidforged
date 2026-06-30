use bevy::prelude::*;
use ir_core::*;
use crate::{player, enemy, combat, pickup};

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Player systems
            .add_systems(Update, (
                player::read_player_input,
                player::player_movement,
                player::apply_player_velocity,
                player::player_auto_attack,
            ).run_if(in_state(AppState::Playing)))

            // Enemy systems
            .add_systems(Update, (
                enemy::enemy_ai,
                enemy::apply_enemy_velocity,
            ).run_if(in_state(AppState::Playing)))

            // Combat systems
            .add_systems(Update, (
                combat::move_projectiles,
                combat::projectile_hit,
                combat::apply_damage,
                combat::handle_death,
            ).run_if(in_state(AppState::Playing)))

            // Pickup systems
            .add_systems(Update, pickup::gem_magnet.run_if(in_state(AppState::Playing)));
    }
}
