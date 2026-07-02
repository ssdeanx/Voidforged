//! Stamina system — Sprinting, stamina regen, death animation.
use bevy::prelude::*;
use ir_core::*;

#[derive(Component, Default)]
pub struct Sprinting(pub bool);

pub fn death_animation_system(
    mut commands: Commands, time: Res<Time>,
    mut query: Query<(Entity, &mut DeathAnimation, &mut Transform)>,
) {
    let dt = time.delta_secs();
    for (entity, mut death, mut transform) in query.iter_mut() {
        death.timer -= dt;
        let progress = 1.0 - (death.timer / 0.5).max(0.0);
        let eased = 1.0 - (1.0 - progress) * (1.0 - progress);
        let scale = death.initial_scale * (1.0 - eased);
        transform.scale = Vec3::splat(scale.max(0.0));
        transform.translation.y += dt * 0.5 * (1.0 - progress);
        if death.timer <= 0.0 { commands.entity(entity).despawn(); }
    }
}

pub fn stamina_regen(time: Res<Time>, mut query: Query<&mut Stamina, With<Player>>) {
    for mut stamina in query.iter_mut() {
        if stamina.stamina_lockout_timer > 0.0 {
            stamina.stamina_lockout_timer = (stamina.stamina_lockout_timer - time.delta_secs()).max(0.0);
        }
        if stamina.stamina_lockout_timer <= 0.0 {
            stamina.current = (stamina.current + stamina.regen_rate * time.delta_secs()).min(stamina.max);
        }
    }
}