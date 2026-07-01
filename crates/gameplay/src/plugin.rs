use bevy::prelude::*;
use ir_core::*;
use crate::{combat, collection, enemy, pickup, player};

pub struct GameplayPlugin;

/// Run condition: player can move in World, Dungeon, and Playing.
fn can_move(state: Res<State<AppState>>) -> bool {
    matches!(*state.get(), AppState::World | AppState::Dungeon | AppState::Playing)
}

/// Run condition: combat in all game states (World, Dungeon, Playing).
fn has_combat(state: Res<State<AppState>>) -> bool {
    matches!(*state.get(), AppState::World | AppState::Dungeon | AppState::Playing)
}

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app
            // Apply equipment when player spawns in any game state
            .add_systems(OnEnter(AppState::World), (
                player::apply_equipment,
            ))
            .add_systems(OnEnter(AppState::Dungeon), (
                player::apply_equipment,
            ))
            .add_systems(OnEnter(AppState::Playing), (
                player::apply_equipment,
            ))
            // Player movement systems — run in World, Dungeon, Playing
            .add_systems(Update, (
                player::read_player_input,
                player::player_dash,
                player::player_movement,
                player::apply_player_velocity,
            ).run_if(can_move))

            // Player attack systems
            .add_systems(Update, (
                player::player_attack,
                player::player_secondary_attack,
                player::player_cast,
            ).run_if(has_combat))

            // Enemy systems
            .add_systems(Update, (
                enemy::enemy_ai,
                enemy::apply_enemy_velocity,
                enemy::enemy_melee_attack,
                enemy::enemy_ranged_attack,
            ).run_if(has_combat))

            // Combat systems
            .add_systems(Update, (
                combat::move_projectiles,
                combat::projectile_hit,
                combat::projectile_hit_player,
                combat::apply_damage,
                combat::handle_death,
            ).chain().run_if(has_combat))

            // Pickup systems
            .add_systems(Update, (
                pickup::gem_magnet,
                pickup::collect_health_pickups,
                pickup::collect_gold_pickups,
                collection::collect_gems,
            ).run_if(has_combat));
    }
}
