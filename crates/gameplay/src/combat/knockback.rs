//! Knockback system — applies and decays knockback velocity.
use bevy::prelude::*;
use ir_core::*;

pub fn apply_knockback(
    mut commands: Commands, time: Res<Time>,
    mut query: Query<(Entity, &mut Velocity, &mut Knockback)>,
) {
    for (entity, mut velocity, mut knockback) in query.iter_mut() {
        velocity.0 += knockback.velocity * time.delta_secs();
        let damping_factor = (1.0 - knockback.damping * time.delta_secs()).max(0.0);
        knockback.velocity *= damping_factor;
        if knockback.velocity.length_squared() < 0.01 { commands.entity(entity).remove::<Knockback>(); }
    }
}