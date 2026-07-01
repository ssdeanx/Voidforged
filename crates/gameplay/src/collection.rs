use bevy::prelude::*;
use ir_core::*;

/// Detects when XP gems reach the player and awards the XP.
pub fn collect_gems(
    mut commands: Commands,
    player_query: Query<(Entity, &Transform), With<Player>>,
    gem_query: Query<(Entity, &ExperienceGem, &Transform), Without<Player>>,
    mut xp_events: EventWriter<ExperienceGainEvent>,
    mut pickup_events: EventWriter<PickupEvent>,
) {
    let (player_entity, player_pos) = match player_query.get_single() {
        Ok(p) => (p.0, p.1.translation),
        Err(_) => return,
    };
    for (gem_entity, gem, transform) in gem_query.iter() {
        if transform.translation.distance(player_pos) < 1.5 {
            xp_events.send(ExperienceGainEvent {
                amount: gem.value,
                source: gem_entity,
            });
            pickup_events.send(PickupEvent {
                player: player_entity,
                item: gem_entity,
                kind: PickupKind::TemporaryBoost,
            });
            commands.entity(gem_entity).despawn();
        }
    }
}
