//! Applies velocity to enemy positions.
use bevy::prelude::*;
use ir_core::*;

pub fn apply_enemy_velocity(time: Res<Time>, mut query: Query<(&mut Transform, &Velocity), With<Enemy>>) {
    for (mut transform, velocity) in query.iter_mut() { transform.translation += velocity.0 * time.delta_secs(); }
}