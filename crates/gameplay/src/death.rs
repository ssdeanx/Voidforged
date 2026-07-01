//! Death & respawn systems — handles PlayerDeathEvent for open-world and dungeon contexts.

use bevy::prelude::*;
use ir_core::*;

/// Listens for PlayerDeathEvent and applies context-appropriate consequences.
pub fn handle_player_death_event(
    mut events: EventReader<PlayerDeathEvent>,
    mut dungeon_end: EventWriter<DungeonEndEvent>,
    mut next_state: ResMut<NextState<AppState>>,
    graveyard: Res<Graveyard>,
    penalty: Res<DeathPenalty>,
    mut progression: ResMut<RunProgression>,
    mut player_query: Query<(&mut Transform, &mut Inventory), With<Player>>,
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
            // Open world: respawn at graveyard, apply gold penalty
            info!("Open world death — respawning at graveyard");
            if let Ok((mut tf, mut inv)) = player_query.get_single_mut() {
                tf.translation = graveyard.position;
                let gold_loss = (inv.gold as f32 * penalty.gold_loss_pct) as u64;
                inv.remove_gold(gold_loss);
                info!("Lost {} gold from death penalty", gold_loss);
            }
            // Heal the player to full on respawn
            if let Ok(mut health) = player_query.get_single_mut() {
                let _ = health; // placeholder — full heal on world state re-enter
            }
            next_state.set(AppState::World);
        }
    }
}
