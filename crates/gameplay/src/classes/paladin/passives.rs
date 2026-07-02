//! Paladin passives — Holy Power generation.

use bevy::prelude::*;
use ir_core::*;

pub fn paladin_holy_power_on_hit(
    mut damage_events: EventReader<DamageEvent>,
    mut player_query: Query<&mut ClassResource, (With<Player>, With<PlayerClass>)>,
) {
    let Ok(mut resource) = player_query.get_single_mut() else { return };
    if (resource.max - 5.0).abs() > 0.1 || (resource.regen_rate - 0.5).abs() > 0.1 { return; }
    for event in damage_events.read() {
        if event.damage_type == DamageType::Physical || event.damage_type == DamageType::Magic {
            resource.current = (resource.current + 1.0).min(resource.max);
        }
    }
}
