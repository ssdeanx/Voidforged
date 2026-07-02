//! Warrior passive systems — Rage generation on dealing/taking damage.

use bevy::prelude::*;
use ir_core::*;

/// Generates rage when the warrior deals damage.
pub fn warrior_rage_on_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut player_query: Query<&mut ClassResource, (With<Player>, With<PlayerClass>)>,
) {
    let Ok(mut resource) = player_query.get_single_mut() else { return };
    if (resource.max - 100.0).abs() > 0.1 || (resource.regen_rate - 2.0).abs() > 0.1 { return; }
    for event in damage_events.read() {
        if event.damage_type == DamageType::Physical || event.damage_type == DamageType::True {
            resource.current = (resource.current + 5.0).min(resource.max);
        }
    }
}

/// Generates rage when the warrior takes damage.
pub fn warrior_rage_on_take_damage(
    mut damage_events: EventReader<DamageEvent>,
    mut player_query: Query<(Entity, &mut ClassResource), (With<Player>, With<PlayerClass>)>,
) {
    let Ok((player_entity, mut resource)) = player_query.get_single_mut() else { return };
    if (resource.max - 100.0).abs() > 0.1 || (resource.regen_rate - 2.0).abs() > 0.1 { return; }
    for event in damage_events.read() {
        if event.target == player_entity {
            resource.current = (resource.current + 5.0).min(resource.max);
        }
    }
}
