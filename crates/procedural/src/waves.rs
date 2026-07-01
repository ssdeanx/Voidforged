//! Wave-based enemy spawning system — spawns enemies in waves around the player,
//! with escalating difficulty and enemy variety.

use bevy::prelude::*;
use ir_core::*;
use rand::Rng;

/// Spawns a wave of enemies around the player.
pub fn spawn_wave(
    mut commands: Commands,
    mut wave_state: ResMut<WaveState>,
    player_query: Query<&Transform, With<Player>>,
    time: Res<Time>,
    play_timer: Res<PlayTimer>,
) {
    // 3-second delay before first wave spawns
    if wave_state.wave_number == 1 && wave_state.enemies_spawned == 0 && play_timer.0 < 3.0 {
        return;
    }
    if wave_state.enemies_spawned >= wave_state.enemies_total {
        return;
    }

    wave_state.spawn_timer -= time.delta_secs();
    if wave_state.spawn_timer > 0.0 {
        return;
    }

    let player_pos = match player_query.get_single() {
        Ok(t) => t.translation,
        Err(_) => return,
    };

    let mut rng = rand::thread_rng();

    // Spawn a group of enemies
    let spawn_count = (wave_state.enemies_total - wave_state.enemies_spawned)
        .min(3 + (wave_state.wave_number / 5) as u32);

    for _ in 0..spawn_count {
        // Random spawn position at edge of screen
        let angle = rng.gen::<f32>() * std::f32::consts::TAU;
        let distance = 12.0 + rng.gen::<f32>() * 5.0;
        let spawn_pos = Vec3::new(
            player_pos.x + angle.cos() * distance,
            0.0,
            player_pos.z + angle.sin() * distance,
        );

        let variant = select_enemy_variant(wave_state.wave_number, &mut rng);
        commands.spawn(EnemyBundle::new(variant, wave_state.wave_number, spawn_pos));

        wave_state.enemies_spawned += 1;
    }

    wave_state.spawn_timer = wave_state.spawn_interval;
}

fn select_enemy_variant(wave: u32, rng: &mut impl Rng) -> EnemyVariant {
    let roll = rng.gen::<f32>();
    if wave >= 10 && roll < 0.05 {
        EnemyVariant::Boss
    } else if wave >= 5 && roll < 0.15 {
        EnemyVariant::Elite
    } else if roll < 0.30 {
        EnemyVariant::Ranged
    } else if roll < 0.45 {
        EnemyVariant::Charger
    } else {
        EnemyVariant::Grunt
    }
}

/// Checks if wave is cleared and triggers next wave.
pub fn check_wave_cleared(
    mut wave_state: ResMut<WaveState>,
    enemy_query: Query<(), With<Enemy>>,
    mut wave_cleared_events: EventWriter<WaveClearedEvent>,
    mut wave_start_events: EventWriter<WaveStartEvent>,
    _time: Res<Time>,
) {
    if wave_state.enemies_spawned == 0 {
        return; // Wave hasn't started yet
    }

    let enemies_alive = enemy_query.iter().count() as u32;

    if enemies_alive == 0 && wave_state.enemies_spawned >= wave_state.enemies_total {
        // Wave cleared!
        wave_cleared_events.send(WaveClearedEvent {
            wave_number: wave_state.wave_number,
        });

        // Advance to next wave
        wave_state.wave_number += 1;
        wave_state.enemies_spawned = 0;
        wave_state.enemies_total = (wave_state.wave_number * 5 + 3).min(100);
        wave_state.difficulty_multiplier = 1.0 + (wave_state.wave_number as f32 * 0.1);

        wave_start_events.send(WaveStartEvent {
            wave_number: wave_state.wave_number,
            enemy_count: wave_state.enemies_total,
        });
    }
}
