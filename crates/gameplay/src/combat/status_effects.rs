//! Status effect timers — Frozen, Stun, HitStun, HitStop, HitFlash.
use bevy::prelude::*;
use ir_core::*;

pub fn tick_frozen(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Frozen)>) {
    for (entity, mut frozen) in query.iter_mut() {
        frozen.remaining -= time.delta_secs();
        if frozen.remaining <= 0.0 { commands.entity(entity).remove::<Frozen>(); }
    }
}

pub fn tick_stun(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut Stun)>) {
    for (entity, mut stun) in query.iter_mut() {
        stun.remaining -= time.delta_secs();
        if stun.remaining <= 0.0 { commands.entity(entity).remove::<Stun>(); }
    }
}

pub fn tick_hit_stun(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut HitStun)>) {
    for (entity, mut hit_stun) in query.iter_mut() {
        hit_stun.remaining -= time.delta_secs();
        if hit_stun.remaining <= 0.0 { commands.entity(entity).remove::<HitStun>(); }
    }
}

pub fn tick_hit_stop(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut HitStop)>) {
    for (entity, mut hit_stop) in query.iter_mut() {
        hit_stop.remaining -= time.delta_secs();
        if hit_stop.remaining <= 0.0 { commands.entity(entity).remove::<HitStop>(); }
    }
}

pub fn apply_stun_movement_block(mut query: Query<&mut Velocity, (With<Stun>, Without<Player>)>) {
    for mut velocity in query.iter_mut() { velocity.0 = Vec3::ZERO; }
}

pub fn tick_hit_flash(mut commands: Commands, time: Res<Time>, mut query: Query<(Entity, &mut HitFlash)>) {
    for (entity, mut flash) in query.iter_mut() {
        flash.remaining -= time.delta_secs();
        if flash.remaining <= 0.0 { commands.entity(entity).remove::<HitFlash>(); }
    }
}