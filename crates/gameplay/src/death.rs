//! Death & respawn systems — handles PlayerDeathEvent for open-world and dungeon contexts.
//! Includes respawn timer with 3-second delay before respawn.

use bevy::prelude::*;
use ir_core::*;

/// Listens for PlayerDeathEvent and applies context-appropriate consequences.
/// Fixes the double-mut-borrow issue by querying separately.
pub fn handle_player_death_event(
    mut commands: Commands,
    time: Res<Time>,
    mut events: EventReader<PlayerDeathEvent>,
    mut dungeon_end: EventWriter<DungeonEndEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    graveyard: Res<Graveyard>,
    penalty: Res<DeathPenalty>,
    mut progression: ResMut<RunProgression>,
    player_query: Query<(Entity, &Transform, &Player), With<Player>>,
    mut health_query: Query<&mut Health, With<Player>>,
    mut inventory_query: Query<&mut Inventory, With<Player>>,
    mut player_query_mut: Query<&mut Transform, With<Player>>,
) {
    for event in events.read() {
        if event.in_dungeon {
            info!(
                "Dungeon death — run over. Score: {} kills",
                progression.kills
            );
            dungeon_end.send(DungeonEndEvent {
                cleared: false,
                kills: progression.kills,
                wave_reached: 0,
                gold_collected: progression.gold_collected,
                xp_earned: progression.xp_earned,
                run_time: progression.run_time,
            });
            next_state.set(AppState::GameOver);
        } else {
            // Open world: respawn at graveyard with penalties
            info!("Open world death — respawning at graveyard");

            // Apply XP loss penalty
            let xp_loss = (progression.xp_earned as f32 * penalty.xp_loss_pct) as u64;
            progression.xp_earned = progression.xp_earned.saturating_sub(xp_loss);
            info!("Lost {} XP from death penalty", xp_loss);

            // Apply gold loss (separate query to avoid double-mut-borrow)
            let gold_loss = match inventory_query.get_single_mut() {
                Ok(inv) => {
                    (inv.gold as f32 * penalty.gold_loss_pct) as u64
                }
                Err(_) => 0,
            };
            if gold_loss > 0 {
                if let Ok(mut inv) = inventory_query.get_single_mut() {
                    inv.remove_gold(gold_loss);
                    info!("Lost {} gold from death penalty", gold_loss);
                }
            }

            // Teleport player to graveyard (separate query)
            if let Ok(mut tf) = player_query_mut.get_single_mut() {
                tf.translation = graveyard.position;
            }

            // Full heal on respawn
            if let Ok(mut health) = health_query.get_single_mut() {
                health.current = health.max;
                health.invulnerable_until = 0.0;
            }

            // Add 3-second invulnerability buffer after respawn
            if let Ok(mut health) = health_query.get_single_mut() {
                health.invulnerable_until = time.elapsed_secs() as f32 + 3.0;
            }

            // Add respawn timer for visual feedback
            if let Ok((entity, _transform, _player)) = player_query.get_single() {
                commands.entity(entity).insert(RespawnTimer::new(3.0));
            }

            next_state.set(AppState::World);
        }
    }
}
